import { writable } from 'svelte/store';
import type { Config } from '../types';

const initialState: Config = {
  apiKey: '',
  model: 'claude-3-opus-20240229',
  theme: 'light',
  maxHistory: 100,
  autoCreateChat: true, // デフォルトでは自動作成を有効にする
};

export const settingsStore = writable<Config>(initialState);