use crate::homepage::Homepage;
use crate::orderlistitem::*;
use crate::BASE_URL;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/order/:id")]
    OrderListEntry { id: u32 },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Homepage base_url={BASE_URL} /> },
        Route::OrderListEntry { id } => html! {
            <OrderListItem id={id} />
        },
    }
}

#[function_component(Router)]
pub fn app() -> Html {
    html! {
        <div style="font-family: monospace">
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </div>
    }
}