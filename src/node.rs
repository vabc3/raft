use cluster::Cluster;
use cluster::ClientA;

#[derive(Debug)]
enum Status {
    Follower,
}

#[derive(Debug)]
pub struct Node {
    status: Status,
    cluster: Cluster,
}

impl Node {
    pub fn new() -> Node {
        let mut cluster = Cluster::new();
        cluster.add_client(Box::new(ClientA::new()));
        cluster.add_client(Box::new(ClientA::new()));
        Node {
            status: Status::Follower,
            cluster: cluster,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
