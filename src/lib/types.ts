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
}