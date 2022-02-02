use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    return html! {
    <div class="not-found" >
       <h1 class="not-found__title">{"Not Found"}</h1>
        <span class="not-found__subscription">{"The page you are looking for does not exist."}</span>
    </div>
   }
}
