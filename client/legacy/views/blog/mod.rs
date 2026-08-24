mod components;

use crate::Footer;
use sycamore::prelude::*;
/*
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
*/

#[component]
pub(crate) fn BlogView() -> View {
    /*
    let this_node_ref = create_node_ref();
    on_mount(move || {
        use_context::<ActiveView>().blog_view = Some(this_node_ref.get().dyn_into::<HtmlElement>().expect("Failed to cast 'blog-view' node to HtmlElement."));
    });
    */

    view! {
        div(/* r#ref=this_node_ref, */ id="blog-view", class="view-frame") {
            div(id="blog-view-body") {}
            Footer()
        }
    }
}
