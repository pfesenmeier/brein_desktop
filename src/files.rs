use std::fs;
use std::io::Write;

#[derive(Default, PartialEq)]
pub struct Files {
    path_stack: Vec<String>,
    pub path_names: Vec<String>,
    err: Option<String>,
    selected_file: Option<usize>,
}

impl Files {
   pub fn new() -> Self {
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

    pub fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    pub fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_names[dir_id];
        self.path_stack.push(path.clone());
        self.reload_path_list();
    }

    pub fn select_file(&mut self, dir_id: usize) {
        self.selected_file = Some(dir_id);
    }

    pub fn get_file_contents(&self) -> String {
        if let Some(dir_id) = self.selected_file {
            let path = &self.path_names[dir_id];
            let file = &fs::read(path).unwrap_or_default();
            let contents: String = String::from_utf8_lossy(&file).parse().unwrap_or_default();
            contents
        } else {
            "".to_string()
        }
    }

    pub fn save_file(&self, contents: &str) -> Result<(), impl std::error::Error> {
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

    pub fn current(&self) -> &str {
        self.path_stack.last().unwrap()
    }
    fn clear_err(&mut self) {
        self.err = None;
    }
    fn clear_selected_file(&mut self) {
        self.selected_file = None;
    }
}

