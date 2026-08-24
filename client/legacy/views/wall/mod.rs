mod components;

use crate::Footer;
use sycamore::prelude::*;

#[component]
pub(crate) fn WallView() -> View {
    view! {
        div(id="wall-view", class="view-frame") {
            div(id="wall-view-body") {}
            Footer()
        }
    }
}
