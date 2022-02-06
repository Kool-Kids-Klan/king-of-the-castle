use std::collections::HashMap;

use kotc_reqwasm::server_structs::Resource;
use yew::{function_component, html, Properties};

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
            <img class={"token"} name={ token.name.clone() } src={ token.path.clone() } />
        }
    }).collect()
}

#[derive(Clone, Properties, PartialEq)]
pub struct StatsProps {
    pub stats: HashMap<String, Vec<kotc_reqwasm::server_structs::Token>>,
}

#[function_component(Stats)]
pub fn player_stats(StatsProps { stats }: &StatsProps) -> Html {
    html! {
        <div class={"game__tokens"}>
            <p>{"Player's tokens"}</p>
            {
                for stats.iter().map(|(key, value)| html! {
                <>
                    <p>{ key.clone() }</p>
                    {
                        html! {
                            <TokenList 
                                tokens={ value.iter().map(|token| Token::new(&token.resource, token.points)).collect::<Vec<Token>>()}
                            />
                        }
                    }
                </>
                }) 
            }
        </div>
    }
}
