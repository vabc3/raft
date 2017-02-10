use cluster::Cluster;

#[derive(Debug)]
enum Status {
    Follower,
}

#[derive(Debug)]
pub struct Node {
    status: Status,
    cluster: Cluster,
}

impl Node {
    pub fn new() -> Node {
        Node {
            status: Status::Follower,
            cluster: Cluster::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
