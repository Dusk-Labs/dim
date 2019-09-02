use crate::connection::{Client, ServerInner, ServerRef};
use crate::event::*;
use std::cell::RefCell;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use ws::listen;

pub type EventType = Event; // change this to other events

pub struct Server {
    threads: Vec<std::thread::JoinHandle<()>>,
    _serverref: ServerRef,
    tx: std::sync::mpsc::Sender<EventType>,
}

impl Server {
    pub fn new() -> Self {
        let (tx, rx) = channel::<EventType>();
        let mut threads = Vec::new();
        let inner = Arc::new(Mutex::new(RefCell::new(ServerInner::new())));

        let inner_clone = inner.clone();
        let server_thread = thread::spawn(move || {
            listen("127.0.0.1:3012", |sender| {
                Client::new(inner_clone.clone(), sender)
            })
            .unwrap()
        });

        threads.push(server_thread);

        let clone = inner.clone();
        let message_thread = thread::spawn(move || {
            for event in rx {
                clone
                    .lock()
                    .unwrap()
                    .borrow_mut()
                    .broadcast(&event.get_res(), event.build());
            }
        });
        threads.push(message_thread);

        Self {
            threads,
            _serverref: inner,
            tx,
        }
    }

    pub fn get_tx(&self) -> std::sync::mpsc::Sender<EventType> {
        self.tx.clone()
    }

    pub fn join_threads(&mut self) {
        for thread in self.threads.drain(0..) {
            thread.join().unwrap();
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
