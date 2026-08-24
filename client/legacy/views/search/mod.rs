mod components;

use crate::Footer;
use sycamore::prelude::*;

#[component]
pub(crate) fn SearchView() -> View {
    view! {
        div(id="search-view", class="view-frame") {
            div(id="search-view-body") {}
            Footer()
        }
    }
}
