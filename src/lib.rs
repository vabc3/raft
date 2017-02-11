extern crate hyper;

mod node;
mod cluster;
mod server;

pub use node::Node;

type NodeId = u64;