use crate::Files;
use dioxus::prelude::*;

#[derive(PartialEq)]
pub enum EditorState {
    Display,
    Edit,
}

#[inline_props]
pub fn Editor<'a>(cx: Scope, files: &'a UseRef<Files>) -> Element {
    let (edit_state, set_edit_state) = use_state(&cx, || EditorState::Display);
    let file = files.read().read_file();

    rsx!(cx, 
        div {
            class: "w-full",
            div {
                class: "flex bg-zinc-400",
                div {
                  class: "bg-blue-100 px-5 py-3",
                  onclick: move |_| set_edit_state(EditorState::Display),
                  "preview"
                }
                div {
                  class: "bg-red-100 px-5 py-3",
                  onclick: move |_| set_edit_state(EditorState::Edit),
                  "edit"
                }
            }
            div {
            match edit_state {
                     EditorState::Display => {
                         rsx! {
                             div {
                                 dangerous_inner_html: "{file}"
                             },
                         }},
                     EditorState::Edit => {
                         let value = file;
                         rsx!{
                             textarea {
                                 class: "w-full h-screen",
                                // onchange: move |event| recipe_body.set((*event.value.clone()).to_string()),
                                 value: "{value}"
                             }
                         }
                 },
            },
            files.read().err.as_ref().map(|err| {
                rsx! (
                    div {
                        code { "{err}" }
                        button { onclick: move |_| files.write().clear_err(), "x" }
                    }
                )
            })
        }
    })
}
