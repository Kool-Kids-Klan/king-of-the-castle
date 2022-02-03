use yew::prelude::*;
use crate::components::pages::lobby::PlayerColor;

#[derive(Properties, PartialEq)]
pub struct HeadstoneProps {
    pub color: PlayerColor,
    pub player_name: String,
}

#[function_component(Headstone)]
pub fn headstone(props: &HeadstoneProps) -> Html {
    let img_src = match props.color {
        PlayerColor::White => "../../../assets/cards/backsides/backside_white.png",
        PlayerColor::Black => "../../../assets/cards/backsides/backside_black.png",
        PlayerColor::Green => "client/assets/cards/backsides/backside_green.png",
        PlayerColor::Yellow => "client/assets/cards/backsides/backside_yellow.png",
        PlayerColor::Blue => "client/assets/cards/backsides/backside_blue.png",
        PlayerColor::Red => "client/assets/cards/backsides/backside_red.png",
    };

    html! {
         <div class="headstone" >
            <h2 class="headstone_name">{props.player_name.clone()}</h2>
            <img class="headstone__img" src={img_src} alt="player headstone" />
            <span>{"Ready"}</span>
        </div>
    }
}