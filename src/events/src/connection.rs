use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ws::{CloseCode, Handler, Handshake, Message, Sender};

pub type ServerRef = Arc<Mutex<RefCell<ServerInner>>>;

pub struct ServerInner {
    clients: HashMap<String, Vec<Sender>>,
}

#[derive(Clone)]
pub struct Client {
    server: ServerRef,
    sender: Sender,
    resource: Option<String>,
}

impl ServerInner {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    fn add_client(&mut self, resource: &str, sender: &Sender) {
        if let Some(list) = self.clients.get_mut(&resource.to_owned()) {
            list.push(sender.clone());
            return;
        }

        let mut list = Vec::new();
        list.push(sender.clone());

        self.clients.insert(resource.to_owned(), list);
    }

    fn remove_client(&mut self, sender: &Sender) -> Option<()> {
        let token = sender.token();

        for (_, v) in self.clients.iter_mut() {
            let pos = match v.iter().position(|x| x.token() == token) {
                Some(x) => x,
                None => continue,
            };

            v.remove(pos);
        }

        Some(())
    }

    pub fn broadcast(&self, route: &str, msg: String) {
        if let Some(list) = self.clients.get(route) {
            for client in list.iter() {
                client.send(msg.clone()).unwrap()
            }
        };
    }
}

impl Default for ServerInner {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new(server: ServerRef, sender: ws::Sender) -> Self {
        Self {
            server,
            sender,
            resource: None,
        }
    }
}

impl Handler for Client {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        self.server
            .lock()
            .unwrap()
            .borrow_mut()
            .add_client(req.resource(), &self.sender);
        self.resource = Some(req.resource().to_owned());

        let res = ws::Response::from_request(req)?;
        Ok(res)
    }

    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.server
            .lock()
            .unwrap()
            .borrow_mut()
            .remove_client(&self.sender);
    }

    fn on_message(&mut self, _: Message) -> ws::Result<()> {
        Ok(())
    }
}
