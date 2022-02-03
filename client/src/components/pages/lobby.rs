use yew::prelude::*;
use crate::components::pages::headstone::Headstone;


#[derive(PartialEq, Debug, Clone)]
pub enum PlayerColor {
    White,
    Black,
    Green,
    Yellow,
    Blue,
    Red,
}

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let tmp_color = PlayerColor::Black;
    let tmp_username = "Username".to_string();
    let ready = false;

    html! {
         <div class="lobby" >
            <h1 class="lobby__title">{"Lobby"}</h1>
            <div class="lobby__headstones">
                <Headstone color={PlayerColor::Black} player_name={tmp_username.clone()} />
                <Headstone color={PlayerColor::White} player_name={tmp_username.clone()} />
                <Headstone color={PlayerColor::Red} player_name={tmp_username.clone()} />
                <Headstone color={PlayerColor::Blue} player_name={tmp_username.clone()} />
                <Headstone color={PlayerColor::Green} player_name={tmp_username.clone()} />
            </div>
            <button class="lobby__button" type="button">{if ready {"Not Ready"} else {"Ready"}}</button>
         </div>
    }
}
