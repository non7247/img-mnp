#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod image_processing;

use std::sync::Mutex;
use std::path::Path;
use tauri::{Manager, State};

#[derive(Debug)]
struct ImagePath {
    original: String,
    work: String,

    original_pixels: Vec<u8>,
    height: u32,
    width: u32,
}

struct ImagePathState {
    state: Mutex<ImagePath>,
}

impl ImagePathState {
    pub fn new() -> Self {
        ImagePathState {
            state: Mutex::new(
                ImagePath{
                     original: String::from(""),
                    work: String::from(""),
                    original_pixels: Vec::new(),
                    height: 0,
                    width: 0
                }
            )
        }
    }

    pub fn set_original(&self, s: &str) -> Result<(), String> {
        match self.state.lock() {
            Ok(mut image_path) => {
                image_path.original = s.to_string();
                Ok(())
            },
            Err(e) => Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        }
    }

    pub fn set_original_pixels(&self, pixels: &Vec<u8>, height: u32, width: u32) -> Result<(), String> {
        match self.state.lock() {
            Ok(mut image_path) => {
                image_path.original_pixels.clear();
                image_path.original_pixels.reserve(pixels.len());
                for pixel in pixels {
                    image_path.original_pixels.push(*pixel);
                }

                image_path.width = width;
                image_path.height = height;

                println!("original_pixels.len: {}", image_path.original_pixels.len());
                println!("heght: {}, width: {}", image_path.height, image_path.width);
                Ok(())
            },
            Err(e) => Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        }
    }

    pub fn get_original(&self) -> Result<String, String> {
        match self.state.lock() {
            Ok(image_path) => Ok(image_path.original.clone()),
            Err(e) => Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        }
    }

    pub fn make_work_path(&self) -> Result<String, String> {
        let mut image_path = match self.state.lock() {
            Ok(state) => state,
            Err(e) => return Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        };

        let original_path = Path::new(&image_path.original);
        let file_name = match original_path
            .file_name()
            .and_then(|s| s.to_str()) {
                Some(s) => s,
                None => {
                    image_path.work = String::from("");
                    return Ok(image_path.work.clone());
                },
        };

        let work_name = format!("_${}", file_name);
        let work_path = original_path.with_file_name(work_name);

        image_path.work = match work_path.to_str() {
            Some(s) => String::from(s),
            None => String::from(""),
        };
        image_path.work = image_path.work.replace("\\", "/");
        Ok(image_path.work.clone())
    }

    pub fn make_invert_array(&self) -> Result<Vec<u8>, String> {
        match self.state.lock() {
            Ok(image_path) => {
                let mut result: Vec<u8> = Vec::new();
                if image_path.original_pixels.len() > 0 {
                    let pixels = &image_path.original_pixels;
                    result = image_processing::to_invert_array(pixels);
                }
                Ok(result)
            }
            Err(e) => Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        }
    }

    pub fn make_grayscale_array(&self) -> Result<Vec<u8>, String> {
        match self.state.lock() {
            Ok(image_path) => {
                let mut result: Vec<u8> = Vec::new();
                if image_path.original_pixels.len() > 0 {
                    let pixels = &image_path.original_pixels;
                    result = image_processing::to_grayscale_array(pixels);
                }
                Ok(result)
            }
            Err(e) => Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        }
    }

    pub fn make_sepia_array(&self) -> Result<Vec<u8>, String> {
        match self.state.lock() {
            Ok(image_path) => {
                let mut result: Vec<u8> = Vec::new();
                if image_path.original_pixels.len() > 0 {
                    let pixels = &image_path.original_pixels;
                    result = image_processing::to_sepia_array(pixels);
                }
                Ok(result)
            }
            Err(e) => Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        }
    }

    pub fn make_mosaic_array(&self, area: u32) -> Result<Vec<u8>, String> {
        match self.state.lock() {
            Ok(image_path) => {
                let mut result: Vec<u8> = Vec::new();
                if image_path.original_pixels.len() > 0 {
                    let pixels = &image_path.original_pixels;
                    result = image_processing::to_mosaic_array(pixels, image_path.height, image_path.width, area);
                }
                Ok(result)
            }
            Err(e) => Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        }
    }

