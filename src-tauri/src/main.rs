#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        img_invert,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn img_invert(pixels: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    let mut idx = 0;
    loop {
        if idx > pixels.len() - 1 {
            break;
        }
        result.push(255 - pixels[idx]);
        result.push(255 - pixels[idx + 1]);
        result.push(255 - pixels[idx + 2]);
        idx += 4;
    }

    result
}
