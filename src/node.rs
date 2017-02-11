use cluster::Cluster;
use cluster::ClientA;
use NodeId;

#[derive(Debug)]
enum Status {
    Follower,
}

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub cluster: Cluster,
    status: Status,
}

impl Node {
    pub fn new(id: NodeId, node_config: String) -> Node {
        let mut cluster = Cluster::new();
        let mut index = 0;
        for address in Self::parse_clients_str(node_config) {
            if index != id {
                cluster.add_client(Box::new(ClientA::new(index, address)));
            }
            
            index += 1;
        }
        Node {
            id: id,
            cluster: cluster,
            status: Status::Follower,
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
