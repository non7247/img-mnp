#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;
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
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        set_original_path,
        get_original_path,
        img_invert,
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
fn img_invert() -> String {
    let result = String::from("");

    result
}
