use std::collections::HashMap;
use std::ops::Deref;
use yew::{html, Html, function_component, classes, Properties};
use kotc_reqwasm::server_structs::Color;

#[derive(Properties, PartialEq)]
pub struct EndPopupProps {
    pub results: HashMap<String, (Color, u8)>,
}

#[function_component(EndPopup)]
pub fn end_popup(props: &EndPopupProps) -> Html {
    let mut results: Vec<(String, (Color, u8))> = props.results.clone().into_iter().collect();
    results.sort_by(|a, b| a.deref().1.1.cmp(&b.deref().1.1));
    let rows = results.iter().enumerate().map(|(order, (name, (color, points)))| {
        let color_class = format!("player--{:?}", color).to_lowercase();
        let first_str = format!("{}. Player: ", order);
        let second_str = format!("Points: {}", points);
        html! {
            <span class="end-popup__row">
                {first_str}
                <span class={classes!(color_class)}>
                    {name.clone()}
                </span>
                    {second_str}
            </span>
        }
    }).collect::<Html>();
    html! {
        <div class="end-popup">
            <h2 class="end-popup__title">{"Final standings"}</h2>
            {rows}
        </div>
    }
}
