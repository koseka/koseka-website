use sycamore::prelude::*;

#[component]
pub(crate) fn Footer() -> View {
    view! {
        div(id="footer", class="flex flex-col") {
            div(id="socials", class="flex flex-row") {
                a(href="tel:+1234567890") {
                    object(id="phone-icon", r#type="image/svg+xml", data="../assets/icons/phone.svg") {}
                }
                a(href="mailto:info@koseka.net") {
                    object(id="email-icon", r#type="image/svg+xml", data="../assets/icons/email.svg") {}
                }
                a(href="https://twitter.com/koseka") {
                    object(id="twitter-icon", r#type="image/svg+xml", data="../assets/icons/twitter.svg") {}
                }
                a(href="https://www.linkedin.com/company/koseka") {
                    object(id="linkedin-icon", r#type="image/svg+xml", data="../assets/icons/linkedin.svg") {}
                }
                a(href="https://github.com/koseka") {
                    object(id="github-icon", r#type="image/svg+xml", data="../assets/icons/github.svg") {}
                }
                a(href="https://youtube.com/@koseka") {
                    object(id="youtube-icon", r#type="image/svg+xml", data="../assets/icons/youtube.svg") {}
                }
            }
            p(id="copyright") { "© 2025 Koseka - All rights reserved." }
        }
    }
}
