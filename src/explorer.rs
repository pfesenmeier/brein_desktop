use dioxus::prelude::*;
use crate::Files;

#[derive(Props)]
pub struct ExplorerProps<'a> {
    files: &'a UseRef<Files>,
    on_folder_click: EventHandler<'a, usize>,
    on_file_click: EventHandler<'a, usize>,
}

pub fn Explorer<'a>(cx: Scope<'a, ExplorerProps<'a>>) -> Element<'a> {
            rsx!(cx, div {
                class: "flex flex-col",
                cx.props.files.read().path_names.iter().enumerate().map(|(dir_id, path)| {
                    let path_end = path.split('/').last().unwrap_or(path.as_str());
                    let (icon_type, on_click): (&str, &EventHandler<'a, usize>) = if path_end.contains(".") {
                        ("description" , &cx.props.on_folder_click )
                    } else {
                        ("folder", &cx.props.on_file_click )
                    };

                    rsx! (
                        div { class: "folder flex", key: "{path}",
                            i { class: "material-icons",
                                onclick: move |_| on_click.call(dir_id),
                                "{icon_type}"
                                p { class: "cooltip", "0 folders / 0 files" }
                            }
                            h1 { class: "color-zinc-300", "{path_end}" }
                        }
                    )
                })
            })

}
