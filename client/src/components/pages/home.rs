use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;
use crate::LoggedUser;

#[function_component(Home)]
pub fn home() -> Html {
    let store = use_store::<BasicStore<LoggedUser>>();
    let mut logged = false;
    if let Some(user) = store.state().map(|s| s.logged_user.as_ref()).unwrap_or_default() {
        logged = true;
    }

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
                     <form class="form">
                         <label class="form__label center-aligned" for="lobby">{"Join/Create room:"}</label>
                         <input class="form__input" type="text" id="lobby" name="lobby"/>
                         <button class="form__button center-aligned" type="submit">{"Enter"}</button>
                     </form>
                 }
             }}
         </div>
    }
}
