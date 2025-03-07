<script lang="ts">
  import { onMount } from 'svelte';
  import { chatStore } from '$lib/stores/chat';
  import { getChatSessions, deleteChatSession, exportChatHistory, importChatHistory } from '$lib/api';
  import type { ChatSession } from '$lib/types';
  
  let sessions: ChatSession[] = [];
  let isLoading = true;
  let isDeleting = false;
  let isExporting = false;
  let isImporting = false;
  let statusMessage = '';
  
  onMount(async () => {
    try {
      sessions = await getChatSessions();
      chatStore.update(state => ({
        ...state,
        sessions
      }));
    } catch (error) {
      console.error('Failed to load chat sessions:', error);
    } finally {
      isLoading = false;
    }
  });
  
  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return date.toLocaleString();
  }
  
  function openChat(sessionId: string) {
    chatStore.update(state => ({
      ...state,
      currentSessionId: sessionId
    }));
    window.location.href = '/';
  }
  
  async function handleDelete(sessionId: string, event: MouseEvent) {
    // イベントの伝播を停止（親要素のクリックイベントが発火しないように）
    event.stopPropagation();
    
    if (isDeleting) return;
    
    // 確認ダイアログ
    if (!confirm('このチャットセッションを削除してもよろしいですか？この操作は元に戻せません。')) {
      return;
    }
    
    isDeleting = true;
    
    try {
      // セッションを削除
      await deleteChatSession(sessionId);
      
      // セッションリストから削除
      sessions = sessions.filter(session => session.id !== sessionId);
      
      // chatStoreを更新
      chatStore.update(state => ({
        ...state,
        sessions,
        // 削除したセッションが現在選択されていた場合、選択を解除
        currentSessionId: state.currentSessionId === sessionId ? '' : state.currentSessionId
      }));
    } catch (error) {
      console.error('Failed to delete chat session:', error);
      alert('セッションの削除に失敗しました');
    } finally {
      isDeleting = false;
    }
  }

  // チャット履歴をエクスポートする関数
  async function handleExport() {
    if (isExporting) return;
    
    isExporting = true;
    statusMessage = 'エクスポート中...';
    
    try {
      const result = await exportChatHistory();
      statusMessage = result;
      
      // 成功メッセージを3秒後に消す
      setTimeout(() => {
        if (statusMessage === result) {
          statusMessage = '';
        }
      }, 3000);
    } catch (error) {
      console.error('Failed to export chat history:', error);
      statusMessage = `エクスポートに失敗しました: ${error}`;
    } finally {
      isExporting = false;
    }
  }
  
  // チャット履歴をインポートする関数
  async function handleImport() {
    if (isImporting) return;
    
    isImporting = true;
    statusMessage = 'インポート中...';
    
    try {
      const result = await importChatHistory();
      statusMessage = result;
      
      // セッションリストを更新
      sessions = await getChatSessions();
      chatStore.update(state => ({
        ...state,
        sessions
      }));
      
      // 成功メッセージを3秒後に消す
      setTimeout(() => {
        if (statusMessage === result) {
          statusMessage = '';
        }
      }, 3000);
    } catch (error) {
      console.error('Failed to import chat history:', error);
      statusMessage = `インポートに失敗しました: ${error}`;
    } finally {
      isImporting = false;
    }
  }
</script>

<div class="history-container">
  <h1>チャット履歴</h1>
  
  <div class="actions-bar">
    <button class="export-btn" on:click={handleExport} disabled={isExporting || isImporting}>
      {isExporting ? 'エクスポート中...' : 'エクスポート'}
    </button>
    <button class="import-btn" on:click={handleImport} disabled={isExporting || isImporting}>
      {isImporting ? 'インポート中...' : 'インポート'}
    </button>
  </div>
  
  {#if statusMessage}
    <div class="status-message">{statusMessage}</div>
  {/if}
  
  {#if isLoading}
    <div class="loading">読み込み中...</div>
  {:else if sessions.length === 0}
    <div class="empty">チャット履歴がありません</div>
  {:else}
    <div class="sessions">
      <table>
        <thead>
          <tr>
            <th>タイトル</th>
            <th>作成日時</th>
            <th>更新日時</th>
            <th>アクション</th>
          </tr>
        </thead>
        <tbody>
          {#each sessions as session}
            <tr>
              <td>{session.title}</td>
              <td>{formatDate(session.createdAt)}</td>
              <td>{formatDate(session.updatedAt)}</td>
              <td class="actions">
                <button class="open-btn" on:click={() => openChat(session.id)}>開く</button>
                <button class="delete-btn" on:click={(e) => handleDelete(session.id, e)}>削除</button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
  
  <div class="back-link">
    <a href="/">← チャットに戻る</a>
  </div>
</div>

<style>
  .history-container {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  h1 {
    margin-bottom: 2rem;
  }
  
  .loading, .empty {
    text-align: center;
    padding: 2rem;
    color: #757575;
  }
  
  .sessions {
    margin-bottom: 2rem;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
  }
  
  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }
  
  th {
    background-color: #f5f5f5;
    font-weight: bold;
  }
  
  tr:hover {
    background-color: #f9f9f9;
  }
  
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  
  button {
    padding: 0.25rem 0.5rem;
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
  }
  
  .open-btn {
    background-color: #2196f3;
  }
  
  .delete-btn {
    background-color: #f44336;
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
  
  .actions-bar {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  
  .export-btn, .import-btn {
    padding: 0.5rem 1rem;
    font-size: 1rem;
  }
  
  .export-btn {
    background-color: #4caf50;
  }
  
  .import-btn {
    background-color: #ff9800;
  }
  
  .status-message {
    margin-bottom: 1.5rem;
    padding: 0.75rem;
    background-color: #e3f2fd;
    border-radius: 0.25rem;
    border-left: 4px solid #2196f3;
  }
</style>