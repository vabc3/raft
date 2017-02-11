use std::fmt::Debug;
use client::RaftClient as Client;
use message::{Request, MessageType};

#[derive(Debug)]
pub struct Cluster {
    clients: Vec<Box<Client>>,
}

impl Cluster {
    pub fn new() -> Cluster {
        Cluster { clients: Vec::new() }
    }

    pub fn add_client(&mut self, client: Box<Client>) {
        self.clients.push(client);
    }

    pub fn broadcast(&self) {
        for client in &self.clients {
            let req = Request { message_type: MessageType::Announce };
            client.send(req);
        }
    }
}
