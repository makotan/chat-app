<script lang="ts">
  import { onMount } from 'svelte';
  import { chatStore } from '$lib/stores/chat';
  import { getChatSessions, deleteChatSession } from '$lib/api';
  import type { ChatSession } from '$lib/types';
  
  let sessions: ChatSession[] = [];
  let isLoading = true;
  let isDeleting = false;
  
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
</script>

<div class="history-container">
  <h1>チャット履歴</h1>
  
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
</style>