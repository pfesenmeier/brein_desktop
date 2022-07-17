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
                class: "flex flex-col bg-zinc-300 w-80 h-screen",
                cx.props.files.read().path_names.iter().enumerate().map(|(dir_id, path)| {
                    let path_end = path.split('/').last().unwrap_or(path.as_str());
                    let (icon_type, on_click): (&str, &EventHandler<'a, usize>) = if path_end.contains(".") {
                        ("description" , &cx.props.on_file_click )
                    } else {
                        ("folder", &cx.props.on_folder_click )
                    };

                    rsx! (
                        button { 
                            class: "folder flex p-3 border-2 cursor-pointer",
                            key: "{path}",
                            onclick: move |_| on_click.call(dir_id),
                            i { class: "material-icons",
                                "{icon_type}"
                            }
                            h3 { class: "pl-3", "{path_end}" }
                        }
                    )
                })
            })

}
