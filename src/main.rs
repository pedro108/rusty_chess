use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 1 is dark square, 0 is a light square
    let board: [i32; 8] = [1, 0, 1, 0, 1, 0, 1, 0];

    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }

        h1 { class: "text-2xl font-bold", "Hello, Dioxus!" }

        for i in board {
            div { class: "w-16 h-16", "test {i}" }
        }
    }
}
