use tokio::sync::OwnedMutexGuard;
use sqlx::SqliteConnection;
use rusqlite::Connection;
use rusqlite::hooks::Action;

use std::sync::Arc;
use std::sync::Mutex;

pub trait Reactor {
    type Error;

    fn react(&mut self, event: Event) -> Result<(), Self::Error>;
}

#[derive(Clone)]
struct Context {
    uncommited_buffer: Arc<Mutex<Vec<Event>>>,
}

impl Context {
    pub fn new() -> Self {
        Self { 
            uncommited_buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub struct ReactorCore {
    hook_ctx: Context,
}

impl ReactorCore {
    pub fn new() -> Self {
        Self  {
            hook_ctx: Context::new()
        }
    }

    pub async fn register(&self, lock: &mut OwnedMutexGuard<SqliteConnection>) {
        let mut handle_lock = lock.lock_handle().await.unwrap();
        let handle = handle_lock.as_raw_handle();

        // SAFETY: Its safe to construct the connection from a handle because we lock it above
        // which ensures the access is synchronized.
        let conn = unsafe { Connection::from_handle(handle.as_ptr()).unwrap() };

        conn.update_hook(Some(self.update_hook()));
        conn.commit_hook(Some(self.commit_hook()));
        conn.rollback_hook(Some(self.rollback_hook()));

        // SAFETY: We need to forget `conn` otherwise its destructor runs and our hooks get
        // deleted.
        core::mem::forget(conn);
    }

    pub async fn react<E>(self, reactor: impl Reactor<Error = E>) {
        return;
    }

    pub fn update_hook(&self) -> impl FnMut(Action, &str, &str, i64) + Send + 'static {
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

    pub fn commit_hook(&self) -> impl FnMut() -> bool + Send + 'static {
        || { true }
    }

    pub fn rollback_hook(&self) -> impl FnMut() + Send + 'static {
        let context = self.hook_ctx.clone();

        // NOTE: If we get a rollback, we automatically assume that our current buffer of
        // uncommited events is dirty and will not be applied to the real database, thus we can
        // just clear them.
        move || {
            let mut buffer = context.uncommited_buffer.lock().unwrap();
            buffer.clear();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Table {
    Library,
    Media,
}

impl TryFrom<&str> for Table {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "library" => Ok(Self::Library),
            "media" => Ok(Self::Media),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    Insert,
    Update,
    Delete,
}

impl From<Action> for EventType {
    fn from(action: Action) -> EventType {
        match action {
            Action::SQLITE_INSERT => EventType::Insert,
            Action::SQLITE_UPDATE => EventType::Update,
            Action::SQLITE_DELETE => EventType::Delete,
            _ => unimplemented!()
        }
    }
}

pub struct Event {
    pub id: i64,
    pub event_type: EventType,
    pub table: Table,
}
