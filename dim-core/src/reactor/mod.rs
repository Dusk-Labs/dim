pub mod handler;
mod types;

use async_trait::async_trait;

use libsqlite3_sys::sqlite3;
use rusqlite::hooks::Action;
use rusqlite::Connection;
use sqlx::SqliteConnection;

use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;
use std::panic::catch_unwind;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::OwnedMutexGuard;

use tracing::error;
use tracing::instrument;
use tracing::warn;

use types::Event;
use types::Table;

const MAX_COMMIT_DURATION: Duration = Duration::from_millis(1);

#[async_trait]
pub trait Reactor {
    type Error: ::std::error::Error;

    async fn react(&mut self, event: Event) -> Result<(), Self::Error>;
}

#[derive(Clone)]
struct Context {
    uncommited_buffer: Arc<Mutex<Vec<Event>>>,
    tx: UnboundedSender<Event>,
}

impl Context {
    pub fn new(tx: UnboundedSender<Event>) -> Self {
        Self {
            tx,
            uncommited_buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub struct ReactorCore {
    hook_ctx: Context,
    recvr: UnboundedReceiver<Event>,
}

impl ReactorCore {
    pub fn new() -> Self {
        let (tx, rx) = unbounded_channel();

        Self {
            hook_ctx: Context::new(tx),
            recvr: rx,
        }
    }

    pub async fn register(&mut self, lock: &mut OwnedMutexGuard<SqliteConnection>) {
        let mut handle_lock = lock.lock_handle().await.unwrap();
        let handle = handle_lock.as_raw_handle();

        Self::wal_commit_hook_raw(handle.as_ptr(), self.wal_hook());

        // SAFETY: Its safe to construct the connection from a handle because we lock it above
        // which ensures the access is synchronized.
        let conn = unsafe { Connection::from_handle(handle.as_ptr()).unwrap() };

        conn.update_hook(Some(self.update_hook()));
        conn.rollback_hook(Some(self.rollback_hook()));

        // SAFETY: We need to forget `conn` otherwise its destructor runs and our hooks get
        // deleted.
        core::mem::forget(conn);
    }

    pub async fn react<E: ::std::error::Error>(mut self, mut reactor: impl Reactor<Error = E>) {
        while let Some(event) = self.recvr.recv().await {
            if let Err(e) = reactor.react(event).await {
                error!(error = ?e, ?event, "Reactor returned an when processing event.");
            }
        }
    }

    fn update_hook(&self) -> impl FnMut(Action, &str, &str, i64) + Send + 'static {
        let context = self.hook_ctx.clone();

        // NOTE: This callback must complete as quickly as it can as it is called directly after
        // our database operations and not asynchronously. Sqlite will wait for this function to
        // complete executing and then the database operation will also be completed. If this
        // function never ends, the clients operating on the database will lock as well.
        move |action, _, table, rowid| {
            let Ok(source_table): Result<Table, _> = table.try_into() else {
                // Early return because the source table is not something we are watching.
                return;
            };

            // Ignore any events which are invalid
            if matches!(action, Action::UNKNOWN) {
                return;
            }

            let mut buffer = context.uncommited_buffer.lock().unwrap();

            buffer.push(Event {
                id: rowid,
                table: source_table,
                event_type: action.into(),
            });
        }
    }

    fn rollback_hook(&self) -> impl FnMut() + Send + 'static {
        let context = self.hook_ctx.clone();

        // NOTE: If we get a rollback, we automatically assume that our current buffer of
        // uncommited events is dirty and will not be applied to the real database, thus we can
        // just clear them.
        move || {
            let mut buffer = context.uncommited_buffer.lock().unwrap();
            buffer.clear();
        }
    }

    #[instrument(skip_all)]
    fn wal_hook(&self) -> impl FnMut() + Send + 'static {
        let context = self.hook_ctx.clone();

        // NOTE: If we get a commit we can assume our buffer of events is valid and can be reacted
        // on because they are going to be written to the disk.
        move || {
            let now = Instant::now();
            let mut buffer = context.uncommited_buffer.lock().unwrap();
            let buffer_len = buffer.len();

            while let Some(event) = buffer.pop() {
                if let Err(_) = context.tx.send(event) {
                    error!("Failed to send database event.");
                }
            }

            let elapsed = now.elapsed();
            if elapsed > MAX_COMMIT_DURATION {
                warn!(
                    elapsed_ms = elapsed.as_millis(),
                    buffer_len, "Commit hook took too long"
                );
            }
        }
    }

    fn wal_commit_hook_raw<F: FnMut() + Send + 'static>(sqlite: *mut sqlite3, hook: F) {
        // Unfortunately libsqlite3-sys doesnt expose this.
        extern "C" {
            pub fn sqlite3_wal_hook(
                arg1: *mut sqlite3,
                arg2: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: *mut sqlite3,
                        arg3: *const c_char,
                        arg4: c_int,
                    ) -> c_int,
                >,
                arg3: *mut c_void,
            ) -> *mut c_void;
        }

        unsafe extern "C" fn call_boxed_closure<F: FnMut()>(
            p_arg: *mut c_void,
            _: *mut sqlite3,
            _: *const c_char,
            _: c_int,
        ) -> c_int {
            let r = catch_unwind(|| {
                let boxed_hook: *mut F = p_arg.cast::<F>();
                (*boxed_hook)()
            });

            if let Ok(()) = r {
                0
            } else {
                1
            }
        }

        let boxed_hook: *mut F = Box::into_raw(Box::new(hook));
        unsafe {
            sqlite3_wal_hook(sqlite, Some(call_boxed_closure::<F>), boxed_hook.cast());
        }
    }
}
