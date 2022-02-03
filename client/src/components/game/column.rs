use super::card::{Card, CardsList};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Token {
    pub name: String,
    pub path: String,
}

impl Token {
    pub fn new(name: &str) -> Token {
        let name = name.to_string();
        let path = format!("assets/cards/{}.jpeg", name);

        Token { name, path }
    }
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