    pub fn make_smoothing_array(&self) -> Result<Vec<u8>, String> {
        match self.state.lock() {
            Ok(image_path) => {
                let mut result: Vec<u8> = Vec::new();
                if image_path.original_pixels.len() > 0 {
                    let pixels: &Vec<u8> = &image_path.original_pixels;
                    result = image_processing::to_smoothing_array(pixels, image_path.height, image_path.width);
                }
                Ok(result)
            }
            Err(e) => Err(format!("Failed to acquire lock on ImagePathState: {}", e)),
        }
    }
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
        convert_to_invert_im,
        convert_to_grayscale_im,
        convert_to_sepia_im,
        convert_to_mosaic,
        convert_to_smoothing,
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
fn set_original_path(image_path_state: State<'_, ImagePathState>, path: &str) -> Result<(), String> {
    image_path_state.set_original(path)
}

#[tauri::command]
fn get_original_path(image_path_state: State<'_, ImagePathState>) -> Result<String, String> {
    image_path_state.get_original()
}

#[tauri::command]
fn set_original_pixels(image_path_state: State<'_, ImagePathState>, 
                       pixels: Vec<u8>, height: u32, width: u32) -> Result<(), String> {
    image_path_state.set_original_pixels(&pixels, height, width)
}

#[tauri::command]
fn convert_to_invert(image_path_state: State<'_, ImagePathState>) -> Result<String, String> {
    let original_path = image_path_state.get_original()?;
    let work_path = image_path_state.make_work_path()?;

    image_processing::to_invert_image(&original_path, &work_path).map_err(|e| e.to_string())?;

    Ok(work_path)
}

#[tauri::command]
fn convert_to_grayscale(image_path_state: State<'_, ImagePathState>) -> Result<String, String> {
    let original_path = image_path_state.get_original()?;
    let work_path = image_path_state.make_work_path()?;

    image_processing::to_grayscale_image(&original_path, &work_path).map_err(|e| e.to_string())?;

    Ok(work_path)
}

#[tauri::command]
fn convert_to_sepia(image_path_state: State<'_, ImagePathState>) -> Result<String, String> {
    let original_path = image_path_state.get_original()?;
    let work_path = image_path_state.make_work_path()?;

    image_processing::to_sepia_image(&original_path, &work_path).map_err(|e| e.to_string())?;

    Ok(work_path)
}

#[tauri::command]
fn convert_to_invert_array(pixels: Vec<u8>) -> Vec<u8> {
    image_processing::to_invert_array(&pixels)
}

#[tauri::command]
fn convert_to_grayscale_array(pixels: Vec<u8>) -> Vec<u8> {
    image_processing::to_grayscale_array(&pixels)
}

#[tauri::command]
fn convert_to_sepia_array(pixels: Vec<u8>) -> Vec<u8> {
    image_processing::to_sepia_array(&pixels)
}

#[tauri::command]
fn convert_to_invert_im(image_path_state: State<'_, ImagePathState>) -> Result<Vec<u8>, String> {
    image_path_state.make_invert_array()
}

#[tauri::command]
fn convert_to_grayscale_im(image_path_state: State<'_, ImagePathState>) -> Result<Vec<u8>, String> {
    image_path_state.make_grayscale_array()
}

#[tauri::command]
fn convert_to_sepia_im(image_path_state: State<'_, ImagePathState>) -> Result<Vec<u8>, String> {
    image_path_state.make_sepia_array()
}

#[tauri::command]
fn convert_to_mosaic(image_path_state: State<'_, ImagePathState>, area: u32) -> Result<Vec<u8>, String> {
    image_path_state.make_mosaic_array(area)
}

#[tauri::command]
fn convert_to_smoothing(image_path_state: State<'_, ImagePathState>) -> Result<Vec<u8>, String> {
    image_path_state.make_smoothing_array()
}
