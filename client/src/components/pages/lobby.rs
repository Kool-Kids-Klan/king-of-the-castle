use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;
use kotc_reqwasm::{connect_websocket, KotcWebSocket, send_ready, send_join};
use kotc_reqwasm::endpoints::LoggedUser;
use crate::components::pages::headstone::Headstone;
use crate::components::pages::home::{LobbyState};


#[derive(PartialEq, Debug, Clone)]
pub enum PlayerColor {
    White,
    Black,
    Green,
    Yellow,
    Blue,
    Red,
}

#[derive(Clone)]
pub struct KotcWebSocketState {
    websocket: Rc<RefCell<KotcWebSocket>>,
}

impl KotcWebSocketState {
    pub fn new(lobby_id: String, user_id: i32) -> Self {
        let ws = Rc::new(RefCell::new(connect_websocket(lobby_id)));
        send_join(user_id, Rc::clone(&ws));

        KotcWebSocketState {
            websocket: ws,
        }
    }
}

impl Default for KotcWebSocketState {
    fn default() -> Self {
        KotcWebSocketState {
            websocket: Rc::new(RefCell::new(connect_websocket(String::from("1234")))),
        }
    }
}

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let tmp_color = PlayerColor::Black;
    let tmp_username = "Username".to_string();
    let ready = false;

    let store = use_store::<BasicStore<LoggedUser>>();
    let lobby_info_store = use_store::<BasicStore<LobbyState>>();
    let mut lobby_id = String::from("1234");
    if let Some(lobby_info) = lobby_info_store.state().map(|s| s.lobby_id.to_string()) {
        lobby_id = lobby_info;
    }

    log::info!("{:?}", lobby_id);
    log::info!("{:?}", store.state());
    
    let user_id = store.state().map(|s| s.logged_user.as_ref()).unwrap_or_default().unwrap().id;
    let ws = use_state(|| KotcWebSocketState::new(lobby_id, user_id));

    let on_ready_click = {
        Callback::from(move |_e: MouseEvent| {
            let id = store.state().map(|s| s.logged_user.as_ref()).unwrap_or_default().unwrap().id;
            let r = Rc::clone(&ws.websocket);
            send_ready(id, r);
        })
    };

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
            <button class="lobby__button" type="button" onclick={on_ready_click}>{if ready {"Not Ready"} else {"Ready"}}</button>
         </div>
    }
}
