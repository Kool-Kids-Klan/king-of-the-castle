use crate::components::pages::headstone::{HeadstoneList, HeadstoneProps};
use crate::router::Route;
use gloo_storage::{SessionStorage, Storage};
use kotc_reqwasm::endpoints::{
    ColumnsStore, FinalResultsStore, GameStarted, HandStore, LogStore, PlayerOnTurnStore,
    TokenStore, User,
};
use kotc_reqwasm::server_structs::Player;
use kotc_reqwasm::{connect_websocket, send_join, send_ready, GameStateSetters, KotcWebSocket};
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

#[derive(Clone)]
pub struct KotcWebSocketState {
    pub websocket: Rc<RefCell<Option<KotcWebSocket>>>,
}

impl KotcWebSocketState {
    pub fn new(lobby_id: String, user_id: i32, setters: GameStateSetters) -> Self {
        let ws = Rc::new(RefCell::new(Some(connect_websocket(lobby_id, setters))));
        send_join(user_id, Rc::clone(&ws));

        KotcWebSocketState { websocket: ws }
    }
}

impl Default for KotcWebSocketState {
    fn default() -> Self {
        KotcWebSocketState {
            websocket: Rc::new(RefCell::new(None)),
        }
    }
}

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let ready = use_state(|| false);
    let game_started = use_store::<BasicStore<GameStarted>>();
    let columns_store = use_store::<BasicStore<ColumnsStore>>();
    let hand_store = use_store::<BasicStore<HandStore>>();
    let log_store = use_store::<BasicStore<LogStore>>();
    let token_store = use_store::<BasicStore<TokenStore>>();
    let player_on_turn_store = use_store::<BasicStore<PlayerOnTurnStore>>();
    let final_results_store = use_store::<BasicStore<FinalResultsStore>>();

    let set_started = game_started
        .dispatch()
        .reduce_callback_with(|state, i| state.game_started = i);

    let set_columns = columns_store
        .dispatch()
        .reduce_callback_with(|state, cols| state.columns = cols);

    let set_hand = hand_store
        .dispatch()
        .reduce_callback_with(|state, cards| state.hand = cards);

    let set_logs = log_store
        .dispatch()
        .reduce_callback_with(|state, log_detail| state.logs.push(log_detail));

    let set_tokens = token_store
        .dispatch()
        .reduce_callback_with(|state, players_tokens| state.tokens = players_tokens);

    let set_player_on_turn = player_on_turn_store
        .dispatch()
        .reduce_callback_with(|state, player_on_turn| state.player = Some(player_on_turn));

    let set_final_results =
        final_results_store
            .dispatch()
            .reduce_callback_with(|state, final_results| {
                state.results = final_results;
                state.game_ended = true
            });

    let lobby_id: String = SessionStorage::get("lobby_id").unwrap();
    let logged_user: User = SessionStorage::get("user").unwrap();

    let players = use_state(std::vec::Vec::new);

    let set_players: Callback<Vec<Player>> = {
        let players = players.clone();
        Callback::from(move |i: Vec<Player>| players.set(i))
    };

    log::info!("{:?}", lobby_id);
    log::info!("{:?}", logged_user);

    let setters = GameStateSetters {
        set_players,
        set_started,
        set_columns,
        set_hand,
        set_logs,
        set_tokens,
        set_player_on_turn,
        set_final_results,
    };
    let ws_store = use_store::<BasicStore<KotcWebSocketState>>();
    let is_connected = use_state(|| false);
    let ws_state;
    if !*is_connected {
        ws_state = Rc::new(KotcWebSocketState::new(lobby_id, logged_user.id, setters));
        let set_ws: Callback<Rc<RefCell<Option<KotcWebSocket>>>> = ws_store
            .dispatch()
            .reduce_callback_with(|state, websocket| {
                state.websocket = websocket;
                log::info!("SETTINGS WS STORE")
            });
        set_ws.emit(Rc::clone(&ws_state.websocket));
        is_connected.set(true);
    } else {
        ws_state = Rc::clone(ws_store.state().unwrap());
    }
    let ws = match ws_store.state() {
        Some(ws) => ws.as_ref(),
        None => ws_state.as_ref(),
    };

    let on_ready_click = {
        let ready = ready.clone();
        let ws = ws.clone();
        Callback::from(move |_e: MouseEvent| {
            let r = Rc::clone(&ws.websocket);
            send_ready(logged_user.id, r);
            ready.set(!*ready);
        })
    };

    let headstone_props = players
        .iter()
        .map(|p| HeadstoneProps {
            color: p.color.clone(),
            player_name: p.username.clone(),
            ready: p.ready,
        })
        .collect::<Vec<HeadstoneProps>>();

    match game_started.state() {
        None => {}
        Some(state) => {
            if state.game_started {
                let history = use_history().unwrap();
                history.push(Route::Game);
            }
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
