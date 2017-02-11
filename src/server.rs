use hyper::server::{Server as HyperServer, Request, Response};
use message::Request as RaftRequest;

#[derive(Debug)]
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server { address: address }
    }

    pub fn run(&self) {
        fn hello(req: Request, _: Response) {
            println!("{:?}", req.remote_addr);
            
            let rr: RaftRequest = req.into();
            println!("{:?}", rr);
            // for header in req.headers.iter() {
            //     println!("{:?}", header);
            // }
        }

        HyperServer::http(self.address.as_str()).unwrap().handle(hello).unwrap();
    }
}
