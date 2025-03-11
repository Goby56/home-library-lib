use web_sys::window;
#[warn(non_camel_case_types)]

use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

#[function_component]
fn App() -> Html {
    let onclick = Callback::from(|_| {
        if let Some(win) = window() {
            win.location().set_href("https://qrbot.net/x-callback-url/scan?x-success=127.0.0.1:8080").unwrap();
        }
    });
    html! {
        <button {onclick}>{ "Scan" }</button>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
