#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod explorer;
mod head;
mod header;

use dioxus::prelude::*;
use explorer::Explorer;
use head::HEAD;
use header::Header;
use std::fs;
use std::io::Write;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    dioxus::desktop::launch_cfg(App, |c| {
        c.with_window(|w| w.with_resizable(true).with_maximized(true))
   });
}

static App: Component<()> = |cx| {
    let files = use_ref(&cx, || Files::new());
    let preview = use_state(&cx, || "".to_string());
    let buffer = use_state(&cx, || "".to_string());

    rsx!(cx, div {
        class: "flex flex-col",
        HEAD {},
        Header {
            save_button: rsx!(cx, div { class: "p-3 bg-green-300", onclick: move |_| files.write().save_file(buffer).unwrap_or_default(), "Save File" }),
            back_button: rsx!(cx, i { class: "material-icons", onclick: move |_| files.write().go_up(), "logout" }),
            title: rsx!(cx, h1 { "Files: " [files.read().current()]  }),
        },
        div {
            class: "flex",
            Explorer {
                files: files,
                on_folder_click: move |dir_id| {
                  files.write().enter_dir(dir_id);
                  preview.set("".to_string());
                  buffer.set("".to_string());
                },
                on_file_click: move |dir_id| {
                  files.write().select_file(dir_id);
                  let file_contents = files.read().get_file_contents();
                  preview.set(preview_file(&file_contents));
                  buffer.set(file_contents);
                }
            },
            textarea {
              class: "bg-red-100 p-3 flex-1 p-3",
              oninput: move |event| {
                let input = event.value.clone();
                preview.set(preview_file(&input));
                buffer.set(input);
              },
              value: "{buffer}"
            }
            div {
              class: "preview p-3 bg-sky-200 flex-1 display-inline",
              dangerous_inner_html: "{preview}"
            }
        }
    })
};

#[derive(Default, PartialEq)]
pub struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    err: Option<String>,
    selected_file: Option<usize>,
}

impl Files {
    fn new() -> Self {
        let mut files = Self {
            path_stack: vec!["./".to_string()],
            ..Default::default()
        };

        files.reload_path_list();

        files
    }

    fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.last().unwrap();
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

        self.clear_err();
        self.path_names.clear();
        self.clear_selected_file();

        for path in collected {
            self.path_names
                .push(path.unwrap().path().display().to_string());
        }
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
        self.selected_file = Some(dir_id);
    }

    fn get_file_contents(&self) -> String {
        if let Some(dir_id) = self.selected_file {
            let path = &self.path_names[dir_id];
            let file = &fs::read(path).unwrap_or_default();
            let contents: String = String::from_utf8_lossy(&file).parse().unwrap_or_default();
            contents
        } else {
            "".to_string()
        }
    }

    fn save_file(&self, contents: &str) -> Result<(), impl std::error::Error> {
        if let Some(dir_id) = self.selected_file {
            let path = &self.path_names[dir_id];
            let mut file = fs::OpenOptions::new()
                     //   .read(true)
                        .write(true)
                        .open(path)?;
            file.write_all(contents.as_bytes())
        } else {
            Ok(())
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

fn preview_file(contents: &str) -> String {
    let options = pulldown_cmark::Options::empty();
    let parser = pulldown_cmark::Parser::new_ext(&contents, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
