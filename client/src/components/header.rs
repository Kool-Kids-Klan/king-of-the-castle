use gloo_storage::{SessionStorage, Storage};
use crate::{LoggedUser, Route};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::*;

#[function_component(Header)]
pub fn header() -> Html {
    let store = use_store::<BasicStore<LoggedUser>>();
    let mut logged = false;
    let mut username = String::new();
    let user = store.state().map(|s| s.logged_user.as_ref()).unwrap_or_default();
    match user {
        None => {}
        Some(u) => {logged = true; username = u.username.clone()}
    }

    let history = use_history().unwrap();
    let home_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::Home));
        html! {
            <button class="header__link header__link--left-aligned" {onclick}>{"Home"}</button>
        }
    };

    let login_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::Login));
        html! {
            <button class="header__link" {onclick}>{"Login"}</button>
        }
    };
    let logout_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| {
            SessionStorage::clear();
            store.dispatch().reduce(|s| s.logged_user = None);;
            history.push(Route::Home)});
        html! {
            <button class="header__link" {onclick}>{"Logout"}</button>
        }
    };

    let register_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::Register));
        html! {
            <button class="header__link" {onclick}>{"Register"}</button>
        }
    };

    let logged_links = {
        html! {
            <>
                <span class="header__username">{username}</span>
                { logout_button }
            </>
        }
    };

    let logged_out_links = {
        html! {
            <>
                { login_button }
                { register_button }
            </>
        }
    };
    html! {
        <div class="header">
            { home_button }
            {
                if logged {
                    { logged_links }
                 } else {
                    { logged_out_links }
                 }
             }
         </div>
    }
}
