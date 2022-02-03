pub mod card;
pub mod column;
pub mod logs;

use std::collections::HashMap;

use yew::prelude::*;

use card::{Card, CardsList, Hand};
use column::{Column, ColumnsList, Token, TokenList};
use logs::Logs;

#[function_component(Game)]
pub fn game() -> Html {
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

    let hand = vec![Card::new("king/king_black"), Card::new("king/king_black"), Card::new("king/king_black")];
    let columns = vec![
        Column::new(
            Token::new("coins_1"),
            hand.clone(),
        ),
        Column::new(
            Token::new("coins_1"),
            hand.clone(),
        ),
        Column::new(
            Token::new("coins_1"),
            hand.clone(),
        ),
        Column::new(
            Token::new("coins_1"),
            hand.clone(),
        ),
        Column::new(
            Token::new("coins_1"),
            hand.clone(),
        ),
        Column::new(
            Token::new("coins_1"),
            hand.clone(),
        ),
    ];

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
    let update = {
        let player_tokens = player_tokens.clone();
        Callback::from(move |_| 
            player_tokens.set(
                player_tokens
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .chain(vec![("AAA".to_string(), vec![Token::new("beggar"), Token::new("beggar"), Token::new("king"),])])
                    .chain(vec![("BBB".to_string(), vec![Token::new("beggar"), Token::new("king"),])])
                    .chain(vec![("CCC".to_string(), vec![Token::new("beggar"),])])
                    .collect()
            )
        )
    };

    html! {
        <div class="game" onclick={ update }>
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
