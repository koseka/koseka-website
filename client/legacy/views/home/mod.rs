mod components;

use crate::Footer;
use components::{ JoinWaitlist, Meme };
use sycamore::prelude::*;
#[component]
pub(crate) fn HomeView() -> View {
    view! {
        div(id="home-view", class="fixed -z-10 inset-0 pt-[72px] pl-2") {
            // h(class="font-koseka antialiased"){ "Pushing technology towards Web3" }
        }
    }
}
