use crate::kotc_messages::{ClientMessage, Connect, Disconnect, ServerWsMessage, WsAction};
use crate::lobby::Lobby;
use actix::prelude::{Actor, Context, Handler, Recipient};
use kotc_commons::messages::message_types::ServerWsMessageType;
use kotc_commons::messages::{ClientWsMessage};
use std::collections::HashMap;
use kotc_game::game::card::{Card, Character};
use kotc_game::game::ws_messages::UpdateHand;

pub type Socket = Recipient<ServerWsMessage>;

pub struct KotcWsServer {
    sessions: HashMap<usize, Socket>, // map of all sockets
    lobbies: HashMap<usize, Lobby>,   // map of all lobbies
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
            .sessions
            .insert(msg.id);

        let ws_action = WsAction {
            detail: format!("User {} joined", msg.id),
        };
        let ws_action_serialized = serde_json::to_string(&ws_action).unwrap();
        self.lobbies
            .get(&msg.lobby_id)
            .unwrap()
            .sessions
            .iter()
            .filter(|connection_id| *connection_id.to_owned() != msg.id)
            .for_each(|connection_id| {
                self.send_message(
                    ServerWsMessageType::WsAction,
                    &ws_action_serialized,
                    connection_id,
                )
            });

        self.sessions.insert(msg.id, msg.addr);

        let ws_action = WsAction {
            detail: format!("Your id is {}", msg.id),
        };
        let ws_action_serialized = serde_json::to_string(&ws_action).unwrap();
        self.send_message(
            ServerWsMessageType::WsAction,
            &ws_action_serialized,
            &msg.id,
        );
    }
}

impl Handler<Disconnect> for KotcWsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        let ws_action = WsAction {
            detail: format!("User {} disconnected.", &msg.id),
        };
        let ws_action_serialized = serde_json::to_string(&ws_action).unwrap();
        if self.sessions.remove(&msg.id).is_some() {
            self.lobbies
                .get(&msg.lobby_id)
                .unwrap()
                .sessions
                .iter()
                .filter(|session_id| *session_id.to_owned() != msg.id)
                .for_each(|session_id| {
                    self.send_message(
                        ServerWsMessageType::WsAction,
                        &ws_action_serialized,
                        session_id,
                    )
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
        let mut card = Vec::new();
        card.push(Card::new(String::from("owner"), Character::Alchemist, 10.0));
        let update_hand = UpdateHand {
            hand: card,
        };
        let update_hand_serialized = serde_json::to_string(&update_hand).unwrap();
        for _ in 0..10 {
            self.lobbies
                .get(&msg.lobby_id)
                .unwrap()
                .sessions
                .iter()
                .for_each(|client| {
                    self.send_message(
                        ServerWsMessageType::UpdateHand,
                        &update_hand_serialized,
                        client,
                    )
                });
        } // TODO: DO NOT use unwrap!!
    }
}
