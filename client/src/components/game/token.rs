use std::collections::HashMap;

use kotc_reqwasm::server_structs::{Color, Resource};
use yew::{function_component, html, Properties, classes};

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
    tokens
        .iter()
        .map(|token| {
            html! {
                <img class={"token"} name={ token.name.clone() } src={ token.path.clone() } />
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
pub struct StatsProps {
    pub stats: HashMap<String, (Color, Vec<kotc_reqwasm::server_structs::Token>)>,
}

#[function_component(Stats)]
pub fn player_stats(StatsProps { stats }: &StatsProps) -> Html {
    html! {
        <div class={"players-stats"}>
            <span class="players-stats__title">{"Player's tokens"}</span>
            {
                for stats.iter().map(|(name, (color, value))| {
                let color_class = format!("player-tokens__name--{:?}", color).to_lowercase();
                html! {
                    <div class="player-tokens">
                        <span class={classes!("player-tokens__name", color_class.clone())}>{ name.clone() }</span>
                        {
                            html! {
                                <TokenList
                                    tokens={ value.iter().map(|token| Token::new(&token.resource, token.points)).collect::<Vec<Token>>()}
                                />
                            }
                        }
                    </div>
                }
            })
            }
        </div>
    }
}
