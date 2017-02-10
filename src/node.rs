use cluster::Cluster;
use cluster::ClientA;

#[derive(Debug)]
enum Status {
    Follower,
}

#[derive(Debug)]
pub struct Node {
    status: Status,
    cluster: Cluster<ClientA>,
}

impl Node {
    pub fn new() -> Node {
        let mut cluster = Cluster::new();
        cluster.add_client(ClientA {});
        cluster.add_client(ClientA {});
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
