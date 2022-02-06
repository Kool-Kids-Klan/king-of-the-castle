use yew::prelude::*;
use crate::components::pages::lobby::PlayerColor;

#[derive(Properties, PartialEq)]
pub struct HeadstoneProps {
    pub color: PlayerColor,
    pub player_name: String,
    pub ready: bool,
}

#[function_component(Headstone)]
pub fn headstone(props: &HeadstoneProps) -> Html {
    let img_src = match props.color {
        PlayerColor::White => "../../../assets/cards/characters/unknown/unknown_white.png",
        PlayerColor::Black => "../../../assets/cards/characters/unknown/unknown_black.png",
        PlayerColor::Green => "../../../assets/cards/characters/unknown/unknown_green.png",
        PlayerColor::Yellow => "../../../assets/cards/characters/unknown/unknown_yellow.png",
        PlayerColor::Blue => "../../../assets/cards/characters/unknown/unknown_blue.png",
        PlayerColor::Red => "../../../assets/cards/characters/unknown/unknown_red.png",
    };

    html! {
         <div class="headstone" >
            <span class="headstone__name">{props.player_name.clone()}</span>
            <img class="headstone__img" src={img_src} alt="player headstone" />
            <span>{if props.ready {"Ready"} else {"Unready"}}</span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct HeadstoneListProps {
    pub players: Vec<HeadstoneProps>,
}

#[function_component(HeadstoneList)]
pub fn headstone_list(HeadstoneListProps { players }: &HeadstoneListProps) -> Html {
    html! {
        <div class="lobby__headstones">
        {
            players.iter().map(|headstone| {
                html! {
                    <Headstone color={PlayerColor::Black} player_name={headstone.player_name.clone()} ready={headstone.ready} />
                }
            }).collect::<Vec<Html>>()
        }
        </div>
    }
}
