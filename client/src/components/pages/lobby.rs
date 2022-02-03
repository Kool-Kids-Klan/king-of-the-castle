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

    html! {
         <div class="lobby" >
            <h1>{"Lobby"}</h1>
            <div class="lobby__headstones">
                <Headstone color={tmp_color.clone()} player_name={tmp_username.clone()} />
                <Headstone color={tmp_color.clone()} player_name={tmp_username.clone()} />
                <Headstone color={tmp_color.clone()} player_name={tmp_username.clone()} />
                <Headstone color={tmp_color.clone()} player_name={tmp_username.clone()} />
                <Headstone color={tmp_color.clone()} player_name={tmp_username.clone()} />
            </div>
         </div>
    }
}
