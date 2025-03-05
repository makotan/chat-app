<script lang="ts">
  import { onMount } from 'svelte';
  import { settingsStore } from '$lib/stores/settings';
  import { getConfig, saveConfig, initializeMcp } from '$lib/api';
  import type { Config } from '$lib/types';
  
  let config: Config = {
    apiKey: '',
    model: 'claude-3-opus-20240229',
    theme: 'light',
    maxHistory: 100,
    autoCreateChat: true
  };
  
  let isSaving = false;
  let saveMessage = '';
  
  onMount(async () => {
    try {
      config = await getConfig();
      settingsStore.set(config);
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  });
  
  async function handleSubmit() {
    isSaving = true;
    saveMessage = '';
    
    try {
      await saveConfig(config);
      settingsStore.set(config);
      
      // API Keyが設定されている場合、MCPクライアントを初期化
      if (config.apiKey) {
        await initializeMcp(config.apiKey);
      }
      
      saveMessage = '設定を保存しました';
    } catch (error) {
      console.error('Failed to save config:', error);
      saveMessage = `エラー: ${error}`;
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="settings-container">
  <h1>設定</h1>
  
  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="apiKey">Claude API Key</label>
      <input
        type="password"
        id="apiKey"
        bind:value={config.apiKey}
        placeholder="sk-..."
      />
    </div>
    
    <div class="form-group">
      <label for="model">モデル</label>
      <select id="model" bind:value={config.model}>
        <option value="claude-3-opus-20240229">Claude 3 Opus</option>
        <option value="claude-3-sonnet-20240229">Claude 3 Sonnet</option>
        <option value="claude-3-haiku-20240307">Claude 3 Haiku</option>
      </select>
    </div>
    
    <div class="form-group">
      <label for="theme">テーマ</label>
      <select id="theme" bind:value={config.theme}>
        <option value="light">ライト</option>
        <option value="dark">ダーク</option>
      </select>
    </div>
    <div class="form-group">
      <label for="maxHistory">履歴の最大保存数</label>
      <input
        type="number"
        id="maxHistory"
        bind:value={config.maxHistory}
        min="10"
        max="1000"
      />
    </div>
    
    <div class="form-group">
      <label for="autoCreateChat">新規チャットの自動作成</label>
      <div class="checkbox-container">
        <input
          type="checkbox"
          id="autoCreateChat"
          bind:checked={config.autoCreateChat}
        />
        <span class="checkbox-label">アプリ起動時に新規チャットを自動作成する</span>
      </div>
    </div>
    
    <button type="submit" disabled={isSaving}>
      {isSaving ? '保存中...' : '保存'}
    </button>
    
    {#if saveMessage}
      <div class="save-message">{saveMessage}</div>
    {/if}
  </form>
  
  <div class="back-link">
    <a href="/">← チャットに戻る</a>
  </div>
</div>

<style>
  .settings-container {
    max-width: 600px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  h1 {
    margin-bottom: 2rem;
  }
  
  .form-group {
    margin-bottom: 1.5rem;
  }
  
  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: bold;
  }
  
  input, select {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 0.25rem;
  }
  
  button {
    padding: 0.5rem 1rem;
    background-color: #2196f3;
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
  }
  
  button:disabled {
    background-color: #bdbdbd;
    cursor: not-allowed;
  }
  
  .save-message {
    margin-top: 1rem;
    padding: 0.5rem;
    border-radius: 0.25rem;
    background-color: #e3f2fd;
  }
  
  .back-link {
    margin-top: 2rem;
  }
  
  .back-link a {
    color: #2196f3;
    text-decoration: none;
  }
  
  .back-link a:hover {
    text-decoration: underline;
  }
  
  .checkbox-container {
    display: flex;
    align-items: center;
  }
  
  .checkbox-container input[type="checkbox"] {
    width: auto;
    margin-right: 0.5rem;
  }
  
  .checkbox-label {
    font-size: 0.9rem;
  }
</style>