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
        PlayerColor::Green => "../../../assets/cards/backsides/backside_green.png",
        PlayerColor::Yellow => "../../../assets/cards/backsides/backside_yellow.png",
        PlayerColor::Blue => "../../../assets/cards/backsides/backside_blue.png",
        PlayerColor::Red => "../../../assets/cards/backsides/backside_red.png",
    };

    html! {
         <div class="headstone" >
            <span class="headstone__name">{props.player_name.clone()}</span>
            <img class="headstone__img" src={img_src} alt="player headstone" />
            <span>{"Ready"}</span>
        </div>
    }
}