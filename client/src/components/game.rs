pub mod card;
pub mod column;

use yew::prelude::*;

use card::{Card, CardsList};
use column::{Column, ColumnsList, Token};

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
    ];

    html! {
        <div class="game">
            <div id={"game__columns"}>
                <ColumnsList columns={ columns } />
            </div>
            <div id={"game__hand"}>
                <CardsList cards={ hand } on_click={ on_card_select } />
            </div>
            { for details }
        </div>
    }
}
