use dioxus::prelude::*;

#[derive(Props)]
pub struct HeaderProps<'a> {
    title: Element<'a>,
    back_button: Element<'a>,
    save_button: Element<'a>,
}

pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element<'a> {
    rsx!(cx, 
        header {
            i { class: "material-icons icon-menu", "menu" },
            cx.props.title.as_ref(),
            span { },
            cx.props.save_button.as_ref(),   
            cx.props.back_button.as_ref(),
        }
    )
}

