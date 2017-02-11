extern crate raft;
use raft::Node;
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

    a.main_loop();
    println!("not");
}