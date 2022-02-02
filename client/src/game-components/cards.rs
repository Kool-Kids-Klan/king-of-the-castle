use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Card {
    pub name: String,
    pub path: String,
}

impl Card {
    pub fn new(name: &str) -> Card {
        let name = name.to_string();
        let path = format!("assets/cards/{}.jpeg", name);

        Card { name, path }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct CardsListProps {
    pub cards: Vec<Card>,
    pub on_click: Callback<Card>,
}

#[function_component(CardsList)]
pub fn cards_list(CardsListProps { cards, on_click }: &CardsListProps) -> Html {
    cards.iter().map(|card| {
        let on_card_select = {
            let on_click = on_click.clone();
            let card = card.clone();
            Callback::from(move |_| {
                on_click.emit(card.clone())
            })
        };

        html! {
            <img name={ card.name.clone() } src={ card.path.clone() } onclick={on_card_select} />
        }
    }).collect()
}