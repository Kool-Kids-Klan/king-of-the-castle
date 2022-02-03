use yew::prelude::*;
use web_sys::HtmlInputElement;


#[derive(Clone, Debug, Default)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[function_component(Login)]
pub fn login() -> Html {

    // let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);
    // let user_login = {
    //     let login_info = login_info.clone();
    //     use_async(async move {
    //         let request = LoginInfoWrapper {
    //             user: (*login_info).clone(),
    //         };
    //         login(request).await
    //     })
    // };

    // use_effect_with_deps(
    //     move |user_login| {
    //         if let Some(user_info) = &user_login.data {
    //             user_ctx.login(user_info.user.clone());
    //         }
    //         || ()
    //     },
    //     user_login.clone(),
    // );

    // let onsubmit = {
    //     let user_login = user_login.clone();
    //     Callback::from(move |e: FocusEvent| {
    //         e.prevent_default(); /* Prevent event propagation */
    //         let user_login = user_login.clone();
    //         user_login.run();
    //     })
    // };

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
             <form class= "form">
                 <label class="form__label" for="username">{"Username"}</label>
                 <input value={login_info.username.clone()} oninput={oninput_username} class="form__input" type="text" id="username" name="username"/>
                 <label class="form__label" for="password">{"Password"}</label>
                 <input value={login_info.password.clone()} oninput={oninput_password} class="form__input" type="Password" id="password" name="password" />
                 <button class="form__button" type="submit">{"Login"}</button>
             </form>
         </div>
    }
}
