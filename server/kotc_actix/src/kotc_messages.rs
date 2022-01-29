use actix::prelude::Message;
use actix::Recipient;

#[derive(Message)]
#[rtype(result = "()")]
pub struct KotcMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<KotcMessage>,
    pub id: usize,
    pub lobby_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub lobby_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub session_id: usize,
    pub lobby_id: usize,
    pub msg: String,
}
