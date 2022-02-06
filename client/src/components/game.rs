pub mod card;
pub mod column;
pub mod logs;
pub mod token;

use std::collections::HashMap;

use kotc_reqwasm::{endpoints::{ColumnsStore, HandStore, LogStore, TokenStore, CardStore}};
use yew::prelude::*;

use card::{Card, Hand};
use column::{Column as OtherColumn, ColumnsList};
use token::{Token, Stats};
use logs::Logs;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

#[function_component(Game)]
pub fn game() -> Html {
    let columns_store = use_store::<BasicStore<ColumnsStore>>();
    let hand_store = use_store::<BasicStore<HandStore>>();
    let log_store = use_store::<BasicStore<LogStore>>();
    let token_store = use_store::<BasicStore<TokenStore>>();
    let card_store = use_store::<BasicStore<CardStore>>();

    let on_card_select = {
        card_store.dispatch().reduce_callback_with(|store, i| store.card = i)
    };

    let columns = match columns_store.state() {
        None => vec![],
        Some(state) => {
            state.columns.iter()
                .map(|col| OtherColumn::new(
                    Token::new(&col.token.resource, col.token.points),
                    col.cards.iter().map(|card| Card::new(card)).collect::<Vec<Card>>(),
                ))
                .collect::<Vec<OtherColumn>>()
        }
    };

    let hand = match hand_store.state() {
        None => vec![],
        Some(state) => {
            state.hand.iter()
                .map(|card| Card::new(card))
                .collect::<Vec<Card>>()
        }
    };

    let logs = match log_store.state() {
        None => vec![],
        Some(state) => state.logs.clone(),
    };

    let player_tokens = match token_store.state() {
        None => HashMap::new(),
        Some(state) => state.tokens.clone(),
    };

    html! {
        <div class="game">
            <ColumnsList columns={ columns } />
            <Hand cards={ hand } on_click={ on_card_select } />
            <Logs {logs} />
            <Stats stats={player_tokens} />
        </div>
    }
}
