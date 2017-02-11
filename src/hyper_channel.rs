use hyper::client::{Request as ClientRequest, Client as HyperClient };
use hyper::server::{Request as HyperRequest, Response as HyperResponse};
use message::{Request, Response, MessageType};
use client::RaftClient;
use NodeId;

// use hyper::header::Headers;
header! { (XRaftTerm, "X-Raft-Term") => [u64] }


#[derive(Debug)]
pub struct HyperRaftClient {
    id: NodeId,
    address: String,
}

impl HyperRaftClient {
    pub fn new(id: NodeId, addr: String) -> HyperRaftClient {
        HyperRaftClient {
            id: id,
            address: addr,
        }
    }
}

impl RaftClient for HyperRaftClient {
    fn send(&self, request:Request) -> Response {
        println!("Send to {}.", self.address);
        let client = HyperClient::new();
        let addr = "http://".to_owned() + self.address.as_str() + "/";
        let rb = client.get(addr.as_str())
            .header(XRaftTerm(1));

        let _ = rb.send();//.unwrap();
        // assert_eq!(res.status, hyper::Ok);
        Response { status: true }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
    }
}