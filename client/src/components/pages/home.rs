use crate::Route;
use gloo_storage::{SessionStorage, Storage};
use kotc_reqwasm::endpoints::LoggedUser;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

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
    if store
        .state()
        .map(|s| s.logged_user.as_ref())
        .unwrap_or_default()
        .is_some()
    {
        logged = true;
    }

    let onsubmit = {
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            history.push(Route::Lobby);
        })
    };

    let oninput_lobby_id = {
        let lobby_info = lobby_info;
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*lobby_info).clone();
            info.lobby_id = input.value();
            SessionStorage::set("lobby_id", info.lobby_id.clone()).unwrap();
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
                         <input class="form__input" pattern="[0-9]{3,}" title="Lobby must be at least 3 long numeric." type="text" id="lobby" name="lobby" oninput={oninput_lobby_id}/>
                         <button class="form__button center-aligned" type="submit">{"Enter"}</button>
                     </form>
                 }
             }}
         </div>
    };
}
