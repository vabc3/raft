use hyper::client::{Request as ClientRequest, Client as HC1 };
use hyper::server::{Request as HyperRequest, Response as HyperResponse};
use message::{Request, Response, MessageType};
use cluster::Client;
use NodeId;

#[derive(Debug)]
pub struct HyperClient {
    id: NodeId,
    address: String,
}

impl HyperClient {
    pub fn new(id: NodeId, addr: String) -> HyperClient {
        HyperClient {
            id: id,
            address: addr,
        }
    }
}

impl Client for HyperClient {
    fn send(&self) {
        println!("Send to {}.", self.address);
        let client = HC1::new();
        let addr = "http://".to_owned() + self.address.as_str();
        let _ = client.get(addr.as_str()).send();//.unwrap();
        // assert_eq!(res.status, hyper::Ok);
    }
}



impl<'a> From<HyperResponse<'a>> for Response {
    fn from(_: HyperResponse<'a>) -> Response {
        // Response { message_type: MessageType::Announce }
        Response { status: true }
    }
}

impl<'a> From<Request> for ClientRequest<i32> {
    fn from(_: Request) -> ClientRequest<i32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use hyper::server::{Request as HyperRequest, Response as HyperResponse};
    use message::Response;
    #[test]
    fn convert() {

        // pub version: version::HttpVersion,
        // // Stream the Response is writing to, not accessible through UnwrittenResponse
        // body: HttpWriter<&'a mut (Write + 'a)>,
        // // The status code for the request.
        // status: status::StatusCode,
        // // The outgoing headers on this response.
        // headers: &'a mut header::Headers,

        // _writing: PhantomData<W>

    }
}