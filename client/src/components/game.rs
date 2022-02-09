pub mod card;
pub mod column;
pub mod end_popup;
pub mod logs;
pub mod token;

use std::collections::HashMap;

use kotc_reqwasm::endpoints::{
    CardStore, ColumnsStore, FinalResultsStore, HandStore, LogStore, PlayerOnTurnStore, TokenStore,
};
use yew::prelude::*;

use crate::components::game::end_popup::EndPopup;
use card::{Card, Hand};
use column::{Column as OtherColumn, ColumnsList};
use logs::Logs;
use token::{Stats, Token};
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

#[function_component(Game)]
pub fn game() -> Html {
    let columns_store = use_store::<BasicStore<ColumnsStore>>();
    let hand_store = use_store::<BasicStore<HandStore>>();
    let log_store = use_store::<BasicStore<LogStore>>();
    let token_store = use_store::<BasicStore<TokenStore>>();
    let card_store = use_store::<BasicStore<CardStore>>();
    let player_on_turn_store = use_store::<BasicStore<PlayerOnTurnStore>>();
    let final_results_store = use_store::<BasicStore<FinalResultsStore>>();

    let on_card_select = {
        card_store
            .dispatch()
            .reduce_callback_with(|store, i| store.card = i)
    };

    let columns = match columns_store.state() {
        None => vec![],
        Some(state) => state
            .columns
            .iter()
            .map(|col| {
                OtherColumn::new(
                    Token::new(&col.token.resource, col.token.points),
                    col.cards
                        .iter()
                        .map(|card| Card::new(card))
                        .collect::<Vec<Card>>(),
                )
            })
            .collect::<Vec<OtherColumn>>(),
    };

    let hand = match hand_store.state() {
        None => vec![],
        Some(state) => state
            .hand
            .iter()
            .map(|card| Card::new(card))
            .collect::<Vec<Card>>(),
    };

    let mut logs = match log_store.state() {
        None => vec![],
        Some(state) => state.logs.clone(),
    };
    logs.reverse();

    let player_tokens = match token_store.state() {
        None => HashMap::new(),
        Some(state) => state.tokens.clone(),
    };

    let player_on_turn_banner = match player_on_turn_store.state() {
        None => html! {},
        Some(state) => match state.player.clone() {
            None => html! {},
            Some(player_on_turn) => {
                let color_class = format!("player--{:?}", player_on_turn.color).to_lowercase();
                let players_turn = format!("{:?}'", player_on_turn.username);
                html! {
                    <span class="game__turn-head">
                        <span class={classes!("game__turn-head", color_class.clone())}>
                            {players_turn}
                        </span>
                        {"'s turn"}
                    </span>
                }
            }
        },
    };


    let game = html! {
        <div class="game">
            {player_on_turn_banner}
            <ColumnsList columns={ columns } />
            <Hand cards={ hand } on_click={ on_card_select } class={ "" } />
            <Logs {logs} />
            <Stats stats={player_tokens} />
        </div>
    };

    match final_results_store.state() {
        None => html! {},
        Some(state) => {
            if state.game_ended {
                return html! {
                    <EndPopup results={state.results.clone()} />
                }
            }
            return game
        }
    }
}

