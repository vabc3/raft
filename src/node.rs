use NodeId;
use cluster::{Cluster,ClientA};
use server::Server as RaftServer;

#[derive(Debug)]
enum Status {
    Follower,
}

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub cluster: Cluster,
    status: Status,
    server: RaftServer,
}

impl Node {
    pub fn new(id: NodeId, node_config: String) -> Node {
        let mut cluster = Cluster::new();
        let mut index = 0;
        let mut server_address: Option<String> = None;
        for address in Self::parse_clients_str(node_config) {
            if index != id {
                cluster.add_client(Box::new(ClientA::new(index, address)));
            } else {
                server_address = Some(address);
            }

            index += 1;
        }
        Node {
            id: id,
            cluster: cluster,
            status: Status::Follower,
            server: RaftServer::new(server_address.expect("server addr be cound.")),
        }
    }

    pub fn main_loop(&self) {
        self.server.run();
    }

    fn parse_clients_str(line: String) -> Vec<String> {
        line.split(',').map(|x| x.trim().to_owned()).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
