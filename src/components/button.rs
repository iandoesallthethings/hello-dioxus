use crate::prelude::*;

#[inline_props]
pub fn Button<'a>(
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
