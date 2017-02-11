use hyper::server::{Server as HyperServer, Request, Response};

#[derive(Debug)]
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server { address: address }
    }

    pub fn run(&self) {
        fn hello(req: Request, res: Response) {
            println!("{:?}", req.remote_addr);
            println!("ins");
        }

        HyperServer::http(self.address.as_str()).unwrap().handle(hello).unwrap();
    }
}
