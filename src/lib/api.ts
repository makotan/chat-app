import { invoke } from "@tauri-apps/api/core";
import type { ChatSession, Message, Config } from './types';

// MCP関連
export async function initializeMcp(apiKey: string): Promise<string> {
  return invoke('initialize_mcp', { apiKey });
}

export async function sendMessage(content: string, sessionId: string): Promise<string> {
  return invoke('send_message', { content, sessionId });
}

// データベース関連
export async function createChatSession(title: string): Promise<string> {
  return invoke('create_chat_session', { title });
}

export async function getChatSessions(): Promise<ChatSession[]> {
  return invoke('get_chat_sessions');
}

export async function getChatMessages(sessionId: string): Promise<Message[]> {
  return invoke('get_chat_messages', { sessionId });
}

export async function addChatMessage(
  sessionId: string,
  role: 'user' | 'assistant',
  content: string
): Promise<string> {
  return invoke('add_chat_message', { sessionId, role, content });
}

export async function deleteChatSession(sessionId: string): Promise<void> {
  return invoke('delete_chat_session', { sessionId });
}

// 設定関連
export async function getConfig(): Promise<Config> {
  return invoke('get_config');
}

export async function saveConfig(config: Config): Promise<void> {
  return invoke('save_config_command', { config });
}

// エクスポート/インポート関連
export async function exportChatHistory(): Promise<string> {
  return invoke('export_chat_history');
}

export async function importChatHistory(): Promise<string> {
  return invoke('import_chat_history');
}