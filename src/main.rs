#![allow(non_snake_case)]
mod prelude;
use prelude::*;
mod routes;
use routes::*;
mod components;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Routes {}
    })
}
