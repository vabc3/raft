extern crate raft;
use raft::Node;

fn main() {
    let a = Node::new();
    println!("{:?}", a);
    a.cluster.broadcast();
}