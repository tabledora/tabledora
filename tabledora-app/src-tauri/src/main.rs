// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let app = build_app();
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::MainEventsCleared => {}
        _ => {
            dbg!(event);
        }
    });
}

fn build_app() -> tauri::App {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_children, open_local_folder])
        .setup(|app| {
            tauri::WindowBuilder::new(
                app,
                "workspace-0",
                tauri::WindowUrl::App(std::path::PathBuf::from("index.html")),
            )
            .title("Tabledora")
            .build()?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
}

#[tauri::command]
async fn list_children(root: String) -> Vec<tauri::api::dir::DiskEntry> {
    tauri::api::dir::read_dir(root, true).unwrap()
}

#[tauri::command]
async fn open_local_folder() -> Option<String> {
    match tauri::api::dialog::blocking::FileDialogBuilder::new().pick_folder() {
        Some(path) => Some(path.to_string_lossy().to_string()),
        None => None,
    }
}
