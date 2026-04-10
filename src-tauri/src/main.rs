#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod openai;
mod optimizer;

#[cfg(not(target_os = "macos"))]
const SHORTCUT: &str = "Ctrl+Alt+P";

#[cfg(target_os = "macos")]
const SHORTCUT: &str = "Cmd+Alt+P";

use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::ShortcutState;

#[derive(Clone, serde::Serialize)]
struct StatusPayload {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    original: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optimized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[tauri::command]
fn get_api_key() -> Option<String> {
    optimizer::load_api_key()
}

#[tauri::command]
fn set_api_key(key: String) -> Result<(), String> {
    optimizer::save_api_key(&key)
}

#[tauri::command]
fn delete_api_key() -> Result<(), String> {
    optimizer::delete_api_key()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(SHORTCUT)
                .expect("invalid shortcut")
                .with_handler(|app, _shortcut, event| {
                    if event.state != ShortcutState::Pressed {
                        return;
                    }
                    let app_handle = app.app_handle().clone();
                    tauri::async_runtime::spawn(async move {
                        handle_optimize(app_handle).await;
                    });
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![get_api_key, set_api_key, delete_api_key])
        .setup(|_| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn handle_optimize(app: tauri::AppHandle) {
    let _ = app.emit(
        "optimize-status",
        StatusPayload {
            status: "started".into(),
            original: None,
            optimized: None,
            error: None,
        },
    );

    let clipboard = app.clipboard();

    let original = match clipboard.read_text() {
        Ok(t) if !t.trim().is_empty() => t,
        _ => {
            let _ = app.emit(
                "optimize-status",
                StatusPayload {
                    status: "error".into(),
                    original: None,
                    optimized: None,
                    error: Some("Clipboard is empty".into()),
                },
            );
            return;
        }
    };

    let api_key = match optimizer::load_api_key() {
        Some(k) => k,
        None => {
            let _ = app.emit(
                "optimize-status",
                StatusPayload {
                    status: "error".into(),
                    original: None,
                    optimized: None,
                    error: Some("No API key set. Open the app to configure.".into()),
                },
            );
            return;
        }
    };

    match openai::optimize_prompt(&api_key, &original).await {
        Ok(optimized) => {
            if let Err(e) = clipboard.write_text(&optimized) {
                let _ = app.emit(
                    "optimize-status",
                    StatusPayload {
                        status: "error".into(),
                        original: None,
                        optimized: None,
                        error: Some(format!("Failed to write clipboard: {}", e)),
                    },
                );
                return;
            }
            let _ = app.emit(
                "optimize-status",
                StatusPayload {
                    status: "done".into(),
                    original: Some(original),
                    optimized: Some(optimized),
                    error: None,
                },
            );

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            let _ = simulate_paste();
        }
        Err(e) => {
            let _ = clipboard.write_text(&original);
            let _ = app.emit(
                "optimize-status",
                StatusPayload {
                    status: "error".into(),
                    original: None,
                    optimized: None,
                    error: Some(format!("{}", e)),
                },
            );
        }
    }
}

fn main() {
    run();
}

fn simulate_paste() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use enigo::Key;
    use enigo::Keyboard;
    use enigo::Settings;
    let mut enigo = enigo::Enigo::new(&Settings::default())?;

    #[cfg(not(target_os = "macos"))]
    let modifier = Key::Control;
    #[cfg(target_os = "macos")]
    let modifier = Key::Meta;

    enigo.key(modifier, enigo::Direction::Press)?;
    enigo.key(Key::Unicode('v'), enigo::Direction::Click)?;
    enigo.key(modifier, enigo::Direction::Release)?;
    Ok(())
}
