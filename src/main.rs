use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "Hello, Dioxus!" }
        h2 { "New header 2!" }
    }
}
