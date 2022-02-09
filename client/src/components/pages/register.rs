use kotc_reqwasm::endpoints::register_user;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::router::Route;

#[derive(Clone, Debug, Default)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[function_component(Register)]
pub fn register() -> Html {
    let history = use_history().unwrap();

    let register_info = use_state(RegisterInfo::default);

    let onsubmit = {
        let register_info = register_info.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let info = (*register_info).clone();
            register_user(info.username, info.email, info.password);
            history.push(Route::Home);
        })
    };

    let oninput_username = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info);
        })
    };

    let oninput_email = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info);
        })
    };

    let oninput_password = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info);
        })
    };

    html! {
         <div class="register" >
             <form class="form" {onsubmit} >
                 <label class="form__label" for="username">{"Username:"}</label>
                 <input value={register_info.username.clone()} oninput={oninput_username} class="form__input" type="text" id="username" name="username"/>
                 <label class="form__label" for="email">{"Email:"}</label>
                 <input value={register_info.email.clone()} oninput={oninput_email} class="form__input" type="email" id="email" name="email"/>
                 <label class="form__label" for="password">{"Password:"}</label>
                 <input value={register_info.password.clone()} oninput={oninput_password} class="form__input" type="Password" id="password" name="password" />
                 <button class="form__button" type="submit">{"Register"}</button>
             </form>
         </div>
    }
}
