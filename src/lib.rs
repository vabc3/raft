extern crate hyper;

mod client;
mod cluster;
mod message;
mod node;
mod server;
mod hyper_channel;

pub use node::Node;

type NodeId = u64;