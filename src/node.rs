use cluster::Cluster;
use cluster::ClientA;

#[derive(Debug)]
enum Status {
    Follower,
}

#[derive(Debug)]
pub struct Node {
    status: Status,
    pub cluster: Cluster,
}

impl Node {
    pub fn new() -> Node {
        let mut cluster = Cluster::new();
        for address in Self::parse_clients_str("a,b21".to_owned()) {
            cluster.add_client(Box::new(ClientA::new(address)));
        }
        Node {
            status: Status::Follower,
            cluster: cluster,
        }
    }

    fn parse_clients_str(line: String) -> Vec<String> {
        line.split(',').map(|x| x.to_owned()).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
