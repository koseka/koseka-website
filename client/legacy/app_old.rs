use crate::views::*;
use sycamore::prelude::*;
use sycamore::web::tags::*;

#[component]
fn HomeContainer() -> View {
    div().id("home-container").class("h-full flex justify-center items-center").children((
        button().id("home-view-button").attr("autofocus", "true").class("focus:ring-0 focus:outline-none items-center").children(
            img().id("koseka-logo").class("select-none w-9 h-9").src("assets/icons/favicon.svg").alt("Koseka logo.").draggable("false"),
        ),
        HomeView()
    )).into()
}

#[component]
pub(super) fn App() -> View {
    /*
    provide_context(ActiveView {
        id: "home-view",
        home_view: None,
        book_view: None,
        blog_view: None,
        wall_view: None,
        fund_view: None,
    });
    */

    view! {
        div(id="app", class="inset-0") {
            div(id="header", class="absolute content-box top-0 left-0 right-0 h-[60px] rounded-2xl m-2 flex flex-row justify-between items-center") {
                div(id="header-border", class="absolute z-0 inset-0 rounded-2xl backdrop-blur-[3px] border-[0.5px] border-slate-500/80 bg-slate-600/50") {}

                div(id="header-content-left", class="relative h-full rounded-2xl p-2 flex flex-row gap-2 justify-center items-center") {
                    HomeContainer()

                    div(id="searchbar-container", class="group hover:cursor-pointer relative h-full rounded-xl pl-2 pr-2 flex flex-row gap-2 items-center") {
                        input(id="searchbar-input", class="peer order-2 hover:cursor-pointer relative z-10 h-6 focus:ring-0 focus:outline-none font-koseka font-extralight text-lg antialiased text-slate-300/90 placeholder-slate-300/70 bg-transparent", placeholder="Search...") {}
                        div(id="searchbar-border", class="order-4 absolute inset-0 rounded-xl pulse-border", style="animation-delay: 850ms;") {}
                        img(id="searchbar-icon", class="order-1 select-none relative z-10 w-6 h-6 opacity-80 group-hover:opacity-90 peer-focus:opacity-90 transition-opacity duration-[500ms] ease-out", src="assets/icons/search.svg", alt="Search icon.", draggable="false") {}
                        div(id="searchbar-cross-icon", class="order-3 relative z-10 w-4 h-4 rounded-lg opacity-80 peer-placeholder-shown:opacity-0 transition-opacity duration-[250ms] ease-out") {
                            img(id="searchbar-cross-icon-base", class="select-none absolute inset-0 rounded-lg", src="assets/icons/cross.svg", alt="Cross icon base.", draggable="false") {}
                            img(id="searchbar-cross-icon-animator", class="select-none relative z-10 w-full h-full rounded-lg hover:animate-ping-sm", src="assets/icons/cross.svg", alt="Cross icon animator.", draggable="false") {}
                        }
                    }
                }

                div(id="header-content-right", class="relative h-full rounded-2xl p-2 flex flex-row gap-8 items-center") {
                    div(id="navbar", class="h-full rounded-xl flex flex-row gap-8 items-center") {
                        div(id="book-container") {
                            button(id="book-view-button", class="group flex flex-row gap-[3px] items-center") {
                                img(id="book-view-icon", class="view-icon", src="assets/icons/book.svg", alt="Book icon.", draggable="false") {}
                                p(id="book-view-title", class="select-none font-koseka font-[350] text-xl antialiased underline decoration-[1.5px] underline-offset-[3px] text-slate-300/90 decoration-slate-300/0 group-hover:decoration-slate-300/90") { "Book" }
                            }
                        }

                        div(id="blog-container") {
                            button(id="blog-view-button", class="group flex flex-row gap-[5px] items-center") {
                                img(id="blog-view-icon", class="view-icon", src="assets/icons/blog.svg", alt="Blog icon.", draggable="false") {}
                                p(id="blog-view-title", class="view-title") { "Blog" }
                            }
                        }

                        div(id="wall-container") {
                            button(id="wall-view-button", class="group flex flex-row gap-[4px] items-center") {
                                img(id="wall-view-icon", class="view-icon", src="assets/icons/wall.svg", alt="Wall icon.", draggable="false") {}
                                p(id="wall-view-title", class="view-title") { "Wall" }
                            }
                        }

                        div(id="fund-container") {
                            button(id="fund-view-button", class="group flex flex-row gap-[5px] items-center") {
                                img(id="fund-view-icon", class="view-icon", src="assets/icons/fund.svg", alt="Fund icon.", draggable="false") {}
                                p(id="fund-view-title", class="view-title") { "Fund" }
                            }
                        }
                    }

                    div(id="settings-container", class="h-full") {
                        button(id="settings-button", class="group relative h-full rounded-xl flex flex-row pl-2 pr-2 gap-1 items-center") {
                            div(id="settings-button-border", class="absolute inset-0 rounded-xl pulse-border", style="animation-delay: 850ms;") {}
                            img(id="settings-button-icon", class="select-none relative z-10 w-6 h-6 opacity-90", src="assets/icons/region.svg", alt="Region icon.", draggable="false") {}
                            p(id="settings-button-text", class="select-none relative z-10 text-slate-300/90 font-koseka font-[350] text-xl antialiased") { "BE" }
                        }
                    }
                }
            }
        }
    }
}
