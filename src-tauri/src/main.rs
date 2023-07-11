// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::{path::Path, sync::mpsc::channel};
use tauri::State;
use tauri::{Manager, Window};

struct WatchState {
    watcher: Mutex<RecommendedWatcher>,
    rx: Arc<Mutex<Receiver<Event>>>,
    current_path: Mutex<Option<String>>,
}
// 監視するファイルの種類
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum WatchType {
    Xml,
    Xsl,
    Css,
}
type WatchStates = HashMap<WatchType, WatchState>;
fn create_watcher() -> (RecommendedWatcher, Receiver<Event>) {
    let (tx, rx) = channel();
    let config = Config::default()
        .with_poll_interval(std::time::Duration::from_secs(20))
        .with_compare_contents(true);
    let watcher = RecommendedWatcher::new(
        move |res: Result<Event, _>| match res {
            Ok(event) => {
                tx.send(event.to_owned()).unwrap();
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        },
        config,
    )
    .unwrap();
    (watcher, rx)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            watch_file,
            open_file,
            unwatch_file,
        ])
        .setup(|app| {
            let (xml_watcher, xml_rx) = create_watcher();
            let (xsl_watcher, xsl_rx) = create_watcher();
            let (css_watcher, css_rx) = create_watcher();
            app.manage(HashMap::from([
                (
                    WatchType::Xml,
                    WatchState {
                        watcher: Mutex::new(xml_watcher),
                        rx: Arc::new(Mutex::new(xml_rx)),
                        current_path: Mutex::new(None),
                    },
                ),
                (
                    WatchType::Xsl,
                    WatchState {
                        watcher: Mutex::new(xsl_watcher),
                        rx: Arc::new(Mutex::new(xsl_rx)),
                        current_path: Mutex::new(None),
                    },
                ),
                (
                    WatchType::Css,
                    WatchState {
                        watcher: Mutex::new(css_watcher),
                        rx: Arc::new(Mutex::new(css_rx)),
                        current_path: Mutex::new(None),
                    },
                ),
            ]));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// ファイルの内容を読み込む
/// * `path` - ファイルのパス
/// * `Result<String,String>` - ファイルの内容
fn read_file(path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(content)
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    println!("open: {}", &path);
    let output = std::process::Command::new("cmd")
        .args(&["/C", "start", &path])
        .output();

    if let Err(e) = output {
        return Err(e.to_string());
    }
    Ok(())
}

#[derive(Serialize, Clone)]
struct ChangeFilePayload {
    content: String,
}
#[tauri::command]
async fn watch_file(
    window: Window,
    state: State<'_, WatchStates>,
    path: String,
    title: String,
) -> Result<(), String> {
    if path_exists(path.to_owned().as_str()) == false {
        return Err("file not found".to_owned());
    }
    let ext = Path::new(&path).extension().unwrap().to_str().unwrap();
    let watcher_type = match ext {
        "xml" => WatchType::Xml,
        "xsl" => WatchType::Xsl,
        "css" => WatchType::Css,
        _ => return Err("invalid file extension".to_owned()),
    };
    let mut watcher = state.get(&watcher_type).unwrap().watcher.lock().unwrap();
    let arx = state.get(&watcher_type).unwrap().rx.clone();
    let mut current_path = state
        .get(&watcher_type)
        .unwrap()
        .current_path
        .lock()
        .unwrap();

    // 既に監視しているファイルと同じ場合は何もしない
    if let Some(cur) = current_path.as_ref() {
        if cur == &path {
            return Ok(());
        }
        // 既に監視しているファイルがある場合は監視を解除する
        watcher.unwatch(Path::new(cur)).map_err(|err| {
            println!("unwatch error: {:?}", err);
            err.to_string()
        })?;
    }

    // 対象のファイルを監視する
    watcher
        .watch(Path::new(&path), RecursiveMode::NonRecursive)
        .unwrap();

    // 監視対象のファイルパスを更新
    *current_path = Some(path.to_owned());

    // ファイルの変更を通知する
    window
        .emit_all(
            &format!("change_{}", title.to_owned()),
            read_file(path.to_owned())?,
        )
        .unwrap();

    std::thread::spawn(move || {
        // 監視イベントを受け取る
        let rx = arx.lock().unwrap();
        while let Ok(event) = rx.recv() {
            if let EventKind::Modify(_) = event.kind {
                println!("event: {:?}", event.paths);
                window
                    .emit_all(
                        &format!("change_{}", title.to_owned()),
                        read_file(path.to_owned()).unwrap(),
                    )
                    .unwrap();
            }
        }
    });
    Ok(())
}

/// 指定したパスが存在するかどうかを返す
fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}
#[tauri::command]
fn unwatch_file(state: State<'_, WatchStates>, ext: String) -> Result<(), String> {
    let watcher_type = match ext.as_str() {
        "xml" => WatchType::Xml,
        "xsl" => WatchType::Xsl,
        "css" => WatchType::Css,
        _ => return Err("invalid file extension".to_owned()),
    };
    let watch_state = state.get(&watcher_type).unwrap();
    let mut watcher = watch_state.watcher.lock().unwrap();
    if let Some(path) = &*watch_state.current_path.lock().unwrap() {
        watcher
            .unwatch(Path::new(path))
            .map_err(|err| err.to_string())?;
    }
    
    // 監視対象のファイルパスをNoneにする
    *watch_state.current_path.lock().unwrap() = None;

    Ok(())
}
