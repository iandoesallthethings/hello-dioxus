use crate::prelude::*;
mod counter;
use counter::*;
mod files;
use files::*;

pub fn Routes(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col items-center gap-2",
            Router {
                Nav {}

                Route { to: "/", Home {}}
                Route { to: "/counter" , CounterPage {}},
                Route { to: "/files", FilesPage {}},
                Route { to: "", NotFound {}}
            }
        }
    })
}

fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "flex flex-row w-full items-center gap-2 p-2",

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
        h1 {
            "Welcome to Dioxus!"
        }

        p {
            "This is a simple example of a Dioxus app."
        }
    })
}

pub fn NotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "404 Not Found"
        }
    })
}
