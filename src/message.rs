#[derive(Debug, Clone)]
pub enum MessageType {
    SyncRequest,
    SyncReply { time: u64 },
    Ping,
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Message {
    pub from: usize,
    pub to: usize,
    pub payload: MessageType,
}
