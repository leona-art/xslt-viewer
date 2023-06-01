// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// ファイル内の文字列を返す
#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    let contents = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(contents)
}
