use dioxus::prelude::*;

#[derive(Props)]
pub struct HeaderProps<'a> {
    title: Element<'a>,
    button: Element<'a>,
}

pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element<'a> {
    rsx!(cx, 
        header {
            i { class: "material-icons icon-menu", "menu" },
            cx.props.title.as_ref(),
            span { },
            cx.props.button.as_ref(),
        }
    )
}

