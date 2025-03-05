<script lang="ts">
  import { onMount } from 'svelte';
  import { settingsStore } from '$lib/stores/settings';
  import { getConfig, exportChatHistory, importChatHistory } from '$lib/api';
  import { goto } from '$app/navigation';
  import ShortcutHelp from '$lib/components/ShortcutHelp.svelte';
  
  let theme = 'light';
  let showShortcutHelp = false;
  
  // ショートカットキーのハンドラー
  function handleKeydown(event: KeyboardEvent) {
    // Ctrl/Cmd + キーの組み合わせを検出
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    const modifier = isMac ? event.metaKey : event.ctrlKey;
    
    if (modifier) {
      switch (event.key) {
        case 'n': // 新規チャット
          event.preventDefault();
          goto('/');
          break;
        case 'h': // 履歴
          event.preventDefault();
          goto('/history');
          break;
        case 's': // 設定
          event.preventDefault();
          goto('/settings');
          break;
        case 'e': // エクスポート
          event.preventDefault();
          handleExport();
          break;
        case 'i': // インポート
          event.preventDefault();
          handleImport();
          break;
        case '?': // ショートカットヘルプ
          event.preventDefault();
          showShortcutHelp = !showShortcutHelp;
          break;
      }
    }
  }
  
  // チャット履歴をエクスポートする関数
  async function handleExport() {
    try {
      const result = await exportChatHistory();
      alert(result);
    } catch (error) {
      console.error('Failed to export chat history:', error);
      alert(`エクスポートに失敗しました: ${error}`);
    }
  }
  
  // チャット履歴をインポートする関数
  async function handleImport() {
    try {
      const result = await importChatHistory();
      alert(result);
    } catch (error) {
      console.error('Failed to import chat history:', error);
      alert(`インポートに失敗しました: ${error}`);
    }
  }
  
  // 設定からテーマを読み込む
  onMount(async () => {
    try {
      const config = await getConfig();
      settingsStore.set(config);
      theme = config.theme;
    } catch (error) {
      console.error('Failed to load config:', error);
    }
    
    // グローバルなキーボードイベントリスナーを追加
    window.addEventListener('keydown', handleKeydown);
    
    // クリーンアップ関数を返す
    return () => {
      window.removeEventListener('keydown', handleKeydown);
    };
  });
  
  // settingsStoreの変更を監視
  $: theme = $settingsStore.theme;
</script>

<!-- テーマクラスをHTML要素に適用 -->
<div class="app-container {theme}">
  <slot />
  <ShortcutHelp bind:isOpen={showShortcutHelp} />
</div>

<style>
  :global(body, html) {
    margin: 0;
    padding: 0;
    height: 100%;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  }
  
  :global(#app) {
    height: 100vh;
  }
  
  .app-container {
    height: 100vh;
    width: 100%;
    transition: background-color 0.3s, color 0.3s;
  }
  
  /* ライトテーマ（デフォルト） */
  .light {
    --bg-color: #ffffff;
    --text-color: #333333;
    --sidebar-bg: #f5f5f5;
    --sidebar-border: #dddddd;
    --message-user-bg: #e3f2fd;
    --message-assistant-bg: #f5f5f5;
    --input-border: #dddddd;
    --button-primary: #2196f3;
    --button-danger: #f44336;
    --hover-bg: #e0e0e0;
    --active-bg: #e3f2fd;
    --divider: #eeeeee;
  }
  
  /* ダークテーマ */
  .dark {
    --bg-color: #121212;
    --text-color: #e0e0e0;
    --sidebar-bg: #1e1e1e;
    --sidebar-border: #333333;
    --message-user-bg: #1a3a5a;
    --message-assistant-bg: #2a2a2a;
    --input-border: #444444;
    --button-primary: #1976d2;
    --button-danger: #d32f2f;
    --hover-bg: #2a2a2a;
    --active-bg: #1a3a5a;
    --divider: #333333;
  }
  
  /* グローバルなテーマ変数の適用 */
  :global(.light) {
    background-color: var(--bg-color);
    color: var(--text-color);
  }
  
  :global(.dark) {
    background-color: var(--bg-color);
    color: var(--text-color);
  }
  
  /* 入力フィールドのスタイル */
  :global(.dark input), :global(.dark textarea), :global(.dark select) {
    background-color: #2a2a2a;
    color: #e0e0e0;
    border-color: var(--input-border);
  }
  
  /* ボタンのスタイル */
  :global(.dark button) {
    background-color: var(--button-primary);
  }
  
  :global(.dark .delete-btn) {
    background-color: var(--button-danger);
  }
</style>