export interface ChatSession {
  id: string;
  title: string;
  createdAt: string;
  updatedAt: string;
}

export interface Message {
  id: string;
  sessionId: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: string;
}

export interface Config {
  apiKey: string;
  model: string;
  theme: 'light' | 'dark';
  maxHistory: number;
  autoCreateChat: boolean; // 新規チャットの自動作成を制御
}

export interface ExportData {
  sessions: ChatSession[];
  messages: Message[];
  version: string;
  exportedAt: string;
}