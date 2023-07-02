#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Nav {},
            Route { to: "/", Home {}}
            Route { to: "counter" , CounterPage {}},
            Route { to: "", NotFound {}}
        }
    })
}

fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            gap: "1rem",

            Link {
                to: "/",
                "Home"
            }

            Link {
                to: "/counter",
                "Counter"
            }
        }
    })
}
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            gap: "1rem",

            h1 {
                "Welcome to Dioxus!"
            }

            p {
                "This is a simple example of a Dioxus app."
            }

                Link {
                    to: "/counter",
                     "Go to counter page"
                }
        }
    })
}

fn NotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "404 Not Found"
        }
    })
}

// define a component that renders a div with the text "Hello, world!"
fn CounterPage(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            gap: "1rem",

            h1 {
                "Hello, world!",
            }

            Button {
                on_click: move |_| count += 1
                "clicked {count} times"
            }
        },


    })
}

#[inline_props]
fn Button<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    on_click: EventHandler<'a, MouseEvent>,
) -> Element {
    cx.render(rsx! {
        button {
            onclick: move |event| on_click.call(event),
            children,
        }

    })
}
