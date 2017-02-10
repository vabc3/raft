use std::fmt::{Debug, Formatter, Error};

pub trait Client {
    fn send(&self) {}
}

pub struct ClientA {}

impl Client for ClientA {
    fn send(&self) {}
}

impl Debug for ClientA {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<ClientA:{}>", 7)
    }
}

#[derive(Debug)]
pub struct Cluster<C: Client> {
    clients: Vec<C>,
}

impl<C> Cluster<C>
    where C: Client
{
    pub fn new() -> Cluster<C> {
        Cluster { clients: Vec::new() }
    }

    pub fn add_client(&mut self, client: C) {
        self.clients.push(client);
    }
}
