use sycamore::prelude::*;

#[component]
pub(super) fn JoinWaitlist() -> View {
    view! {
        div(id="join-waitlist", class="flex flex-row items-center justify-center") {
            div(class="rounded-lg") {
                input(id="email-input", class="font-mono", r#type="text", placeholder="Email address...") {}
            }
            button(id="join-button", class="bg-purple-500 rounded-md outline-1 outline-purple-500 hover:outline-purple-600 focus:outline-purple-700 ring-1", r#type="submit") { "Join waitlist" }
        }
    }
}

#[component]
pub(super) fn Meme() -> View {
    view! {
        img(src="../assets/meme.gif", alt="Meme image.", class="rounded-md") {}
    }
}
