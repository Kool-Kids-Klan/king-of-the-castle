pub mod card;
pub mod column;
pub mod logs;

use std::collections::HashMap;

use kotc_reqwasm::{endpoints::{ColumnsStore, HandStore, LogStore, TokenStore}};
use yew::prelude::*;

use card::{Card, Hand};
use column::{Column as OtherColumn, ColumnsList, Token, TokenList};
use logs::Logs;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

#[function_component(Game)]
pub fn game() -> Html {
    let columns_store = use_store::<BasicStore<ColumnsStore>>();
    let hand_store = use_store::<BasicStore<HandStore>>();
    let log_store = use_store::<BasicStore<LogStore>>();
    let token_store = use_store::<BasicStore<TokenStore>>();

    let selected_card = use_state(|| None);
    let on_card_select = {
        let selected_card = selected_card.clone();
        Callback::from(move |card: Card| selected_card.set(Some(card)))
    };

    let details = selected_card.as_ref().map(|card| {
        html! {
           <p>{ card.name.clone() }</p>
        }
    });

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
            { for details }
            <Logs {logs} />

            <div class={"game__tokens"}>
                { for player_tokens.iter().map(|(key, value)| html! {
                    <>
                        <p>{key.clone()}</p>
                        {
                            html! {
                                <TokenList tokens={value.iter().map(|token| Token::new(&token.resource, token.points)).collect::<Vec<Token>>()} />
                            }
                        }
                    </>
                }) }
            </div>
        </div>
    }
}
