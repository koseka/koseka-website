mod components;

use crate::Footer;
use components::BookViewSidebar;
use sycamore::prelude::*;

#[component]
pub(crate) fn BookView() -> View {
    view! {
        div(id="book-view", class="view-frame") {
            BookViewSidebar()
            div(id="book-view-body") {}
            div(id="book-view-content-table") {}
            Footer()
        }
    }
}

/*
use std::fs;
use comrak::{markdown_to_html, Options};

fn main() {
    let standards_md_path = "../assets/STANDARDS.md";
    let standard_html_path = "dist/STANDARD.html";
    let standard_md_options = Options::default();
    let standards_md = fs::read_to_string(standards_md_path).expect(format!("Failed to read {}.", standards_md_path).as_str());
    let standards_html = markdown_to_html(&standards_md, &standard_md_options);
    fs::write(standard_html_path, standards_html).expect(format!("Failed to write to {}.", standard_html_path).as_str());
}
 */
