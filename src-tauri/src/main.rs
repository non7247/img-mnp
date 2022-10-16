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
    height: u32,
    width: u32,
}

struct ImagePathState {
    state: Mutex<ImagePath>,
}

impl ImagePathState {
    pub fn new() -> Self {
        ImagePathState {
            state: Mutex::new(ImagePath{ original: String::from(""),
                                         work: String::from(""),
                                         original_pixels: Vec::new(),
                                         height: 0,
                                         width: 0 })
        }
    }

    pub fn set_original(&self, s: &str) {
        let mut image_path = self.state.lock().unwrap();

        image_path.original = s.to_string();
    }

    pub fn set_original_pixels(&self, pixels: &Vec<u8>, height: u32, width: u32) {
        let mut image_path = self.state.lock().unwrap();

        image_path.original_pixels.clear();
        image_path.original_pixels.reserve(pixels.len());
        for pixel in pixels {
            image_path.original_pixels.push(*pixel);
        }

        image_path.width = width;
        image_path.height = height;

        println!("original_pixels.len: {}", image_path.original_pixels.len());
        println!("heght: {}, width: {}", image_path.height, image_path.width);
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

    pub fn make_invert_array(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let image_path = self.state.lock().unwrap();
        if image_path.original_pixels.len() > 0 {
            let pixels = &image_path.original_pixels;
            result = to_invert_array(pixels);
        }

        result
    }

    pub fn make_grayscale_array(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let image_path = self.state.lock().unwrap();
        if image_path.original_pixels.len() > 0 {
            let pixels = &image_path.original_pixels;
            result = to_grayscale_array(pixels);
        }

        result
    }

    pub fn make_sepia_array(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let image_path = self.state.lock().unwrap();
        if image_path.original_pixels.len() > 0 {
            let pixels = &image_path.original_pixels;
            result = to_sepia_array(pixels);
        }

        result
    }

    pub fn make_mosaic_array(&self, area: u32) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let image_path = self.state.lock().unwrap();
        if image_path.original_pixels.len() > 0 {
            let pixels = &image_path.original_pixels;
            result = to_mosaic_array(pixels, image_path.height, image_path.width, area);
        }

        result
    }
}

