pub mod card;
pub mod column;

use yew::prelude::*;

use card::{Card, CardsList};

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

    let cards = vec![Card::new("king"), Card::new("beggar")];

    html! {
       <div class="game">
          <h1>{"Game"}</h1>
          <CardsList cards={ cards } on_click={ on_card_select } />
          { for details }
       </div>
    }
}
