use super::card::{Card, CardsList};
use kotc_reqwasm::server_structs::Resource;
use yew::prelude::*;

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
}

#[function_component(ColumnComponent)]
pub fn column(ColumnProps { column }: &ColumnProps) -> Html {
    html! {
        <div class={"game__column"}>
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
    html! {
        <div id={"game__columns"}>
        {
            columns
                .iter()
                .map(|column| {
                    html! {
                        <ColumnComponent column={column.clone()} />
                    }
                })
                .collect::<Vec<Html>>()
        }
        </div>
    }
}
