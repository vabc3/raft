pub enum MessageType {
    Announce,
}

pub struct Request {
    pub message_type: MessageType,
}

pub struct Response {
    pub status: bool,
}