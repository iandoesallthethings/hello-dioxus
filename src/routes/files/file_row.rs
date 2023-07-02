use crate::prelude::*;

#[inline_props]
pub fn FileRow(cx: Scope, path: String) -> Element {
    cx.render(rsx! {
        li {
                    "{path}"
                    audio { src: "{path}", controls: "true" }
        }
    })
}
