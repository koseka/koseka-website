use sycamore::prelude::*;

#[component]
pub(crate) fn Header() -> View {
    view! {
        div(id="header", class="sticky z-10 content-box h-16 border-t-8 border-l-8 border-r-8 border-transparent") {
            div(id="header-content", class="w-full h-full rounded-2xl backdrop-blur-[3px] border-[0.5px] border-slate-500/80 bg-slate-600/50 flex flex-row justify-between items-center") {
                div(id="header-content-left", class="h-full rounded-2xl p-2 flex flex-row gap-2 justify-center items-center") {
                    button(id="home-view-button") {
                        img(id="koseka-logo", class="w-9 h-9", src="assets/icons/favicon.svg", alt="Koseka logo.") {}
                    }

                    div(id="searchbar", class="group hover:cursor-pointer relative h-full rounded-xl pl-2 pr-2 flex flex-row gap-2 items-center") {
                        input(id="searchbar-input", class="peer order-2 hover:cursor-pointer relative z-10 w-48 h-6 focus:ring-0 focus:outline-none font-koseka font-extralight text-lg antialiased text-slate-300/90 placeholder-slate-300/70 bg-transparent", placeholder="Search...") {}
                        img(id="searchbar-icon", class="order-1 relative z-10 w-6 h-6 opacity-80 group-hover:opacity-90 peer-focus:opacity-90 transition-opacity duration-[500ms] ease-out", src="assets/icons/search.svg", alt="Search icon.") {}
                        div(id="searchbar-cross-icon", class="order-3 relative z-10 w-4 h-4 rounded-lg opacity-80 peer-placeholder-shown:opacity-0 transition-opacity duration-[250ms] ease-out") {
                            img(id="searchbar-cross-icon-base", class="absolute inset-0 rounded-lg", src="assets/icons/cross.svg", alt="Cross icon base.") {}
                            img(id="searchbar-cross-icon-animator", class="relative z-10 w-full h-full rounded-lg hover:animate-ping-sm", src="assets/icons/cross.svg", alt="Cross icon animator.") {}
                        }
                        div(id="searchbar-border", class="order-4 absolute inset-0 rounded-xl pulse-border", style="animation-delay: 850ms;") {}
                    }
                }

                div(id="header-content-right", class="h-full rounded-2xl p-2 flex flex-row gap-8 items-center") {
                    div(id="navbar", class="h-full flex flex-row gap-8 items-center") {
                        button(id="book-view-button", class="group flex flex-row gap-[3px] items-center") {
                            img(id="book-view-icon", class="view-icon", src="assets/icons/book.svg", alt="Book icon.") {}
                            p(id="book-view-title", class="view-title") { "Book" }
                        }
                        button(id="blog-view-button", class="group flex flex-row gap-[5px] items-center") {
                            img(id="blog-view-icon", class="view-icon", src="assets/icons/blog.svg", alt="Blog icon.") {}
                            p(id="blog-view-title", class="view-title") { "Blog" }
                        }
                        button(id="wall-view-button", class="group flex flex-row gap-[4px] items-center") {
                            img(id="wall-view-icon", class="view-icon", src="assets/icons/wall.svg", alt="Wall icon.") {}
                            p(id="wall-view-title", class="view-title") { "Wall" }
                        }
                        button(id="fund-view-button", class="group flex flex-row gap-[5px] items-center") {
                            img(id="fund-view-icon", class="view-icon", src="assets/icons/fund.svg", alt="Fund icon.") {}
                            p(id="fund-view-title", class="view-title") { "Fund" }
                        }
                    }

                    button(id="settings-button", class="group relative h-full rounded-xl flex flex-row pl-2 pr-2 gap-1 items-center") {
                        img(id="settings-button-icon", class="relative z-10 w-6 h-6 opacity-90", src="assets/icons/region.svg", alt="Region icon.") {}
                        p(id="settings-button-text", class="relative z-10 text-slate-300/90 font-koseka font-[350] text-xl antialiased") { "BE" }
                        div(id="settings-button-border", class="absolute z-10 inset-0 rounded-xl pulse-border", style="animation-delay: 850ms;") {}
                    }
                }
            }
        }
    }
}
