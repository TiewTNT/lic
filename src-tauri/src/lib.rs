use image::ImageReader;
use std::path::Path;
use std::process::Command;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

fn vips_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let subpath = if cfg!(target_os = "windows") {
        "resources/bin/windows/bin/vips.exe"
    } else if cfg!(target_os = "macos") {
        "resources/bin/macos/vips"
    } else {
        "resources/bin/windows/bin/vips.exe"
    };

    app.path()
        .resolve(subpath, BaseDirectory::Resource)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn convert_image_2(str_path: String, img_format: String) {
    let path = Path::new(&str_path);
    let dir = path.parent().unwrap();
    let img = ImageReader::open(&str_path).unwrap().decode().unwrap();
    let _ = img.save(dir.join(Path::new(&(
        path.file_stem().unwrap().to_string_lossy().to_string()
            + &'.'.to_string()
            + &img_format.to_string())
    )));
}

#[tauri::command]
fn convert_image(app: AppHandle, str_path: String, img_format: String) {
    let path = Path::new(&str_path);
    let dir = path.parent().unwrap();
    let binding = dir.join(Path::new(&(
        path.file_stem().unwrap().to_string_lossy().to_string()
            + &'.'.to_string()
            + &img_format.to_string())
    ));
    let output_path = binding.to_string_lossy();

    println!("{:?}", path);
    println!("{:?}", output_path);
    println!(
        "{:?}",
        vips_path(&app).unwrap().to_string_lossy().to_string()
    );

    let cmd_status = Command::new(vips_path(&app).unwrap().to_string_lossy().to_string())
        .args(["copy", str_path.as_str(), output_path.to_string().as_str()])
        .status();
    println!("{:?}", cmd_status)
}

#[tauri::command]
fn convert_images(app: AppHandle, str_paths: Vec<String>, img_format: String) {
    for str_path in str_paths {
        convert_image(app.clone(), str_path, img_format.clone());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // get_array,
            // remove_from_array,
            // clear_array,
            // add_images,
            convert_image,
            convert_images
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
