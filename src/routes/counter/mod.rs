use crate::prelude::*;

pub fn CounterPage(cx: Scope) -> Element {
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
