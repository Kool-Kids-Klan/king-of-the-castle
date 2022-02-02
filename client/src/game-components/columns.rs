use yew::prelude::*;
use yew::{Component, Message, Properties, Context, Html, html};
use super::cards::CardsList;

#[derive(Clone, PartialEq)]
pub struct Token {
    pub name: String,
    pub path: String,
}

impl Token {
    pub fn new(name: &str) -> Token {
        let name = name.to_string();
        let path = format!("assets/tokens/{}.jpeg", name);

        Token { name, path }
    }
}

#[derive(Clone, PartialEq)]
pub struct Column {
    pub token: Token,
    pub cards: Vec<Cards>
}

pub struct ColumnProps {
    pub column: Column,
}


#[function_component(ColumnComponent)]
pub fn column(ColumnProps { column }: &CardsListProps) -> Html {
    html! {
        <div>
            <img name={ column.token.name.clone() } src={ column.token.path.clone() } />
            { self.cards_list }
        </div>
    }
}

pub struct ColumnsListProps {
    pub columns: Vec<Column>,
}

#[function_component(ColumnsList)]
pub fn columns_list(ColumnsListProps { columns }: &ColumnsListProps) -> Html {
    columns.iter().map(|column| {
        html! {
           <ColumnComponent column=column />
        }
    }).collect()
}