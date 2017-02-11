extern crate raft;
use raft::Node;

extern crate hyper;
use hyper::server::{Server, Request, Response};

use std::env;

fn main() {
    let mut node_id = 0;
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        node_id = match args[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input is not integer.");
                return;
            }
        };
    }

    let node_config = "127.0.0.1:2000, 127.0.0.1:2001, 127.0.0.1:2002".to_owned();
    let a = Node::new(node_id, node_config);
    println!("{:?}", a);
    a.cluster.broadcast();

    // fn hello(req: Request, res: Response) {
    //     println!("{:?}",req.remote_addr);
    //     println!("ins");
    // }

    // Server::http("0.0.0.0:1025").unwrap().handle(hello).unwrap();
    println!("not");
}