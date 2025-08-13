use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct SubArea {
    pub row: u32,
    pub col: u32,
    pub row_length: u32,
    pub height: u32,
    pub width: u32
}

impl SubArea {
    pub fn new(row: u32, col: u32, row_length: u32, height: u32, width: u32) -> Self {
        Self { row, col, row_length, height, width }
    }
}

pub fn to_invert_image(original_path: &str, work_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(work_path).exists() {
        fs::remove_file(work_path)?;
    }

    let img = image::open(original_path)?;
    let mut img = img.to_rgb8();

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let r = 255 - pixel[0];
        let g = 255 - pixel[1];
        let b = 255 - pixel[2];
        *pixel = image::Rgb([r, g, b]);
    }

    img.save(work_path)?;

    Ok(())
}

pub fn to_grayscale_image(original_path: &str, work_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(work_path).exists() {
        fs::remove_file(work_path)?;
    }

    let img = image::open(original_path)?;
    let mut img = img.to_rgb8();

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let gray = pixel[0] as f64 * 0.3 + pixel[1] as f64 * 0.59 + pixel[2] as f64 * 0.11;
        let gray = gray as u8;
        *pixel = image::Rgb([gray, gray, gray]);
    }

    img.save(work_path)?;

    Ok(())
}

pub fn to_sepia_image(original_path: &str, work_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(work_path).exists() {
        fs::remove_file(work_path)?;
    }

    let img = image::open(original_path)?;
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

    img.save(work_path)?;

    Ok(())
}

