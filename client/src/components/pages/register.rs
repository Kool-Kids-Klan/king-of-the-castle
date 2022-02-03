use yew::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
         <div class="register" >
             <form class="form">
                 <label class="form__label" for="username">{"Username:"}</label>
                 <input class="form__input" type="text" id="username" name="username"/>
                 <label class="form__label" for="email">{"Email:"}</label>
                 <input class="form__input" type="text" id="email" name="email"/>
                 <label class="form__label" for="password">{"Password:"}</label>
                 <input class="form__input" type="Password" id="password" name="password" />
                 <button class="form__button" type="submit">{"Register"}</button>
             </form>
         </div>
    }
}
