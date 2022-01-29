use crate::kotc_messages::{ClientMessage, Connect, Disconnect, KotcMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};

type Socket = Recipient<KotcMessage>;

pub struct KotcWsServer {
    sessions: HashMap<usize, Socket>,        // map of all sockets
    lobbies: HashMap<usize, HashSet<usize>>, // map of all lobbies
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
    fn send_message(&self, message: &str, id_to: &usize) {
        if let Some(recipient) = self.sessions.get(id_to) {
            let _ = recipient.do_send(KotcMessage(String::from(message)));
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
            .or_insert_with(HashSet::new)
            .insert(msg.id);

        self.lobbies
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|connection_id| *connection_id.to_owned() != msg.id)
            .for_each(|connection_id| {
                self.send_message(&format!("User {} joined", msg.id), connection_id)
            });

        self.sessions.insert(msg.id, msg.addr);

        self.send_message(&format!("Your id is {}", msg.id), &msg.id);
    }
}

impl Handler<Disconnect> for KotcWsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        if self.sessions.remove(&msg.id).is_some() {
            self.lobbies
                .get(&msg.lobby_id)
                .unwrap()
                .iter()
                .filter(|session_id| *session_id.to_owned() != msg.id)
                .for_each(|session_id| {
                    self.send_message(&format!("User {} disconnected.", &msg.id), session_id)
                });
            if let Some(lobby) = self.lobbies.get_mut(&msg.lobby_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id); // remove the user from lobby (there are other users)
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
        self.lobbies
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_message(&msg.msg, client)); // TODO: DO NOT use unwrap!!
    }
}
