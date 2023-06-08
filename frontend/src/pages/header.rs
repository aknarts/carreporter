use crate::app::Route;
use crate::error::Error;
use crate::hooks::use_user_context;
use crate::types::auth::ApiResult;
use std::collections::HashMap;
use yew::prelude::*;
use yew_hooks::{use_async, use_click_away, UseAsyncHandle};
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let active = use_state(|| false);
    let resent = use_state(|| true);
    let dropdown = use_state(|| false);
    let location = use_location().unwrap();
    let route = Route::from_path(location.path(), &HashMap::new());

    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };
    let activated = *active;

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    html!(
        <>
            <nav class="navbar navbar-expand-lg sticky-top shadow bg-light" aria-label="main navigation">
                <div class="container-fluid">
                    <Link<Route> to={Route::Home} classes="navbar-brand fs-2">
                        { "Invalid Parking" }
                    </Link<Route>>
                    <button class={classes!("navbar-toggler", active_class.1)} type="button" {onclick} aria-controls="navbarSupportedContent" aria-expanded={(!activated).to_string()} aria-label="Toggle navigation">
                      <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class={classes!("collapse","navbar-collapse", active_class.0)} id="navbarSupportedContent">
                    </div>
                </div>
            </nav>
        </>
    )
}