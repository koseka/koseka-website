use sycamore::prelude::*;

#[component]
pub(super) fn App() -> View {
    view! {
        main(id="app", class="min-h-screen min-h-[100dvh] flex flex-col items-center justify-center gap-8 p-6 text-center") {
            h1(id="wordmark", class="select-none font-koseka font-[350] text-3xl antialiased text-slate-300/90 tracking-wide") { "Koseka" }

            div(id="meme-card", class="w-full max-w-md rounded-2xl border-[0.5px] border-slate-500/80 bg-slate-600/30 p-2 backdrop-blur-[3px]") {
                img(id="cooking-meme", class="select-none w-full rounded-xl", src="assets/memes/cooking.gif", alt="A cartoon dog happily cooking over a flaming pan.", draggable="false") {}
            }

            div(id="tagline", class="flex flex-col gap-1") {
                p(id="tagline-main", class="select-none font-koseka font-[350] text-2xl antialiased text-slate-300/90") { "We are cooking." }
                p(id="tagline-sub", class="select-none font-koseka font-extralight text-lg antialiased text-slate-300/60") { "Come back soon." }
            }

            p(id="copyright", class="select-none font-koseka font-extralight text-sm antialiased text-slate-400/50") { "© 2026 Koseka" }
        }
    }
}
