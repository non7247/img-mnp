#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;
use std::path::Path;
use tauri::{Manager, State};

#[derive(Debug)]
struct ImagePath {
    original: String,
    work: String,
}

struct ImagePathState {
    state: Mutex<ImagePath>,
}

impl ImagePathState {
    pub fn new() -> Self {
        ImagePathState {
            state: Mutex::new(ImagePath{ original: String::from(""),
                                         work: String::from("") })
        }
    }

    pub fn set_original(&self, s: &str) {
        let mut image_path = self.state.lock().unwrap();

        image_path.original = s.to_string();
    }

    pub fn get_original(&self) -> String {
        let image_path = self.state.lock().unwrap();

        image_path.original.clone()
    }

    pub fn make_work_path(&self) -> String {
        let mut image_path = self.state.lock().unwrap();

        let original_path = Path::new(&image_path.original);
        let file_name = match original_path
            .file_name()
            .and_then(|s| s.to_str()) {
                Some(s) => s,
                None => {
                    image_path.work = String::from("");
                    return image_path.work.clone();
                },
        };

        let work_name = format!("$$$_{}", file_name);
        let work_path = original_path.with_file_name(work_name);

        image_path.work = match work_path.to_str() {
            Some(s) => String::from(s),
            None => String::from(""),
        };
        image_path.work = image_path.work.replace("\\", "/");
        image_path.work.clone()
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        set_original_path,
        get_original_path,
        convert_to_invert,
    ])
    .setup(|app| {
        let image_path_state = ImagePathState::new();
        app.manage(image_path_state);

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn set_original_path(image_path_state: State<'_, ImagePathState>, path: &str) {
    image_path_state.set_original(path);
}

#[tauri::command]
fn get_original_path(image_path_state: State<'_, ImagePathState>) -> String {
    image_path_state.get_original()
}

#[tauri::command]
fn convert_to_invert(image_path_state: State<'_, ImagePathState>) -> String {
    image_path_state.make_work_path()
}
