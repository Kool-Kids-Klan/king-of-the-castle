use crate::Route;
use kotc_reqwasm::endpoints::login_user;
use kotc_reqwasm::endpoints::{LoggedUser, User};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::*;

#[derive(Clone, Debug, Default)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let history = use_history().unwrap();
    let store = use_store::<BasicStore<LoggedUser>>();

    let set_user: Callback<Option<User>> =
        { Callback::from(move |i| store.dispatch().reduce(|state| state.logged_user = i)) };

    let login_info = use_state(LoginInfo::default);

    let onsubmit = {
        let login_info = login_info.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let info = (*login_info).clone();
            let x = set_user.clone();
            login_user(info.username, info.password, x);

            history.push(Route::Home);
        })
    };

    let oninput_username = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.username = input.value();
            login_info.set(info);
        })
    };
    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    html! {
         <div class="login" >
             <form class= "form" {onsubmit}>
                 <label class="form__label" for="username">{"Username"}</label>
                 <input value={login_info.username.clone()} oninput={oninput_username} class="form__input" type="text" id="username" name="username"/>
                 <label class="form__label" for="password">{"Password"}</label>
                 <input value={login_info.password.clone()} oninput={oninput_password} class="form__input" type="Password" id="password" name="password" />
                 <button class="form__button" type="submit">{"Login"}</button>
             </form>
         </div>
    }
}
