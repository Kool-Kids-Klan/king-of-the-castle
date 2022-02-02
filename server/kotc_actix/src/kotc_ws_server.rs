use crate::kotc_messages::{ClientMessage, Connect, Disconnect, Error, ServerWsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap};
use kotc_commons::messages::{ClientWsMessage, PlayCard};
use kotc_commons::messages::message_types::ServerWsMessageType;
use crate::lobby::Lobby;

pub type Socket = Recipient<ServerWsMessage>;

pub struct KotcWsServer {
    sessions: HashMap<usize, Socket>,        // map of all sockets
    lobbies: HashMap<usize, Lobby>, // map of all lobbies
}

impl Default for KotcWsServer {
    fn default() -> Self {
        KotcWsServer {
            sessions: HashMap::new(),
            lobbies: HashMap::new(),
        }
    }
}

impl KotcWsServer {
    fn send_message(&self, message_type: ServerWsMessageType, content: &String, id_to: &usize) {
        if let Some(recipient) = self.sessions.get(id_to) {
            let _ = recipient.do_send(ServerWsMessage {
                message_type,
                content: content.clone(),
            });
        } else {
            println!(
                "Could not send message to user with id='{}'. User not found.",
                id_to
            );
        };
    }
}

impl Actor for KotcWsServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for KotcWsServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.lobbies
            .entry(msg.lobby_id)
            .or_insert_with(Lobby::new)
            .sessions.insert(msg.id);

        self.lobbies
            .get(&msg.lobby_id)
            .unwrap()
            .sessions.iter()
            .filter(|connection_id| *connection_id.to_owned() != msg.id)
            .for_each(|connection_id| {
                self.send_message(ServerWsMessageType::UserJoined, &format!("User {} joined", msg.id), connection_id)
            });

        self.sessions.insert(msg.id, msg.addr);

        self.send_message(ServerWsMessageType::YourId, &format!("Your id is {}", msg.id), &msg.id);
    }
}

impl Handler<Disconnect> for KotcWsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        if self.sessions.remove(&msg.id).is_some() {
            self.lobbies
                .get(&msg.lobby_id)
                .unwrap()
                .sessions.iter()
                .filter(|session_id| *session_id.to_owned() != msg.id)
                .for_each(|session_id| {
                    self.send_message(ServerWsMessageType::UserDisconnected, &format!("User {} disconnected.", &msg.id), session_id)
                });
            if let Some(lobby) = self.lobbies.get_mut(&msg.lobby_id) {
                if lobby.sessions.len() > 1 {
                    lobby.sessions.remove(&msg.id); // remove the user from lobby (there are other users)
                } else {
                    self.lobbies.remove(&msg.lobby_id); // remove the lobby, because it is empty
                };
            };
        };
    }
}

impl Handler<ClientMessage> for KotcWsServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        println!("this is message string {}", msg.msg);
        let client_message: ClientWsMessage = serde_json::from_str(&msg.msg).unwrap();
        println!("this is message object {:?}", client_message);
        let error_message = Error {
            detail: String::from("this is fucking error"),
        };
        let error_message_serialized = serde_json::to_string(&error_message).unwrap();
        self.lobbies
            .get(&msg.lobby_id)
            .unwrap()
            .sessions.iter()
            .for_each(|client| self.send_message(ServerWsMessageType::Error, &error_message_serialized, client)); // TODO: DO NOT use unwrap!!
    }
}
