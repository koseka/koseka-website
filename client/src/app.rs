use sycamore::prelude::*;

// The mark is inlined so it inherits the text color (black on light, white on dark) via `currentColor`.
const MARK: &str = include_str!("../assets/logo/koseka-mark.svg");

#[component]
pub(super) fn App() -> View {
    view! {
        main(id="app", class="min-h-screen min-h-[100dvh] flex flex-col items-center justify-center gap-12 px-6 py-16 text-center") {
            // The mark, then the name as text in Koseka Sans — proportions of the vertical lockup.
            div(id="lockup", class="flex flex-col items-center text-[clamp(56px,20vw,96px)]") {
                div(id="mark", class="select-none w-[1.66em] [&>svg]:block [&>svg]:w-full [&>svg]:h-auto", dangerously_set_inner_html=MARK) {}
                p(id="wordmark", class="select-none mt-[0.14em] text-[1em] leading-none tracking-[-0.075em]") { "koseka" }
            }

            div(id="tagline", class="select-none flex flex-col gap-1 text-base") {
                p(class="text-black/70 dark:text-white/70") { "We're cooking." }
                p(class="text-black/40 dark:text-white/40") { "Come back later." }
            }

            p(id="copyright", class="select-none fixed bottom-6 inset-x-0 text-xs text-black/40 dark:text-white/40") { "© 2026 Koseka" }
        }
    }
}
