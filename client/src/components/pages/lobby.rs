use std::cell::RefCell;
use std::rc::Rc;
use gloo_storage::{SessionStorage, Storage};
use kotc_reqwasm::server_structs::{Player, Column, Card};
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yewdux::prelude::{BasicStore, WithDispatchProps, Dispatcher};
use yewdux_functional::use_store;
use kotc_reqwasm::{connect_websocket, KotcWebSocket, send_ready, send_join, GameStateSetters};
use kotc_reqwasm::endpoints::{LoggedUser, User, GameStarted, ColumnsStore, HandStore};
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
    pub fn new(lobby_id: String, user_id: i32, set_players: Callback<Vec<Player>>, set_started: Callback<bool>, set_columns: Callback<Vec<Column>>, set_hand: Callback<Vec<Card>>) -> Self {
        let setters = GameStateSetters { set_players, set_started, set_columns, set_hand };
        let ws = Rc::new(RefCell::new(connect_websocket(lobby_id, setters)));
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
                GameStateSetters {
                    set_players: Callback::from(|_| print!("")),
                    set_started: Callback::from(|_| print!("")),
                    set_columns: Callback::from(|_| print!("")),
                    set_hand: Callback::from(|_| print!("")),
                }
            ))),
        }
    }
}

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let ready = use_state(|| false);
    let game_started = use_store::<BasicStore<GameStarted>>();
    let columns_store = use_store::<BasicStore<ColumnsStore>>();
    let hand_store = use_store::<BasicStore<HandStore>>();

    let set_started = 
        game_started.dispatch().reduce_callback_with(|state, i| state.game_started = i);

    let set_columns = 
        columns_store.dispatch().reduce_callback_with(|state, cols| state.columns = cols);

    let set_hand = 
        hand_store.dispatch().reduce_callback_with(|state, cards| state.hand = cards);

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
    let ws = use_state(|| KotcWebSocketState::new(lobby_id, logged_user.id, set_players, set_started, set_columns, set_hand));

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

    match game_started.state() {
        None => {},
        Some(state) => if state.game_started {
            let history = use_history().unwrap();
            history.push(Route::Game);
        }
    }

    html! {
         <div class="lobby" >
            <h1 class="lobby__title">{"Lobby"}</h1>
            <HeadstoneList players={headstone_props} />
            <button class="lobby__button" type="button" onclick={on_ready_click}>{if *ready {"Unready"} else {"Ready"}}</button>
         </div>
    }
}
