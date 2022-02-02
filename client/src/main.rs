use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div>{"balasldfas"}</div>
            <div class={"g-signin2"} data-onsuccess={"onSignIn"}></div>
        </>
    }
}

// pub struct App {
//     pub value: i64,
// }

// impl Component for App {
//     type Message = ();
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self { value: 0 }
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         let div = gloo_utils::document().create_element("div").unwrap();
//         div.set_inner_html("
//                 <!DOCTYPE html>
//         <html lang='en'>
//         <head>
//             <meta charset='UTF-8'>
//             <title>KOTC</title>
//             <script src='https://apis.google.com/js/platform.js' async defer></script>
//             <meta name='google-signin-client_id' content='229405536082-o0p730oresk0eeprtm1j9p27523thc47.apps.googleusercontent.com'>
//         </head>
//         <body>
//             <div class='g-signin2' data-onsuccess='onSignIn'></div>
//         </body>
//         </html>

//         ");
//         // See <https://github.com/yewstack/yew/issues/1546>
//         // console::log_1(&div);

//         Html::VRef(div.into())
//     }
// }

fn main() {
    yew::start_app::<App>();
}