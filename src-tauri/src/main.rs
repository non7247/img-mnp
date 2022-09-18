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
    original_pixels: Vec<u8>,
}

struct ImagePathState {
    state: Mutex<ImagePath>,
}

impl ImagePathState {
    pub fn new() -> Self {
        ImagePathState {
            state: Mutex::new(ImagePath{ original: String::from(""),
                                         work: String::from(""),
                                         original_pixels: Vec::new() })
        }
    }

    pub fn set_original(&self, s: &str) {
        let mut image_path = self.state.lock().unwrap();

        image_path.original = s.to_string();
    }

    pub fn set_original_pixels(&self, pixels: &Vec<u8>) {
        let mut image_path = self.state.lock().unwrap();

        image_path.original_pixels.reserve(pixels.len());
        for pixel in pixels {
            image_path.original_pixels.push(*pixel);
        }
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
        let gray = gray as u8;
        *pixel = image::Rgb([gray, gray, gray]);
    }

    img.save(work_path).unwrap();

    Ok(())
}
fn make_sepia_image(original_path: &str, work_path: &str) -> std::io::Result<()> {
    if Path::new(work_path).exists() {
        fs::remove_file(work_path)?;
    }

    let img = image::open(original_path).unwrap();
    let mut img = img.to_rgb8();

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let r = pixel[0] as f64 * 0.393 + pixel[1] as f64 * 0.769 + pixel[2] as f64 * 0.189;
        let g = pixel[0] as f64 * 0.349 + pixel[1] as f64 * 0.686 + pixel[2] as f64 * 0.168;
        let b = pixel[0] as f64 * 0.272 + pixel[1] as f64 * 0.534 + pixel[2] as f64 * 0.131;
        
        let r = if r > 255.0 { 255 as u8 } else { r as u8 };
        let g = if g > 255.0 { 255 as u8 } else { g as u8 };
        let b = if b > 255.0 { 255 as u8 } else { b as u8 };

        *pixel = image::Rgb([r, g, b]);
    }

    img.save(work_path).unwrap();

    Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        set_original_path,
        get_original_path,
        set_original_pixels,
        convert_to_invert,
        convert_to_grayscale,
        convert_to_sepia,
        convert_to_invert_array,
        convert_to_grayscale_array,
        convert_to_sepia_array,
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
fn set_original_pixels(image_path_state: State<'_, ImagePathState>, pixels: Vec<u8>) {
    image_path_state.set_original_pixels(&pixels);
}

#[tauri::command]
fn convert_to_invert(image_path_state: State<'_, ImagePathState>) -> String {
    let original_path = image_path_state.get_original();
    let work_path = image_path_state.make_work_path();

    if let Err(err) = make_invert_image(&original_path, &work_path) {
        println!("{}", err);
        return String::from("");
    }

    work_path
}

#[tauri::command]
fn convert_to_grayscale(image_path_state: State<'_, ImagePathState>) -> String {
    let original_path = image_path_state.get_original();
    let work_path = image_path_state.make_work_path();

    if let Err(err) = make_grayscale_image(&original_path, &work_path) {
        println!("{}", err);
        return String::from("");
    }

    work_path
}

#[tauri::command]
fn convert_to_sepia(image_path_state: State<'_, ImagePathState>) -> String {
    let original_path = image_path_state.get_original();
    let work_path = image_path_state.make_work_path();

    if let Err(err) = make_sepia_image(&original_path, &work_path) {
        println!("{}", err);
        return String::from("");
    }

    work_path
}

#[tauri::command]
fn convert_to_invert_array(pixels: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    result.reserve(pixels.len() - 1);

    for i in (0..pixels.len()).step_by(4) {
        result.push(255 - pixels[i]);
        result.push(255 - pixels[i + 1]);
        result.push(255 - pixels[i + 2]);
        result.push(pixels[i + 3]);
    }

    result
}

#[tauri::command]
fn convert_to_grayscale_array(pixels: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    result.reserve(pixels.len());

    for i in (0..pixels.len()).step_by(4) {
        let gray = pixels[i] as f64 * 0.3
                 + pixels[i + 1] as f64 * 0.59
                 + pixels[i + 2] as f64 * 0.11;
        let gray = gray as u8;
        result.push(gray);
        result.push(gray);
        result.push(gray);
        result.push(pixels[i + 3]);
    }

    result
}

#[tauri::command]
fn convert_to_sepia_array(pixels: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    result.reserve(pixels.len());

    for i in (0..pixels.len()).step_by(4) {
        let r = pixels[i] as f64 * 0.393
              + pixels[i + 1] as f64 * 0.769
              + pixels[i + 2] as f64 * 0.189;
        let g = pixels[i] as f64 * 0.349
              + pixels[i + 1] as f64 * 0.686
              + pixels[i + 2] as f64 * 0.168;
        let b = pixels[i] as f64 * 0.272
              + pixels[i + 1] as f64 * 0.534
              + pixels[i + 2] as f64 * 0.131;

        let r = if r > 255.0 { 255 as u8 } else { r as u8 };
        let g = if g > 255.0 { 255 as u8 } else { g as u8 };
        let b = if b > 255.0 { 255 as u8 } else { b as u8 };

        result.push(r);
        result.push(g);
        result.push(b);
        result.push(pixels[i + 3]);
    }

    result
}
