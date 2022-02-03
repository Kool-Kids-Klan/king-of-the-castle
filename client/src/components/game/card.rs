use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Card {
    pub name: String,
    pub path: String,
}

impl Card {
    pub fn new(name: &str) -> Card {
        let name = name.to_string();
        let path = format!("assets/cards/backsides/{}.png", name);

        Card { name, path }
    }
}

fn def_on_click() -> Callback<Card> {
    Callback::from(move |card: Card| println!("card selected"))
}

#[derive(Clone, Properties, PartialEq)]
pub struct CardsListProps {
    pub cards: Vec<Card>,
    #[prop_or_else(def_on_click)]
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

#[function_component(Hand)]
pub fn hand(CardsListProps { cards, on_click }: &CardsListProps) -> Html {
    html! {
        <div id={"game__hand"}>
            <CardsList cards={cards.clone()} on_click={on_click} />
        </div>
    }
}
