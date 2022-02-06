pub mod card;
pub mod column;
pub mod logs;

use std::collections::HashMap;

use kotc_reqwasm::{endpoints::{ColumnsStore, HandStore}};
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

    let logs: UseStateHandle<Vec<String>> = use_state(|| vec![String::from("LOG START")]);

    let player_tokens: UseStateHandle<HashMap<String, Vec<Token>>> = use_state(|| HashMap::new());

    // let hand = vec![Card::new("king/king_black"), Card::new("king/king_black"), Card::new("king/king_black")];
    // let columns = vec![
    //     OtherColumn::new(
    //         Token::new("coins_1"),
    //         hand.clone(),
    //     ),
    //     OtherColumn::new(
    //         Token::new("coins_1"),
    //         hand.clone(),
    //     ),
    //     OtherColumn::new(
    //         Token::new("coins_1"),
    //         hand.clone(),
    //     ),
    //     OtherColumn::new(
    //         Token::new("coins_1"),
    //         hand.clone(),
    //     ),
    //     OtherColumn::new(
    //         Token::new("coins_1"),
    //         hand.clone(),
    //     ),
    //     OtherColumn::new(
    //         Token::new("coins_1"),
    //         hand.clone(),
    //     ),
    // ];

    // LOG ADDING
    // let update = {
    //     let logs = logs.clone();
    //     Callback::from(move |_| 
    //         logs.set(
    //             logs
    //                 .iter()
    //                 .map(|s| s.clone())
    //                 .chain(vec![String::from("Game updated: added card king to column 2")])
    //                 .collect::<Vec<String>>()
    //         )
    //     )
    // };

    // TOKENS UPDATING
    // let update = {
    //     let player_tokens = player_tokens.clone();
    //     Callback::from(move |_| 
    //         player_tokens.set(
    //             player_tokens
    //                 .iter()
    //                 .map(|(key, value)| (key.clone(), value.clone()))
    //                 .chain(vec![("AAA".to_string(), vec![Token::new(&Resource::Coins, 3), Token::new(&Resource::Coins, 2), Token::new(&Resource::Coins, 1),])])
    //                 .chain(vec![("BBB".to_string(), vec![Token::new(&Resource::Coins, 3), Token::new(&Resource::Coins, 2),])])
    //                 .chain(vec![("CCC".to_string(), vec![Token::new(&Resource::Coins, 5),])])
    //                 .collect()
    //         )
    //     )
    // };

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

    html! {
        <div class="game">
            <ColumnsList columns={ columns } />
            <Hand cards={ hand } on_click={ on_card_select } />
            { for details }
            <Logs logs={ logs.iter().map(|x| x.clone()).collect::<Vec<String>>() } />

            <div class={"game__tokens"}>
                { for player_tokens.iter().map(|(key, value)| html! {
                    <>
                        <p>{key.clone()}</p>
                        {
                            html! {
                                <TokenList tokens={value.iter().map(|token| token.clone()).collect::<Vec<Token>>()} />
                            }
                        }
                    </>
                }) }
            </div>
        </div>
    }
}
