use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement};
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;
use kotc_reqwasm::endpoints::LoggedUser;
use kotc_commons::messages::{ClientWsMessage, Error};
use kotc_commons::messages::message_types::ClientWsMessageType;
use kotc_reqwasm::{connect_websocket, KotcWebSocket};
use crate::Route;

#[derive(Clone, Debug, Default)]
pub struct LobbyState {
    pub lobby_id: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let store = use_store::<BasicStore<LoggedUser>>();
    let lobby_info = use_state(LobbyState::default);
    let mut logged = false;
    if let Some(user) = store.state().map(|s| s.logged_user.as_ref()).unwrap_or_default() {
        logged = true;
    }

    // let store = use_store::<BasicStore<KotcWebSocketState>>();
    // if let Some(ws) = store.state().map(|s| Rc::clone(&s.websocket)) {
    //     let error = Error {
    //         detail: format!("test error"),
    //     };
    //     let error_serialized = serde_json::to_string(&error).unwrap();
    //     spawn_local(async move {
    //         Rc::clone(&ws).borrow_mut().send_message(ClientWsMessage {
    //             message_type: ClientWsMessageType::Error,
    //             content: error_serialized,
    //         }).await;
    //     })
    // };

    let onsubmit = {
        let lobby_info = lobby_info.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let info = (*lobby_info).clone();
            history.push(Route::Lobby);
        })
    };

    let oninput_lobby_id = {
        let lobby_info = lobby_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*lobby_info).clone();
            info.lobby_id = input.value();
            lobby_info.set(info);
        })
    };

    return html! {
         <div class="home">
            <h1 class="home__title">{"King of the Castle"}</h1>
             <span class="home__subscription">{"The browser variant of Královské rošády"}</span>
             {
                 if !logged {
                     html! {
                         <span class="home__subscription">{"Please login to continue..."}</span>
                     }
             } else {
                 html! {
                     <form class="form" {onsubmit}>
                         <label class="form__label center-aligned" for="lobby">{"Join/Create room:"}</label>
                         <input class="form__input" type="text" id="lobby" name="lobby" oninput={oninput_lobby_id}/>
                         <button class="form__button center-aligned" type="submit">{"Enter"}</button>
                     </form>
                 }
             }}
         </div>
    }
}
