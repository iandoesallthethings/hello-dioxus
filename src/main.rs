#![allow(non_snake_case, non_upper_case_globals)]
mod prelude;
use prelude::*;
mod routes;
use routes::*;
mod components;

const tailwind_styles: &str = include_str!("../dist/output.css");

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style {
            tailwind_styles
        }
        Routes {}
    })
}
