mod components;

use crate::Footer;
use sycamore::prelude::*;

#[component]
pub(crate) fn FundView() -> View {
    view! {
        div(id="fund-view", class="view-frame") {
            div(id="fund-view-body") {}
            Footer()
        }
    }
}
