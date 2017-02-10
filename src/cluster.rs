#[derive(Debug)]
struct Client {
    id: i64,
}

#[derive(Debug)]
pub struct Cluster {
    clients: Vec<Client>
}

impl Cluster {
    pub fn new() -> Cluster {
        Cluster { clients: Vec::new() }
    }
}