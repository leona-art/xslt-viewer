// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify::{
    recommended_watcher, Config, PollWatcher, RecommendedWatcher, RecursiveMode, Watcher,
    WatcherKind,
};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use tauri::Manager;

#[derive(Debug, Clone, Deserialize)]
struct StartDetectPayload {
    path: String,
}
#[derive(Debug, Clone, Serialize)]
struct DetectChangePayload {
    content: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("start_detection", |event| {
                if let Some(payload) = event.payload() {
                    if let Ok(payload) = from_str::<StartDetectPayload>(payload) {
                        let p = Path::new(&payload.path);
                        if p.exists() {
                            let (tx, rx) = channel();
                            // watch some stuff
                            let config = Config::default()
                                .with_poll_interval(Duration::from_secs(2))
                                .with_compare_contents(true);

                            let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind()
                                == WatcherKind::PollWatcher
                            {
                                // custom config for PollWatcher kind
                                // you
                                let config =
                                    Config::default().with_poll_interval(Duration::from_secs(1));
                                Box::new(PollWatcher::new(tx, config).unwrap())
                            } else {
                                // use default config for everything else
                                Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
                            };
                            watcher.watch(p, RecursiveMode::Recursive).unwrap();

                            for e in rx {
                                println!("{:?}", e);
                            }
                        };
                    }
                };
            });
            Ok(())
        })
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
