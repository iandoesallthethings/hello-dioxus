use crate::prelude::*;
mod counter;
pub use counter::*;

pub fn Routes(cx: Scope) -> Element {
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

pub fn Home(cx: Scope) -> Element {
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

pub fn NotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "404 Not Found"
        }
    })
}
