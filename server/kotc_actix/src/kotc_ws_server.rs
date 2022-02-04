use crate::kotc_messages::{ClientMessage, Connect, Disconnect, ServerWsMessage, WsAction};
use crate::lobby::Lobby;
use actix::prelude::{Actor, Context, Handler, Recipient};
use kotc_commons::messages::message_types::{ClientWsMessageType, ServerWsMessageType};
use kotc_commons::messages::{ClientWsMessage, Error, PlayCard, Ready, UnReady, UserJoined};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use actix::AsyncContext;
use serde::de::DeserializeOwned;
use kotc_game::game::card::{Card, Character};
use kotc_game::game::Game;
use kotc_game::game::ws_messages::{MessageRecipient, ServerMessage};

pub type Socket = Recipient<ServerWsMessage>;

#[derive(Clone)]
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

fn deserialize<T: DeserializeOwned>(serialized: &String) -> T {
    serde_json::from_str(serialized).unwrap()
}

fn send_messages(session_id: &usize, messages: Vec<ServerMessage>, sessions: &HashSet<usize>, this: &KotcWsServer) {
    messages
        .iter()
        .for_each(|message| {
            match message.recipient {
                MessageRecipient::SingleUser =>{
                    this.send_message (
                        message.message_type.clone(),
                        &message.content.clone(),
                        session_id,
                    )
                },
                MessageRecipient::AllUsers => {
                    sessions
                        .iter()
                        .for_each(|client| {
                            this.send_message(
                                message.message_type.clone(),
                                &message.content.clone(),
                                client,
                            )
                        })
                },
            };
        });
}

impl Handler<ClientMessage> for KotcWsServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        let client_message: ClientWsMessage = deserialize(&msg.msg);
        let lobby = self.lobbies.get(&msg.lobby_id).unwrap();
        let mut game = lobby.game.clone();
        let sessions = lobby.sessions.clone();
        let this = self.clone();

        match client_message.message_type {
            ClientWsMessageType::UserJoined => {
                let user_joined: UserJoined = deserialize(&client_message.content);
                let fut = async move {
                    let messages = game.add_player(user_joined.user_id).await;
                    send_messages(&msg.session_id, messages, &sessions, &this);
                };
                let fut = actix::fut::wrap_future::<_, Self>(fut);
                // ctx.spawn(fut);
                ctx.wait(fut);
                println!("user joined {:?}", user_joined);
            },
            ClientWsMessageType::PlayCard => {
                let play_card: PlayCard = deserialize(&client_message.content);
                let fut = async move {
                    let messages = game.make_action(play_card.user_id, play_card.column_index, play_card.card_index).await;
                    send_messages(&msg.session_id, messages, &sessions, &this);
                };
                let fut = actix::fut::wrap_future::<_, Self>(fut);
                // ctx.spawn(fut);
                ctx.wait(fut);
                println!("Play card {:?}", play_card);
            },
            ClientWsMessageType::Ready => {
                let ready: Ready = deserialize(&client_message.content);
                let message = game.player_flip_ready(ready.user_id);
                send_messages(&msg.session_id, vec![message], &sessions, &this);
                println!("user ready {:?}", ready);
            },
            ClientWsMessageType::Unready => {
                let unready: UnReady = deserialize(&client_message.content);
                let message = game.player_flip_ready(unready.user_id);
                send_messages(&msg.session_id, vec![message], &sessions, &this);
                println!("user unready {:?}", unready);
            }
            ClientWsMessageType::Error => {
                let error: Error = deserialize(&client_message.content);
                println!("error {:?}", error);
            }
        }
    }
}
