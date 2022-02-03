pub mod card;
pub mod column;
pub mod logs;

use yew::prelude::*;

use card::{Card, CardsList, Hand};
use column::{Column, ColumnsList, Token};
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

    let hand = vec![Card::new("king"), Card::new("beggar"), Card::new("beggar"), Card::new("beggar")];
    let columns = vec![
        Column::new(
            Token::new("beggar"),
            hand.clone(),
        ),
        Column::new(
            Token::new("king"),
            hand.clone(),
        ),
        Column::new(
            Token::new("beggar"),
            hand.clone(),
        ),
        Column::new(
            Token::new("beggar"),
            hand.clone(),
        ),
        Column::new(
            Token::new("beggar"),
            hand.clone(),
        ),
        Column::new(
            Token::new("beggar"),
            hand.clone(),
        ),
    ];

    let add_log = {
        let logs = logs.clone();
        Callback::from(move |_| 
            logs.set(
                {
                    let old = logs.iter().map(|s| s.clone());
                    old.chain(vec![String::from("Game updated: added card king to column 2")]).collect::<Vec<String>>()
                }
            )
        )
    };

    html! {
        <div class="game" onclick={ add_log }>
            <ColumnsList columns={ columns } />
            <Hand cards={ hand } on_click={ on_card_select } />
            { for details }
            <Logs logs={ logs.iter().map(|x| x.clone()).collect::<Vec<String>>() } />
        </div>
    }
}
