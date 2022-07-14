//! Example: File Explorer
//! -------------------------
//!
//! This is a fun little desktop application that lets you explore the file system.
//!
//! This example is interesting because it's mixing filesystem operations and GUI, which is typically hard for UI to do.
mod head;
mod header;
mod tabs;
mod explorer;

use dioxus::prelude::*;
use std::boxed::Box;
use std::fs;
use head::HEAD;
use header::Header;
use tabs::Tabs;
use explorer::Explorer;


fn main() {
    // simple_logger::init_with_level(log::Level::Debug).unwrap();
    dioxus::desktop::launch_cfg(App, |c| {
        c.with_window(|w| w.with_resizable(true).with_maximized(true))
    });
}

#[derive(PartialEq)]
    enum EditorState {
        Display,
        Edit,
    }

pub fn App(cx: Scope) -> Element {
    let files = use_ref(&cx, || Files::new());
    let (edit_state, set_edit_state) = use_state(&cx, || EditorState::Display);

    let file = files.read().read_file();

    rsx!(cx, div {
        class: "flex",
        HEAD {},
        Header { 
            button: rsx!(cx, i { class: "material-icons", onclick: move |_| files.write().go_up(), "logout" }),
            title: rsx!(cx, h1 { "Files: " [files.read().current()]  }),
        },
        Explorer {
            files: files,
            on_folder_click: move |dir_id| files.write().select_file(dir_id),
            on_file_click: move |dir_id| files.write().enter_dir(dir_id),
        },
        main {
            class: "flex",
            Tabs {
                div {
                  class: "p-1 bg-blue-100",
                  onclick: move |_| set_edit_state(EditorState::Display),
                      "preview"
                }
                div {
                  class: "p-1 bg-red-100",
                  onclick: move |_| set_edit_state(EditorState::Edit),
                  "edit"
                }
            }
            div {
                class: "block",
                files.read().err.as_ref().map(|err| {
                    rsx! (
                        div {
                            code { "{err}" }
                            button { onclick: move |_| files.write().clear_err(), "x" }
                        }
                    )
                })
                match edit_state {
                        EditorState::Display => {
                            rsx! {
                                div {
                                    class: "markdown-editor",
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
           }
        }

    })
}

struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    err: Option<String>,
    selected_file: Option<usize>,
}

impl Files {
    fn new() -> Self {
        let mut files = Self {
            path_stack: vec!["./".to_string()],
            path_names: vec![],
            err: None,
            selected_file: None,
        };

        files.reload_path_list();

        files
    }

    fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.last().unwrap();
        log::info!("Reloading path list for {:?}", cur_path);
        let paths = match std::fs::read_dir(cur_path) {
            Ok(e) => e,
            Err(err) => {
                let err = format!("An error occured: {:?}", err);
                self.err = Some(err);
                self.path_stack.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();
        log::info!("Path list reloaded {:#?}", collected);

        // clear the current state
        self.clear_err();
        self.path_names.clear();
        self.clear_selected_file();

        for path in collected {
            self.path_names
                .push(path.unwrap().path().display().to_string());
        }
        log::info!("path namees are {:#?}", self.path_names);
    }

    fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_names[dir_id];
        self.path_stack.push(path.clone());
        self.reload_path_list();
    }

    fn select_file(&mut self, dir_id: usize) {
        self.selected_file = Some(dir_id)
    }

    fn read_file(&self) -> String {
        if let Some(dir_id) = self.selected_file {
            let path = &self.path_names[dir_id];
            let file = &fs::read(path).unwrap();
            let contents = String::from_utf8_lossy(&file).parse::<String>().unwrap();
            let options = pulldown_cmark::Options::empty();
            let parser = pulldown_cmark::Parser::new_ext(&contents, options);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            html_output
        } else {
            "".to_string()
        }
    }

    fn current(&self) -> &str {
        self.path_stack.last().unwrap()
    }
    fn clear_err(&mut self) {
        self.err = None;
    }
    fn clear_selected_file(&mut self) {
        self.selected_file = None;
    }
}
