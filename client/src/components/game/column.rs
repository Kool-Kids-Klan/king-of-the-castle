use std::rc::Rc;
use gloo_storage::{SessionStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use super::card::{Card, CardsList};
use super::token::Token;
use kotc_reqwasm::{server_structs::Resource, endpoints::CardStore};
use yew::prelude::*;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;
use kotc_commons::messages::PlayCard;
use kotc_reqwasm::endpoints::User;
use kotc_reqwasm::ws_send::play_card;
use crate::components::pages::lobby::KotcWebSocketState;

#[derive(Clone, PartialEq)]
pub struct Column {
    pub token: Token,
    pub cards: Vec<Card>,
}

impl Column {
    pub fn new(token: Token, cards: Vec<Card>) -> Self {
        Self { token, cards }
    }
}

#[derive(Properties, PartialEq)]
pub struct ColumnProps {
    pub column: Column,
    pub on_click: Callback<MouseEvent>,
    pub on_drop: Callback<DragEvent>,
}

#[function_component(ColumnComponent)]
pub fn column(ColumnProps { column, on_click, on_drop }: &ColumnProps) -> Html {
    let allow_drop = Callback::from(|e: DragEvent| e.prevent_default());

    html! {
        <div class={"game__column"} onclick={on_click} ondrop={on_drop} ondragover={allow_drop} >
            <img class={"game__token"} name={ column.token.name.clone() } src={ column.token.path.clone() } />
            <CardsList cards={column.cards.clone()} class={""} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ColumnsListProps {
    pub columns: Vec<Column>,
}

#[function_component(ColumnsList)]
pub fn columns_list(ColumnsListProps { columns }: &ColumnsListProps) -> Html {
    let card_store = use_store::<BasicStore<CardStore>>();
    let ws_store = use_store::<BasicStore<KotcWebSocketState>>();
    let logged_user: User = SessionStorage::get("user").unwrap();

    let selected_card = match card_store.state() {
        None => None,
        Some(state) => state.card,
    };
    let send_card_to_col = Callback::from(move |i: usize| {
        match selected_card {
            None => {},
            Some(card_index) => {
                log::info!("SENDING CARD {:?} ON COLUMN {}", card_index, i);
                if let Some(ws) = ws_store.state().map(|s| Rc::clone(&s.websocket)) {
                    let client_message = play_card(logged_user.id, card_index, i);
                    spawn_local(async move {
                        match Rc::clone(&ws).borrow_mut().as_mut() {
                            Some(ws) => ws.send_message(client_message).await,
                            None => log::warn!("Websocket was not initialized"),
                        }
                    })
                };
            }
        }
    });

    html! {
        <div id={"game__columns"}>
        {
            columns
                .iter()
                .enumerate()
                .map(|(i, column)| {
                    let on_column_click = { // TODO remove if we want drag
                        // let send_card_to_col = send_card_to_col.clone();
                        let send_card_to_col = Callback::from(|_| println!(""));
                        Callback::from(move |_| {
                            send_card_to_col.emit(i);
                        })
                    };
                    let on_drop = {
                        let on_drag_drop = send_card_to_col.clone();
                        Callback::from(move |e: DragEvent| {
                            e.prevent_default();
                            log::info!("DROPPED");
                            on_drag_drop.emit(i);
                        })
                    };

                    html! {
                        <ColumnComponent column={column.clone()} on_click={on_column_click} on_drop={on_drop} />
                    }
                })
                .collect::<Vec<Html>>()
        }
        </div>
    }
}
