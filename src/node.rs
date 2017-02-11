use NodeId;
use cluster::{Cluster};
use hyper_channel::HyperClient;
use server::Server as RaftServer;
use std::{thread,time};


#[derive(Debug)]
enum Status {
    Follower,
}

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub cluster: Cluster,
    status: Status,
    server_address: String, 
    // server: RaftServer,
}

impl Node {
    pub fn new(id: NodeId, node_config: String) -> Node {
        let mut cluster = Cluster::new();
        let mut index = 0;
        let mut server_address: Option<String> = None;
        for address in Self::parse_clients_str(node_config) {
            if index != id {
                cluster.add_client(Box::new(HyperClient::new(index, address)));
            } else {
                server_address = Some(address);
            }

            index += 1;
        }
        Node {
            id: id,
            cluster: cluster,
            status: Status::Follower,
            // server: RaftServer::new(server_address.expect("server addr be cound.")),
            server_address: server_address.expect("server addr be cound."),
        }
    }

    pub fn main_loop(&self) {
        let addr = self.server_address.clone();
        thread::spawn(move || {
            let server = RaftServer::new(addr);
            server.run();
        });
        loop {
            thread::sleep(time::Duration::from_secs(3));
            println!("Working");
            self.cluster.broadcast();
        }
    }

    fn parse_clients_str(line: String) -> Vec<String> {
        line.split(',').map(|x| x.trim().to_owned()).collect()
    }
}
