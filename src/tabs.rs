use dioxus::prelude::*;

#[inline_props]
pub fn Tabs<'a>(cx: Scope, children: Element<'a>) -> Element {
    rsx!(cx, div {
        class: "flex",
        children
    })
}
