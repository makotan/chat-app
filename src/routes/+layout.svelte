<script lang="ts">
  import { onMount } from 'svelte';
  import { settingsStore } from '$lib/stores/settings';
  import { getConfig } from '$lib/api';
  
  let theme = 'light';
  
  // 設定からテーマを読み込む
  onMount(async () => {
    try {
      const config = await getConfig();
      settingsStore.set(config);
      theme = config.theme;
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  });
  
  // settingsStoreの変更を監視
  $: theme = $settingsStore.theme;
</script>

<!-- テーマクラスをHTML要素に適用 -->
<div class="app-container {theme}">
  <slot />
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