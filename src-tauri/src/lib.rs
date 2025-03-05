mod mcp;
mod database;
mod config;

use mcp::McpClient;
use database::Database;
use config::{Config, load_config, save_config};
use tauri::{State, Manager};
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;

struct AppState {
    mcp_client: Mutex<Option<McpClient>>,
    database: Mutex<Option<Database>>,
    config: Mutex<Config>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn initialize_mcp(
    api_key: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let base_url = "https://api.anthropic.com".to_string();
    
    // 設定から現在のモデルを取得
    let model = {
        let config = state.config.lock().unwrap();
        config.model.clone()
    };
    
    let mcp_client = McpClient::new(api_key, base_url, model);
    
    *state.mcp_client.lock().unwrap() = Some(mcp_client);
    
    Ok("MCP client initialized".to_string())
}

#[tauri::command]
async fn send_message(
    content: String,
    session_id: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // 非同期処理を実行する前に、クライアントが初期化されているか確認
    {
        let guard = state.mcp_client.lock().unwrap();
        if guard.is_none() {
            return Err("MCP client not initialized".to_string());
        }
    }
    
    // 現在のセッションの過去のメッセージを取得
    let mut messages = Vec::new();
    {
        let database_guard = state.database.lock().unwrap();
        let database = database_guard.as_ref().ok_or("Database not initialized")?;
        
        // 過去のメッセージを取得（最大10件）
        let past_messages = database.get_messages(&session_id)
            .map_err(|e| e.to_string())?;
        
        // 過去のメッセージをMCP形式に変換
        for msg in past_messages.iter().rev().take(10).rev() {
            messages.push(mcp::Message {
                role: msg.role.clone(),
                content: msg.content.clone(),
            });
        }
    }
    
    // 新しいユーザーメッセージを追加
    messages.push(mcp::Message {
        role: "user".to_string(),
        content: content.clone(),
    });
    
    // 各メッセージを個別に処理する関数を作成
    let response = process_message_with_mcp(&state, messages).await?;
    
    Ok(response)
}

// MutexGuardの問題を回避するためのヘルパー関数
async fn process_message_with_mcp(
    state: &State<'_, AppState>,
    messages: Vec<mcp::Message>,
) -> Result<String, String> {
    // MutexGuardからクライアントの情報をコピーして、すぐに解放する
    let (api_key, base_url, model) = {
        let guard = state.mcp_client.lock().unwrap();
        let client = guard.as_ref().ok_or("MCP client not initialized")?;
        
        // ゲッターメソッドを使用して必要な情報をコピー
        (
            client.get_api_key(),
            client.get_base_url(),
            client.get_model()
        )
    };
    
    // 静的メソッドを使用して、MutexGuardなしでメッセージを送信
    mcp::McpClient::send_message_static(api_key, base_url, model, messages)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn create_chat_session(title: String, state: State<'_, AppState>) -> Result<String, String> {
    let database_guard = state.database.lock().unwrap();
    let database = database_guard.as_ref().ok_or("Database not initialized")?;
    
    database.create_session(&title)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_chat_sessions(state: State<'_, AppState>) -> Result<Vec<database::ChatSession>, String> {
    let database_guard = state.database.lock().unwrap();
    let database = database_guard.as_ref().ok_or("Database not initialized")?;
    
    database.get_sessions()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_chat_messages(session_id: String, state: State<'_, AppState>) -> Result<Vec<database::Message>, String> {
    let database_guard = state.database.lock().unwrap();
    let database = database_guard.as_ref().ok_or("Database not initialized")?;
    
    database.get_messages(&session_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_chat_message(session_id: String, role: String, content: String, state: State<'_, AppState>) -> Result<String, String> {
    let database_guard = state.database.lock().unwrap();
    let database = database_guard.as_ref().ok_or("Database not initialized")?;
    
    database.add_message(&session_id, &role, &content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_chat_session(session_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut database_guard = state.database.lock().unwrap();
    let database = database_guard.as_mut().ok_or("Database not initialized")?;
    
    database.delete_session(&session_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    let config = state.config.lock().unwrap().clone();
    Ok(config)
}

#[tauri::command]
fn save_config_command(config: Config, state: State<'_, AppState>, app_handle: tauri::AppHandle) -> Result<(), String> {
    *state.config.lock().unwrap() = config.clone();
    save_config(&app_handle, &config)
}

#[tauri::command]
fn export_chat_history(state: State<'_, AppState>, app_handle: tauri::AppHandle) -> Result<String, String> {
    // ファイル保存ダイアログを表示
    let file_path = std::sync::Arc::new(std::sync::Mutex::new(None));
    let file_path_clone = file_path.clone();
    
    app_handle.dialog()
        .file()
        .add_filter("JSON", &["json"])
        .save_file(move |path| {
            if let Ok(mut guard) = file_path_clone.lock() {
                *guard = path;
            }
        });
    
    // ダイアログの結果を待つための短い遅延
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // ファイルパスを取得
    let path_option = file_path.lock().map_err(|e| format!("ミューテックスのロックに失敗しました: {}", e))?;
    
    // ファイルパスが選択されなかった場合
    if path_option.is_none() {
        return Err("エクスポートがキャンセルされました。".to_string());
    }
    
    let file_path = path_option.as_ref().unwrap().to_string();
    
    // データベースからエクスポート
    let database_guard = state.database.lock().unwrap();
    let database = database_guard.as_ref().ok_or("データベースが初期化されていません")?;
    
    database.export_data(&file_path)
        .map(|_| format!("チャット履歴を正常にエクスポートしました: {}", file_path))
}
#[tauri::command]
fn import_chat_history(state: State<'_, AppState>, app_handle: tauri::AppHandle) -> Result<String, String> {
    // ファイル選択ダイアログを表示
    let file_path = std::sync::Arc::new(std::sync::Mutex::new(None));
    let file_path_clone = file_path.clone();
    
    app_handle.dialog()
        .file()
        .add_filter("JSON", &["json"])
        .pick_file(move |path| {
            if let Ok(mut guard) = file_path_clone.lock() {
                *guard = path;
            }
        });
    
    // ダイアログの結果を待つための短い遅延
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // ファイルパスを取得
    let path_option = file_path.lock().map_err(|e| format!("ミューテックスのロックに失敗しました: {}", e))?;
    
    // ファイルパスが選択されなかった場合
    if path_option.is_none() {
        return Err("インポートがキャンセルされました。".to_string());
    }
    
    let file_path = path_option.as_ref().unwrap().to_string().to_string();
    
    // データベースにインポート
    let mut database_guard = state.database.lock().unwrap();
    let database = database_guard.as_mut().ok_or("データベースが初期化されていません")?;
    
    database.import_data(&file_path)
        .map(|_| format!("チャット履歴を正常にインポートしました: {}", file_path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            mcp_client: Mutex::new(None),
            database: Mutex::new(None),
            config: Mutex::new(Config::default()),
        })
        .setup(|app| {
            // アプリケーション初期化処理
            let app_handle = app.handle();
            
            // 設定の読み込み
            let config = load_config(&app_handle);
            *app.state::<AppState>().config.lock().unwrap() = config;
            
            // データベースの初期化
            match Database::new(&app_handle) {
                Ok(database) => {
                    *app.state::<AppState>().database.lock().unwrap() = Some(database);
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {}", e);
                }
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            initialize_mcp,
            send_message,
            create_chat_session,
            get_chat_sessions,
            get_chat_messages,
            add_chat_message,
            delete_chat_session,
            get_config,
            save_config_command,
            export_chat_history,
            import_chat_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
