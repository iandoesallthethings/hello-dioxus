use crate::prelude::*;

pub fn CounterPage(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        h1 { "Hello, world!" },

        Button {
            on_click: move |_| count += 1,
            "clicked {count} times"
        },
    })
}