fn to_invert_image(original_path: &str, work_path: &str) -> std::io::Result<()> { 
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

fn to_grayscale_image(original_path: &str, work_path: &str) -> std::io::Result<()> {
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

fn to_sepia_image(original_path: &str, work_path: &str) -> std::io::Result<()> {
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

fn to_invert_array(pixels: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    result.reserve(pixels.len());

    for i in (0..pixels.len()).step_by(4) {
        result.push(255 - pixels[i]);
        result.push(255 - pixels[i + 1]);
        result.push(255 - pixels[i + 2]);
        result.push(pixels[i + 3]);
    }

    result
}

fn to_grayscale_array(pixels: &Vec<u8>) -> Vec<u8> {
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

fn to_sepia_array(pixels: &Vec<u8>) -> Vec<u8> {
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

fn calc_pixel_average(acc: u32, count: u32) -> u8 {
    let avg = acc / count;
    if avg > 255 {
        255
    } else {
        avg as u8
    }
}

fn to_mosaic_array(pixels: &Vec<u8>, height: u32, width: u32, area: u32) -> Vec<u8> {
    let mut result = pixels.to_vec();

    if pixels.len() != height as usize * width as usize * 4 {
        return result;
    }

    for y in (0..height).step_by(area as usize) {
        if y + area > height { break; }

        for x in (0..width).step_by(area as usize) {
            if x + area > width { break; }

            let mut acc_r: u32 = 0;
            let mut acc_g: u32 = 0;
            let mut acc_b: u32 = 0;

            for ya in 0..area {
                let row_s = (y + ya) * width * 4;

                for xa in 0..area {
                    let cp = (row_s + (x + xa) * 4) as usize;

                    acc_r += pixels[cp] as u32;
                    acc_g += pixels[cp + 1] as u32;
                    acc_b += pixels[cp + 2] as u32;
                }
            }

            let r = calc_pixel_average(acc_r, area * area);
            let g = calc_pixel_average(acc_g, area * area);
            let b = calc_pixel_average(acc_b, area * area);

            for ya in 0..area {
                let row_s = (y + ya) * width * 4;

                for xa in 0..area {
                    let cp = (row_s + (x + xa) * 4) as usize;

                    result[cp] = r as u8;
                    result[cp + 1] = g as u8;
                    result[cp + 2] = b as u8;
                }
            }
        }

        if width % area != 0 {
            let rm = width % area;

            let mut acc_r: u32 = 0;
            let mut acc_g: u32 = 0;
            let mut acc_b: u32 = 0;

            for ya in 0..area {
                let row_s = (y + ya) * width * 4;

                for x in (width - rm)..width {
                    let cp = (row_s + x * 4) as usize;

                    acc_r += pixels[cp] as u32;
                    acc_g += pixels[cp + 1] as u32;
                    acc_b += pixels[cp + 2] as u32;
                }
            }

            let r = calc_pixel_average(acc_r, area * rm);
            let g = calc_pixel_average(acc_g, area * rm);
            let b = calc_pixel_average(acc_b, area * rm);

            for ya in 0..area {
                let row_s = (y + ya) * width * 4;

                for x in (width - rm)..width {
                    let cp = (row_s + x * 4) as usize;

                    result[cp] = r as u8;
                    result[cp + 1] = g as u8;
                    result[cp + 2] = b as u8;
                }
            }
        }
    }

    if height % area != 0 {
        let rm = height % area;

        for x in (0..width).step_by(area as usize) {
            if x + area > width { break; }

            let mut acc_r: u32 = 0;
            let mut acc_g: u32 = 0;
            let mut acc_b: u32 = 0;

            for y in (height - rm)..height {
                let row_s = y * width * 4;

                for xa in 0..area {
                    let cp = (row_s + (x + xa) * 4) as usize;

                    acc_r += pixels[cp] as u32;
                    acc_g += pixels[cp + 1] as u32;
                    acc_b += pixels[cp + 2] as u32;
                }
            }

            let r = calc_pixel_average(acc_r, rm * area);
            let g = calc_pixel_average(acc_g, rm * area);
            let b = calc_pixel_average(acc_b, rm * area);

            for y in (height - rm)..height {
                let row_s = y * width * 4;

                for xa in 0..area {
                    let cp = (row_s + (x + xa) * 4) as usize;

                    result[cp] = r as u8;
                    result[cp + 1] = g as u8;
                    result[cp + 2] = b as u8;
                }
            }
        }

        if width % area != 0 {
            let rm_w = width % area;

            let mut acc_r: u32 = 0;
            let mut acc_g: u32 = 0;
            let mut acc_b: u32 = 0;

            for y in (height - rm)..height {
                let row_s = y * width * 4;

                for x in (width - rm_w)..width {
                    let cp = (row_s + x * 4) as usize;

                    acc_r += pixels[cp] as u32;
                    acc_g += pixels[cp + 1] as u32;
                    acc_b += pixels[cp + 2] as u32;
                }
            }

            let r = calc_pixel_average(acc_r, rm * rm_w);
            let g = calc_pixel_average(acc_g, rm * rm_w);
            let b = calc_pixel_average(acc_b, rm * rm_w);

            for y in (height - rm)..height {
                let row_s = y * width * 4;

                for x in (width - rm_w)..width {
                    let cp = (row_s + x * 4) as usize;

                    result[cp] = r as u8;
                    result[cp + 1] = g as u8;
                    result[cp + 2] = b as u8;
                }
            }
        }
    }

    result
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
fn set_original_pixels(image_path_state: State<'_, ImagePathState>, 
                       pixels: Vec<u8>, height: u32, width: u32) {
    image_path_state.set_original_pixels(&pixels, height, width);
}

#[tauri::command]
fn convert_to_invert(image_path_state: State<'_, ImagePathState>) -> String {
    let original_path = image_path_state.get_original();
    let work_path = image_path_state.make_work_path();

    if let Err(err) = to_invert_image(&original_path, &work_path) {
        println!("{}", err);
        return String::from("");
    }

    work_path
}

#[tauri::command]
fn convert_to_grayscale(image_path_state: State<'_, ImagePathState>) -> String {
    let original_path = image_path_state.get_original();
    let work_path = image_path_state.make_work_path();

    if let Err(err) = to_grayscale_image(&original_path, &work_path) {
        println!("{}", err);
        return String::from("");
    }

    work_path
}

#[tauri::command]
fn convert_to_sepia(image_path_state: State<'_, ImagePathState>) -> String {
    let original_path = image_path_state.get_original();
    let work_path = image_path_state.make_work_path();

    if let Err(err) = to_sepia_image(&original_path, &work_path) {
        println!("{}", err);
        return String::from("");
    }

    work_path
}

#[tauri::command]
fn convert_to_invert_array(pixels: Vec<u8>) -> Vec<u8> {
    to_invert_array(&pixels)
}

#[tauri::command]
fn convert_to_grayscale_array(pixels: Vec<u8>) -> Vec<u8> {
    to_grayscale_array(&pixels)
}

#[tauri::command]
fn convert_to_sepia_array(pixels: Vec<u8>) -> Vec<u8> {
    to_sepia_array(&pixels)
}

#[tauri::command]
fn convert_to_invert_im(image_path_state: State<'_, ImagePathState>) -> Vec<u8> {
    image_path_state.make_invert_array()
}

#[tauri::command]
fn convert_to_grayscale_im(image_path_state: State<'_, ImagePathState>) -> Vec<u8> {
    image_path_state.make_grayscale_array()
}

#[tauri::command]
fn convert_to_sepia_im(image_path_state: State<'_, ImagePathState>) -> Vec<u8> {
    image_path_state.make_sepia_array()
}

#[tauri::command]
fn convert_to_mosaic(image_path_state: State<'_, ImagePathState>, area: u32) -> Vec<u8> {
    image_path_state.make_mosaic_array(area)
}
