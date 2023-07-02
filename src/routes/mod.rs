use crate::prelude::*;
mod counter;
use counter::*;
mod files;
use files::*;

pub fn Routes(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Nav {},
            Route { to: "/", Home {}}
            Route { to: "/counter" , CounterPage {}},
            Route { to: "/files", FilesPage {}},
            Route { to: "", NotFound {}}
        }
    })
}

fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "flex flex-row items-center gap-2 p-2",

            Link {
                to: "/",
                "Home"
            }

            Link {
                to: "/counter",
                "Counter"
            }

            Link {
                to: "/files",
                "Files"
            }
        }
    })
}

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col justify-center gap-2 text-center",

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
