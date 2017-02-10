use std::fmt::Debug;

// TODO why Box<cluster::Client + 'static>
// pub trait Client {
pub trait Client: Debug {
    fn send(&self);
}

#[derive(Debug)]
pub struct ClientA {
    address: String,
}

impl ClientA {
    pub fn new(addr: String) -> ClientA {
        ClientA { address: addr }
    }
}

impl Client for ClientA {
    fn send(&self) {
        println!("Send to {}.", self.address);
    }
}

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
            client.send();
        }
    }
}
