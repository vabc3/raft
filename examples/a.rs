extern crate raft;
use raft::Node;

extern crate hyper;
use hyper::server::{Server, Request, Response};

fn main() {
    let node_config = "127.0.0.1:2001, 127.0.0.1:2002, 127.0.0.1:2003".to_owned();
    let a = Node::new(0, node_config);
    println!("{:?}", a);
    a.cluster.broadcast();

    fn hello(req: Request, res: Response) {
        println!("{:?}",req.remote_addr);
        println!("ins");
    }

    Server::http("0.0.0.0:1025").unwrap().handle(hello).unwrap();
    println!("not");
}