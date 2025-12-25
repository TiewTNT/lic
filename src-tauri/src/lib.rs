use std::ffi::OsStr;

use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

fn vips_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let subpath = if cfg!(target_os = "windows") {
        "resources/bin/windows/bin/vips.exe"
    } else if cfg!(target_os = "macos") {
        "resources/bin/macos/bin/vips"
    } else {
        "resources/bin/windows/bin/vips.exe"
    };

    app.path()
        .resolve(subpath, BaseDirectory::Resource)
        .map_err(|e| e.to_string())
}

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn vips_command(app: &AppHandle) -> Command {
    let mut cmd = Command::new(vips_path(app).unwrap());

    #[cfg(windows)]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.stdout(Stdio::null())
       .stderr(Stdio::null());

    cmd
}

// uses crate [image = "0.25.9"]
// #[tauri::command]
// fn convert_image_2(str_path: String, img_format: String) {
//     let path = Path::new(&str_path);
//     let dir = path.parent().unwrap();
//     let img = ImageReader::open(&str_path).unwrap().decode().unwrap();
//     let _ = img.save(dir.join(Path::new(&(
//         path.file_stem().unwrap().to_string_lossy().to_string()
//             + &'.'.to_string()
//             + &img_format.to_string())
//     )));
// }

#[tauri::command]
fn convert_image(app: AppHandle, str_path: String, img_format: String, dpi: Option<i32>) -> bool {
    let path = Path::new(&str_path);
    let dir = path.parent().unwrap();

    let binding = dir.join(Path::new(
        &(path.file_stem().unwrap().to_string_lossy().to_string()
            + &'.'.to_string()
            + &img_format.to_string()),
    ));
    let output_path = binding.to_string_lossy();
    println!("{:?}", path);
    println!("{:?}", output_path);
    println!(
        "{:?}",
        vips_path(&app).unwrap().to_string_lossy().to_string()
    );

    match path
        .extension()
        .unwrap_or_else(|| OsStr::new(".png"))
        .to_str()
        .unwrap_or_else(|| ".png")
    {
        "pdf" => {
            let mut page: usize = 0;
            let mut has_ok = false;
            loop {
                let output = {
                    let mut name = binding
                        .file_stem()
                        .unwrap_or_else(|| OsStr::new("output"))
                        .to_os_string();
                    name.push(format!("_{:03}", page));
                    name.push(".");
                    name.push(binding.extension().unwrap_or_else(|| OsStr::new("png")));
                    binding.with_file_name(name)
                };

                let input_spec = format!("{}[page={},dpi={}]", str_path, page, dpi.unwrap_or(300));

                if input_spec.as_str() != output.to_string_lossy().as_ref() {
                    if output.exists() {
                        if !app.dialog()
                            .message(format!("{} already exists. Overwrite?", output.to_string_lossy().as_ref()))
                            .title("File exists")
                            .buttons(MessageDialogButtons::OkCancel)
                            .blocking_show() {
                                continue;
                            };

                    }
                    let status = vips_command(&app)
                        .args([
                            "copy",
                            input_spec.as_str(),
                            output.to_string_lossy().as_ref(),
                        ])
                        .status();

                    match status {
                        Ok(s) if s.success() => {
                            page += 1;
                            has_ok = true;
                        }
                        _ => {
                            // page out of range → we're done
                            break;
                        }
                    }
                }
            }
            return has_ok;
        }
        "svg" => {
            println!("{}", dpi.unwrap_or_else(|| 300));
            let output = {
                let mut name = binding
                    .file_stem()
                    .unwrap_or_else(|| OsStr::new("output"))
                    .to_os_string();
                name.push(".");
                name.push(binding.extension().unwrap_or_else(|| OsStr::new("png")));
                binding.with_file_name(name)
            };

            if str_path.as_str() != output.to_string_lossy().as_ref() {
                if output.exists() {
                    if !app.dialog()
                        .message(format!("{} already exists. Overwrite?", output.to_string_lossy().as_ref()))
                        .title("File exists")
                        .buttons(MessageDialogButtons::OkCancel)
                        .blocking_show()
                    {
                        return false;
                    };
                }

                let status = vips_command(&app)
                    .args([
                        "copy",
                        (str_path
                            + format!("[dpi={}]", dpi.unwrap_or_else(|| 300).to_string()).as_ref())
                        .as_ref(),
                        output.to_string_lossy().as_ref(),
                    ])
                    .status();

                println!("{:?}", status);
                match status {
                    Ok(s) if s.success() => return true,
                    _ => return false,
                }
            }
        }
        _ => {
            if str_path.as_str() != output_path.as_ref() {
                if binding.exists() {
                    if !app.dialog()
                        .message(format!("{} already exists. Overwrite?", binding.to_string_lossy().as_ref()))
                        .title("File exists")
                        .buttons(MessageDialogButtons::OkCancel)
                        .blocking_show()
                    {
                        return false;
                    };
                }

                let status = vips_command(&app)
                    .args(["copy", str_path.as_ref(), output_path.as_ref()])
                    .status();

                println!("{:?}", status);
                match status {
                    Ok(s) if s.success() => return true,
                    _ => return false,
                }
            }
        }
    };
    return false;
}

#[tauri::command]
fn convert_images(app: AppHandle, str_paths: Vec<String>, img_format: String, dpi: Option<i32>) {
    for str_path in str_paths {
        convert_image(app.clone(), str_path, img_format.clone(), dpi);
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
