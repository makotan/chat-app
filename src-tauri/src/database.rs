use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;
use uuid::Uuid;
use chrono::Utc;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

// エクスポート/インポート用のデータ構造
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub sessions: Vec<ChatSession>,
    pub messages: Vec<Message>,
    pub version: String,
    #[serde(rename = "exportedAt")]
    pub exported_at: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self> {
        let app_data_dir = app_handle.path().app_data_dir().unwrap();
        std::fs::create_dir_all(&app_data_dir)
            .map_err(|e| rusqlite::Error::InvalidPath(PathBuf::from(e.to_string())))?;
        
        let db_path = app_data_dir.join("chat_history.db");
        let conn = Connection::open(db_path)?;
        
        // テーブル作成
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chat_sessions (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                FOREIGN KEY (session_id) REFERENCES chat_sessions(id)
            )",
            [],
        )?;
        
        Ok(Self { conn })
    }

    // チャット履歴をエクスポートする関数
    pub fn export_data(&self, file_path: &str) -> std::result::Result<(), String> {
        // すべてのセッションを取得
        let sessions = self.get_sessions()
            .map_err(|e| format!("Failed to get sessions: {}", e))?;
        
        // すべてのメッセージを取得
        let mut all_messages = Vec::new();
        for session in &sessions {
            let messages = self.get_messages(&session.id)
                .map_err(|e| format!("Failed to get messages for session {}: {}", session.id, e))?;
            all_messages.extend(messages);
        }
        
        // エクスポートデータを作成
        let export_data = ExportData {
            sessions,
            messages: all_messages,
            version: "1.0".to_string(),
            exported_at: Utc::now().to_rfc3339(),
        };
        
        // JSONに変換
        let json = serde_json::to_string_pretty(&export_data)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;
        
        // ファイルに書き込み
        fs::write(file_path, json)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }
    
    // チャット履歴をインポートする関数
    pub fn import_data(&mut self, file_path: &str) -> std::result::Result<(), String> {
        // ファイルを読み込み
        let json = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        // JSONをパース
        let import_data: ExportData = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        // トランザクションを開始
        let tx = self.conn.transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;
        
        // セッションをインポート
        for session in &import_data.sessions {
            tx.execute(
                "INSERT OR REPLACE INTO chat_sessions (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)",
                params![session.id, session.title, session.created_at, session.updated_at],
            ).map_err(|e| format!("Failed to insert session: {}", e))?;
        }
        
        // メッセージをインポート
        for message in &import_data.messages {
            tx.execute(
                "INSERT OR REPLACE INTO messages (id, session_id, role, content, timestamp) VALUES (?, ?, ?, ?, ?)",
                params![message.id, message.session_id, message.role, message.content, message.timestamp],
            ).map_err(|e| format!("Failed to insert message: {}", e))?;
        }
        
        // トランザクションをコミット
        tx.commit().map_err(|e| format!("Failed to commit transaction: {}", e))?;
        
        Ok(())
    }
    
    pub fn create_session(&self, title: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO chat_sessions (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)",
            params![id, title, now, now],
        )?;
        
        Ok(id)
    }
    
    pub fn add_message(&self, session_id: &str, role: &str, content: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO messages (id, session_id, role, content, timestamp) VALUES (?, ?, ?, ?, ?)",
            params![id, session_id, role, content, now],
        )?;
        
        // セッションの更新日時を更新
        self.conn.execute(
            "UPDATE chat_sessions SET updated_at = ? WHERE id = ?",
            params![now, session_id],
        )?;
        
        Ok(id)
    }
    
    pub fn get_sessions(&self) -> Result<Vec<ChatSession>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, created_at, updated_at FROM chat_sessions ORDER BY updated_at DESC"
        )?;
        
        let sessions = stmt.query_map([], |row| {
            Ok(ChatSession {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;
        
        Ok(sessions)
    }
    
    pub fn get_messages(&self, session_id: &str) -> Result<Vec<Message>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, role, content, timestamp FROM messages
             WHERE session_id = ? ORDER BY timestamp ASC"
        )?;
        
        let messages = stmt.query_map(params![session_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                session_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;
        
        Ok(messages)
    }
    
    pub fn delete_session(&mut self, session_id: &str) -> Result<()> {
        // トランザクションを開始
        let tx = self.conn.transaction()?;
        
        // まず関連するメッセージを削除
        tx.execute(
            "DELETE FROM messages WHERE session_id = ?",
            params![session_id],
        )?;
        
        // 次にセッション自体を削除
        tx.execute(
            "DELETE FROM chat_sessions WHERE id = ?",
            params![session_id],
        )?;
        
        // トランザクションをコミット
        tx.commit()?;
        
        Ok(())
    }
}