use web_sys::{window, UrlSearchParams};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/item/:isbn")]
    Item { isbn: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    let onclick = Callback::from(|_| {
        if let Some(win) = window() {
            win.location()
                .set_href("https://qrbot.net/x-callback-url/scan?x-success=http://192.168.1.223:9090")
                .unwrap();
        }
    });

    let get_isbn_from_query = || -> Option<String> {
        if let Some(window) = window() {
            let search = window.location().search().unwrap_or_default();
            if !search.is_empty() {
                let params = UrlSearchParams::new_with_str(&search).unwrap();
                return params.get("content");
            }
        }
        return None;
    };

    html! {
        <div>
            <button {onclick}>{ "Scan" }</button>
            <p>{ format!("Scanned content: {}", get_isbn_from_query().unwrap_or(String::new())) }</p>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Item { isbn } => html! {<p>{format!("QR Bot gave callback with {}", isbn)}</p>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting web app...");
    yew::Renderer::<Main>::new().render();
}
