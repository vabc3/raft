#[derive(Debug)]
pub enum MessageType {
    Announce,
}

#[derive(Debug)]
pub struct Request {
    pub message_type: MessageType,
    pub term: u64,
}

pub struct Response {
    pub status: bool,
}