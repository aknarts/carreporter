use crate::components::user_context_provider::UserContextProvider;
use crate::pages::footer::Footer;
use crate::pages::header::Header;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::home::Home;
use tracing::debug;
use yew::html::Html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <UserContextProvider>
            <BrowserRouter>
                <Header />
                <div class="d-flex p-2 flex-grow-1">
                    <main class="container">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
                <Footer />
            </BrowserRouter>
        </UserContextProvider>
    )
}

// Allowing because this is how yew defines the function
#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    debug!("Routing to {:?}", routes);
    match routes {
        Route::Home => html!( <Home /> ),
        Route::NotFound => html!( <PageNotFound /> ),
    }
}
