use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {

    let logged = true;

    return html! {
    <div class="home">
       <h1 class="home__title">{"King of the Castle"}</h1>
        <span class="home__subscription">{"The browser variant of Královské rošády"}</span>
        {if logged {
            html! {<span class="home__subscription">{"Please login to continue..."}</span>}
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
