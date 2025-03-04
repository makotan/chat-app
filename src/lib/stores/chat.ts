import { writable } from 'svelte/store';
import type { ChatSession, Message } from '../types';

interface ChatStore {
  currentSessionId: string;
  sessions: ChatSession[];
  messages: Record<string, Message[]>;
}

const initialState: ChatStore = {
  currentSessionId: '',
  sessions: [],
  messages: {},
};

export const chatStore = writable<ChatStore>(initialState);