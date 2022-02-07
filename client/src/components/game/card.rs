use kotc_reqwasm::server_structs::Card as ServerCard;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Card {
    pub name: String,
    pub path: String,
}

impl Card {
    pub fn new(card: &ServerCard) -> Card {
        let name = format!("{:?}", card.character).to_lowercase();
        let color = format!("{:?}", card.color).to_lowercase();
        let path = format!("assets/cards/characters/{}/{}_{}.png", name, name, color);

        Card { name, path }
    }
}

fn def_on_click() -> Callback<Option<usize>> {
    Callback::from(move |_: Option<usize>| println!("card selected"))
}

#[derive(Clone, Properties, PartialEq)]
pub struct CardsListProps {
    pub cards: Vec<Card>,
    #[prop_or_else(def_on_click)]
    pub on_click: Callback<Option<usize>>,
    pub class: String,
}

#[function_component(CardsList)]
pub fn cards_list(CardsListProps { cards, on_click, class }: &CardsListProps) -> Html {
    cards.iter().enumerate().map(|(i, card)| {
        let on_card_select = {
            let on_click = on_click.clone();
            Callback::from(move |_| {
                on_click.emit(Some(i));
            })
        };

        let drag = {
            let on_click = on_click.clone();
            Callback::from(move |_| {
                on_click.emit(Some(i));
            })
        };

        html! {
            <img { class } name={ card.name.clone() } src={ card.path.clone() } onclick={on_card_select} draggable={"true"}  ondragstart={drag} />
        }
    }).collect()
}

#[function_component(Hand)]
pub fn hand(CardsListProps { cards, on_click, class: _ }: &CardsListProps) -> Html {
    html! {
        <div id={"game__hand"}>
            <CardsList cards={ cards.clone() } { on_click } class={ "card" } />
        </div>
    }
}
