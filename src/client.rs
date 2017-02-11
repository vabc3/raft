use std::fmt::Debug;

// TODO why Box<cluster::Client + 'static>
// pub trait Client {
pub trait RaftClient: Debug {
    fn send(&self);
}