mod components;

use crate::Footer;
use sycamore::prelude::*;

#[component]
pub(crate) fn SettingsView() -> View {
    view! {
        div(id="settings-view", class="view-frame") {
            div(id="settings-view-body") {}
            Footer()
        }
    }
}
