#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;
use std::path::Path;
use std::fs;
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

fn make_invert_image(original_path: &str, work_path: &str) -> std::io::Result<()> { 
    if Path::new(work_path).exists() {
        fs::remove_file(work_path)?;
    }

    let img = image::open(original_path).unwrap();
    let mut img = img.to_rgb8();

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let r = 255 - pixel[0];
        let g = 255 - pixel[1];
        let b = 255 - pixel[2];
        *pixel = image::Rgb([r, g, b]);
    }

    img.save(work_path).unwrap();

    Ok(())
}

fn make_grayscale_image(original_path: &str, work_path: &str) -> std::io::Result<()> {
    if Path::new(work_path).exists() {
        fs::remove_file(work_path)?;
    }

    let img = image::open(original_path).unwrap();
    let mut img = img.to_rgb8();

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let gray = pixel[0] as f64 * 0.3 + pixel[1] as f64 * 0.59 + pixel[2] as f64 * 0.11;
        let g = gray as u8;
        *pixel = image::Rgb([g, g, g]);
    }

    img.save(work_path).unwrap();

    Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        set_original_path,
        get_original_path,
        convert_to_invert,
        convert_to_grayscale,
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
    let original_path = image_path_state.get_original();
    let work_path = image_path_state.make_work_path();

    match make_invert_image(&original_path, &work_path) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            return String::from("");
        },
    }

    work_path
}

#[tauri::command]
fn convert_to_grayscale(image_path_state: State<'_, ImagePathState>) -> String {
    let original_path = image_path_state.get_original();
    let work_path = image_path_state.make_work_path();

    match make_grayscale_image(&original_path, &work_path) {
        Ok(()) => (),
        Err(err) => {
            print!("{}", err);
            return String::from("");
        },
    }

    work_path
}
