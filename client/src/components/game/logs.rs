use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogsProps {
    pub logs: Vec<String>,
}

#[function_component(Logs)]
pub fn logs(LogsProps { logs }: &LogsProps) -> Html {
    html! {
        <div class={"game__logs"}>
            {
                logs.iter().map(|log| {
                    html! {
                        <p>{ log }</p>
                    }
                }).collect::<Vec<Html>>()
            }
        </div>
    }
}