pub fn to_invert_array(pixels: &Vec<u8>) -> Vec<u8> {
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

pub fn to_grayscale_array(pixels: &Vec<u8>) -> Vec<u8> {
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

pub fn to_sepia_array(pixels: &Vec<u8>) -> Vec<u8> {
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

pub fn to_mosaic_array(pixels: &Vec<u8>, height: u32, width: u32, area: u32) -> Vec<u8> {
    let mut result = pixels.to_vec();

    if pixels.len() != height as usize * width as usize * 4 {
        return result;
    }

    for y in (0..height).step_by(area as usize) {
        if y + area > height { break; }

        for x in (0..width).step_by(area as usize) {
            if x + area > width { break; }

            to_mosaic_in_area(pixels, &mut result, &SubArea::new(y, x, width, area, area));
        }

        if width % area != 0 {
            let rm = width % area;

            to_mosaic_in_area(pixels, &mut result, &SubArea::new(y, width - rm, width, area, rm));
        }
    }

    if height % area != 0 {
        let rm = height % area;

        for x in (0..width).step_by(area as usize) {
            if x + area > width { break; }

            to_mosaic_in_area(
                pixels,
                &mut result,
                &SubArea::new(height - rm, x, width, rm, area)
            );
        }

        if width % area != 0 {
            let rm_w = width % area;

            to_mosaic_in_area(
                pixels,
                &mut result,
                &SubArea::new(height - rm, width - rm_w, width, rm, rm_w)
            );
        }
    }

    result
}

fn to_mosaic_in_area(pixels: &Vec<u8>, result: &mut Vec<u8>, sub_area: &SubArea) {
    let mut acc_r: u32 = 0;
    let mut acc_g: u32 = 0;
    let mut acc_b: u32 = 0;

    calc_total_in_area(pixels, sub_area, &mut acc_r, &mut acc_g, &mut acc_b);

    let r = calc_pixel_average(acc_r, sub_area.height * sub_area.width); 
    let g = calc_pixel_average(acc_g, sub_area.height * sub_area.width); 
    let b = calc_pixel_average(acc_b, sub_area.height * sub_area.width); 

    set_pixel_in_area(result, sub_area, r as u8, g as u8, b as u8);
}

fn calc_total_in_area(
    pixels: &Vec<u8>, 
    sub_area: &SubArea,
    acc_r: &mut u32, 
    acc_g: &mut u32, 
    acc_b: &mut u32
) {
    for ya in 0..sub_area.height {
        let row_s = (sub_area.row + ya) * sub_area.row_length * 4;

        for xa in 0..sub_area.width {
            let cp = (row_s + (sub_area.col + xa) * 4) as usize;

            *acc_r += pixels[cp] as u32;
            *acc_g += pixels[cp + 1] as u32;
            *acc_b += pixels[cp + 2] as u32;
        }
    }
}

fn set_pixel_in_area(pixels: &mut Vec<u8>, sub_area: &SubArea, r: u8, g: u8, b: u8) {
    for ya in 0..sub_area.height {
        let row_s = (sub_area.row + ya) * sub_area.row_length * 4;

        for xa in 0..sub_area.width {
            let cp = (row_s + (sub_area.col + xa) * 4) as usize;

            pixels[cp] = r;
            pixels[cp + 1] = g;
            pixels[cp + 2] = b;
        }
    }
}

fn calc_luminance_array(pixels: &Vec<u8>) -> Vec<i16> {
    let mut result = Vec::with_capacity(pixels.len() / 4);

    for i in (0..pixels.len()).step_by(4) {
        let lmn = pixels[i] as f64 * 0.299
            + pixels[i + 1] as f64 * 0.587
            + pixels[i + 2] as f64 * 0.114;

        result.push(lmn as i16);
    }

    result
}

fn calc_blue_chrominance_array(pixels: &Vec<u8>) -> Vec<i16> {
    let mut result = Vec::with_capacity(pixels.len() / 4);

    for i in (0..pixels.len()).step_by(4) {
        let cb = pixels[i] as f64 * -0.167
            + pixels[i + 1] as f64 * -0.3313
            + pixels[i + 2] as f64 * 0.5
            + 128.0;

        result.push(cb as i16);
    }

    result
}

fn calc_red_chrominance_array(pixels: &Vec<u8>) -> Vec<i16> {
    let mut result = Vec::with_capacity(pixels.len() / 4);

    for i in (0..pixels.len()).step_by(4) {
        let cr = pixels[i] as f64 * 0.5
            + pixels[i + 1] as f64 * -0.4187
            + pixels[i + 2] as f64 * -0.0813
            + 128.0;

        result.push(cr as i16);
    }

    result
}

pub fn to_smoothing_array(pixels: &Vec<u8>, height: u32, width: u32) -> Vec<u8> {
    let mut result = pixels.to_vec();

    if pixels.len() != height as usize * width as usize * 4 {
        println!("len={}, height={}, width={}", pixels.len(), height, width);
        return result;
    }

    let lmn_ary = calc_luminance_array(pixels);
    let cb_ary = calc_blue_chrominance_array(pixels);
    let cr_ary = calc_red_chrominance_array(pixels);

    let mut smoothing_ary = lmn_ary.to_vec();

    for y in 1..height as usize - 1{
        let yp = (y - 1) * width as usize;
        let yn = (y + 1) * width as usize;

        for x in 1..width as usize - 1 {
            let mut acc = 0;
            acc += lmn_ary[yp + x - 1];
            acc += lmn_ary[yp + x];
            acc += lmn_ary[yp + x + 1];
            acc += lmn_ary[y + x - 1];
            acc += lmn_ary[y + x];
            acc += lmn_ary[y + x + 1];
            acc += lmn_ary[yn + x - 1];
            acc += lmn_ary[yn + x];
            acc += lmn_ary[yn + x + 1];

            let lmn = acc / 9;
            smoothing_ary[y + x] = if lmn > 255 { 255 } else { lmn };
        }
    }

    for i in (0..pixels.len()).step_by(4) {
        let j = i / 4;

        let y = smoothing_ary[j];
        let cb = cb_ary[j] - 128;
        let cr = cr_ary[j] - 128;

        let r = (y as f64 + 45.0 / 32.0 * cr as f64).clamp(0.0, 255.0) as u8;
        let g = (y as f64 - 11.0 / 32.0 * cb as f64 - 23.0 / 32.0 * cr as f64)
            .clamp(0.0, 255.0) as u8;
        let b = (y as f64 + 113.0 / 64.0 * cb as f64).clamp(0.0, 255.0) as u8;

        result[i] = r;
        result[i + 1] = g;
        result[i + 2] = b;
    }

    result
}
