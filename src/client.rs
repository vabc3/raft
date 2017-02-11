use std::fmt::Debug;
use message::{Request, Response};
// TODO why Box<cluster::Client + 'static>
// pub trait Client {
pub trait RaftClient: Debug {
    fn send(&self, Request) -> Response;
}