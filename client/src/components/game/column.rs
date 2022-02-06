use super::card::{Card, CardsList};
use kotc_reqwasm::{server_structs::Resource, endpoints::CardStore};
use yew::prelude::*;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

#[derive(Clone, PartialEq)]
pub struct Token {
    pub name: String,
    pub path: String,
}

impl Token {
    pub fn new(resource: &Resource, points: u8) -> Token {
        let name = format!("{:?}", resource).to_lowercase();
        let path = format!("assets/cards/points/{}_{}.png", name, points);

        Token { name, path }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct TokenListProps {
    pub tokens: Vec<Token>,
}

#[function_component(TokenList)]
pub fn token_list(TokenListProps { tokens }: &TokenListProps) -> Html {
    tokens.iter().map(|token| {
        html! {
            <img name={ token.name.clone() } src={ token.path.clone() } />
        }
    }).collect()
}

#[derive(Clone, PartialEq)]
pub struct Column {
    pub token: Token,
    pub cards: Vec<Card>,
}

impl Column {
    pub fn new(token: Token, cards: Vec<Card>) -> Self {
        Self { token, cards }
    }
}

#[derive(Properties, PartialEq)]
pub struct ColumnProps {
    pub column: Column,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(ColumnComponent)]
pub fn column(ColumnProps { column, on_click }: &ColumnProps) -> Html {
    html! {
        <div class={"game__column"} onclick={on_click}>
            <img class={"game__token"} name={ column.token.name.clone() } src={ column.token.path.clone() } />
            <CardsList cards={column.cards.clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ColumnsListProps {
    pub columns: Vec<Column>,
}

#[function_component(ColumnsList)]
pub fn columns_list(ColumnsListProps { columns }: &ColumnsListProps) -> Html {
    let card_store = use_store::<BasicStore<CardStore>>();
    let selected_card = match card_store.state() {
        None => None,
        Some(state) => state.card,
    };
    let send_card_to_col = Callback::from(move |i: usize| {
        match selected_card {
            None => {},
            Some(card_index) => {
                log::info!("SENDING CARD {:?} ON COLUMN {}", card_index, i);
                // TODO send message
            }
        }
    });

    html! {
        <div id={"game__columns"}>
        {
            columns
                .iter()
                .enumerate()
                .map(|(i, column)| {
                    let on_column_click = {
                        let send_card_to_col = send_card_to_col.clone();
                        Callback::from(move |_| {
                            send_card_to_col.emit(i);
                        })
                    };

                    html! {
                        <ColumnComponent column={column.clone()} on_click={on_column_click} />
                    }
                })
                .collect::<Vec<Html>>()
        }
        </div>
    }
}
