extern crate raft;
use raft::Node;

extern crate hyper;
use hyper::server::{Server, Request, Response};

fn main() {
    let a = Node::new();
    println!("{:?}", a);
    a.cluster.broadcast();

    fn hello(req: Request, res: Response) {
        println!("{:?}",req.remote_addr);
        println!("ins");
    }

    Server::http("0.0.0.0:1025").unwrap().handle(hello).unwrap();
    println!("not");
}