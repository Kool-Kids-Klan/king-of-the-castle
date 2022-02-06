use std::cell::RefCell;
use std::rc::Rc;
use gloo_storage::{SessionStorage, Storage};
use kotc_reqwasm::server_structs::Player;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;
use kotc_reqwasm::{connect_websocket, KotcWebSocket, send_ready, send_join};
use kotc_reqwasm::endpoints::{LoggedUser, User};
use crate::components::pages::headstone::{HeadstoneList, HeadstoneProps};
use crate::components::pages::home::{LobbyState};
use crate::router::Route;


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
    pub fn new(lobby_id: String, user_id: i32, set_players: Callback<Vec<Player>>, set_started: Callback<bool>) -> Self {
        let ws = Rc::new(RefCell::new(connect_websocket(lobby_id, set_players, set_started)));
        send_join(user_id, Rc::clone(&ws));

        KotcWebSocketState {
            websocket: ws,
        }
    }
}

impl Default for KotcWebSocketState {
    fn default() -> Self {
        KotcWebSocketState {
            websocket: Rc::new(RefCell::new(connect_websocket(
                String::from("1234"), 
                Callback::from(|_| print!("")),
                Callback::from(|_| print!("")),
            ))),
        }
    }
}

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let ready = use_state(|| false);
    let game_started = use_state(|| false);

    let set_started = {
        let started = game_started.clone();
        Callback::from(move |is_started| {
            started.set(is_started);
        })
    };

    // let store = use_store::<BasicStore<LoggedUser>>();
    // let lobby_info_store = use_store::<BasicStore<LobbyState>>();
    let lobby_id = SessionStorage::get("lobby_id").unwrap();
    let logged_user: User = SessionStorage::get("user").unwrap();
    // if let Some(lobby_info) = lobby_info_store.state().map(|s| s.lobby_id.to_string()) {
    //     lobby_id = lobby_info;
    // }

    let players = use_state(|| vec![]);

    let set_players: Callback<Vec<Player>> = {
        let players = players.clone();
        Callback::from(move |i: Vec<Player>| players.set(i))
    };

    log::info!("{:?}", lobby_id);
    // log::info!("{:?}", store.state());
    log::info!("{:?}", logged_user);
    
    // let user_id = store.state().map(|s| s.logged_user.as_ref()).unwrap_or_default().unwrap().id;
    let ws = use_state(|| KotcWebSocketState::new(lobby_id, logged_user.id, set_players, set_started));

    let on_ready_click = {
        let ready = ready.clone();
        Callback::from(move |_e: MouseEvent| {
            // let id = store.state().map(|s| s.logged_user.as_ref()).unwrap_or_default().unwrap().id;
            let r = Rc::clone(&ws.websocket);
            send_ready(logged_user.id, r);
            ready.set(!*ready);
        })
    };

    let headstone_props = players.iter().map(|p| HeadstoneProps {
        color: PlayerColor::Black,
        player_name: p.username.clone(),
        ready: p.ready,
    }).collect::<Vec<HeadstoneProps>>();

    if *game_started {
        let history = use_history().unwrap();
        history.push(Route::Game);
    }

    html! {
         <div class="lobby" >
            <h1 class="lobby__title">{"Lobby"}</h1>
            <HeadstoneList players={headstone_props} />
            <button class="lobby__button" type="button" onclick={on_ready_click}>{if *ready {"Unready"} else {"Ready"}}</button>
         </div>
    }
}
