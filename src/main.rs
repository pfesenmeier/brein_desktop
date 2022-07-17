#![allow(non_snake_case)]

mod head;
mod header;
mod explorer;
mod editor;

use dioxus::prelude::*;
use std::fs;
use head::HEAD;
use header::Header;
use explorer::Explorer;
use simple_logger::SimpleLogger;

fn main() {
    // simple_logger::init_with_level(log::Level::Debug).unwrap();
    SimpleLogger::new().with_utc_timestamps().with_level(log::LevelFilter::Info).init().unwrap();
    dioxus::desktop::launch_cfg(App, |c| {
        c.with_window(|w| w.with_resizable(true).with_maximized(true))
    });
}


pub fn App(cx: Scope) -> Element {
    let files = use_ref(&cx, || Files::new());
    let (buffer, set_buffer) = use_state(&cx, || "".to_string());
    let (preview, set_preview) = use_state(&cx, || "".to_string());  

    rsx!(cx, div {
        class: "flex flex-col",
        HEAD {},
        Header { 
            button: rsx!(cx, i { class: "material-icons", onclick: move |_| files.write().go_up(), "logout" }),
            title: rsx!(cx, h1 { "Files: " [files.read().current()]  }),
        },
        div {
            class: "flex",
            Explorer {
                files: files,
                on_folder_click: move |dir_id| { 
                  files.write().enter_dir(dir_id);
                  set_preview("".to_string());
                  set_buffer("".to_string());
                },
                on_file_click: move |dir_id| {
                  files.write().select_file(dir_id); 
                  let file_contents = files.read().get_file_contents();
                  set_preview(preview_file(&file_contents));
                  set_buffer(file_contents);
                }
            },
            textarea {
              class: "bg-red-100 p-3 flex-1 p-3",
              oninput: move |event| {
                let input = event.value.clone();
                set_preview(preview_file(&input));
                set_buffer(input); 
              }, 
              "{buffer}"
            }
            div {
              class: "preview p-3 bg-sky-200 flex-1 display-inline",
              dangerous_inner_html: "{preview}"
            }
        }
    })
}

#[derive(Default, PartialEq)]
pub struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    err: Option<String>,
    selected_file: Option<usize>,
    file_contents: Option<String>,
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
        self.selected_file = Some(dir_id);
        self.load_from_disk();
    }

    fn load_from_disk(&mut self) {
        if let Some(dir_id) = self.selected_file {
            let path = &self.path_names[dir_id];
            let file = &fs::read(path).unwrap();
            let contents = String::from_utf8_lossy(&file).parse::<String>().unwrap();
            self.file_contents = Some(contents);
        }

    }

    fn get_file_contents(&self) -> String {
        if let Some(file) = &self.file_contents {
            file.to_string()
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
        self.file_contents = None;
    }
}


fn preview_file(contents: &str) -> String {
        let options = pulldown_cmark::Options::empty();
        let parser = pulldown_cmark::Parser::new_ext(&contents, options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        html_output
}
