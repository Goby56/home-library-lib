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

    let search_input = use_state(|| String::new());

    let on_search_input = {
        let search_input = search_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                search_input.set(input.value());
            }
        })
    };

    let on_scan_click = Callback::from(|_| {
        if let Some(win) = window() {
            win.location()
                .set_href("https://qrbot.net/x-callback-url/scan?x-success=http://192.168.1.223:9090")
                .unwrap();
        }
    });

    html! {
        <div class="p-4 max-w-screen-md mx-auto">
            <h1 class="text-2xl font-bold mb-4">{"My Home Library"}</h1>
            <input
                type="text"
                placeholder="Search for books..."
                value={(*search_input).clone()}
                oninput={on_search_input}
                class="w-full p-2 border rounded mb-4"
            />
            <button
                onclick={on_scan_click}
                class="bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded mb-4"
            >
                {"Scan Book"}
            </button>
            <h2 class="text-xl font-semibold mb-2">{"Currently Borrowed"}</h2>
            <ul class="space-y-2">
                // Dummy data for now, you can replace with actual fetched items
                <li class="p-2 border rounded">{"The Hobbit by J.R.R. Tolkien"}</li>
                <li class="p-2 border rounded">{"1984 by George Orwell"}</li>
                <li class="p-2 border rounded">{"Dune by Frank Herbert"}</li>
            </ul>
            <p>{ get_isbn_from_query().unwrap_or(String::from("No book scanned")) }</p>
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
