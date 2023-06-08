use yew::prelude::*;
use crate::pages::report::Report;

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <Report />
                </div>
            </div>
        </div>
    )
}