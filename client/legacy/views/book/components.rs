use sycamore::prelude::*;

#[component]
pub(super) fn BookViewSidebar() -> View {
    view! {
        div(id="book-view-sidebar", class="rounded-md") {
            div(id="book-view-sidebar-header", class="rounded-md") {
                button(id="standards-button") {
                    object(r#type="image/svg+xml", data="../assets/icons/standards.svg") {}
                    p(class="text-2xl font-mono") { "Standards" }
                }
                button(id="framework-button") {
                    object(r#type="image/svg+xml", data="../assets/icons/framework.svg") {}
                    p(class="text-2xl font-mono") { "Framework" }
                }
            }
            div(id="separator", class="rounded-md") {}
            div(id="book-view-sidebar-content", class="rounded-md") {}
        }
    }
}
