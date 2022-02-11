use crate::Route;
use kotc_reqwasm::server_structs::Color;
use std::collections::HashMap;
use yew::{classes, function_component, html, Callback, Html, Properties};
use yew_router::history::History;
use yew_router::hooks::use_history;

#[derive(Properties, PartialEq)]
pub struct EndPopupProps {
    pub results: HashMap<String, (Color, u8)>,
}

#[function_component(EndPopup)]
pub fn end_popup(props: &EndPopupProps) -> Html {
    let history = use_history().unwrap();

    let home_button = {
        let history = history;
        let onclick = Callback::once(move |_| history.push(Route::Home));
        html! {
            <button class="end-popup__button" {onclick} >{"Back home"}</button>
        }
    };

    let mut results: Vec<(String, (Color, u8))> = props.results.clone().into_iter().collect();
    results.sort_by(|(_, (_, score_1)), (_, (_, score_2))| score_2.cmp(score_1));
    let rows = results
        .iter()
        .enumerate()
        .map(|(order, (name, (color, points)))| {
            let color_class = format!("player--{:?}", color).to_lowercase();
            let first_str = format!("{}. ", order + 1);
            let second_str = format!(" with score: {}", points);
            html! {
                <span class="end-popup__row">
                    {first_str}
                    <span class={classes!(color_class)}>
                        {name.clone()}
                    </span>
                        {second_str}
                </span>
            }
        })
        .collect::<Html>();

    html! {
        <div class="end-popup">
            <h2 class="end-popup__title">{"Final standings"}</h2>
            <div class="end-popup__rows">
                {rows}
            </div>
            {home_button}
        </div>
    }
}
