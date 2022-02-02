use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    return html! {
    <div class="login" >
        <form class= "form">
            <label class="form__label" for="username">{"Username:"}</label>
            <input class="form__input" type="text" id="username" name="username"/>
            <label class="form__label" for="password">{"Password:"}</label>
            <input class="form__input" type="Password" id="password" name="password" />
            <button class="form__button" type="submit">{"Login"}</button>
        </form>
    </div>
   }
}
