<script lang="ts">
  import { onMount } from 'svelte';
  import { chatStore } from '$lib/stores/chat';
  import { getChatSessions, createChatSession } from '$lib/api';
  import type { ChatSession } from '$lib/types';
  
  let sessions: ChatSession[] = [];
  
  onMount(async () => {
    try {
      sessions = await getChatSessions();
      chatStore.update(state => ({
        ...state,
        sessions
      }));
    } catch (error) {
      console.error('Failed to load chat sessions:', error);
    }
  });
  
  function selectSession(sessionId: string) {
    chatStore.update(state => ({
      ...state,
      currentSessionId: sessionId
    }));
  }
  
  async function createNewChat() {
    try {
      // 現在の日時を含むタイトルを作成
      const now = new Date();
      const title = `新しいチャット (${now.toLocaleString()})`;
      
      // バックエンドでセッションを作成
      const sessionId = await createChatSession(title);
      
      // 新しいセッションオブジェクトを作成
      const newSession: ChatSession = {
        id: sessionId,
        title: title,
        createdAt: now.toISOString(),
        updatedAt: now.toISOString()
      };
      
      // セッションリストに追加
      sessions = [newSession, ...sessions];
      
      // chatStoreを更新
      chatStore.update(state => ({
        ...state,
        sessions,
        currentSessionId: sessionId
      }));
      
      // 新しいセッションを選択
      selectSession(sessionId);
    } catch (error) {
      console.error('Failed to create new chat session:', error);
    }
  }
</script>

<div class="sidebar">
  <div class="header">
    <h2>チャット履歴</h2>
    <button on:click={createNewChat}>新規チャット</button>
  </div>
  
  <div class="sessions">
    {#each sessions as session}
      <div 
        class="session" 
        class:active={$chatStore.currentSessionId === session.id}
        on:click={() => selectSession(session.id)}
      >
        <div class="title">{session.title}</div>
        <div class="date">{new Date(session.updatedAt).toLocaleDateString()}</div>
      </div>
    {/each}
    
    {#if sessions.length === 0}
      <div class="empty">チャット履歴がありません</div>
    {/if}
  </div>
  
  <div class="footer">
    <a href="/settings">設定</a>
  </div>
</div>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: 250px;
    height: 100%;
    background-color: var(--sidebar-bg, #f5f5f5);
    border-right: 1px solid var(--sidebar-border, #ddd);
  }
  
  .header {
    padding: 1rem;
    border-bottom: 1px solid var(--sidebar-border, #ddd);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .header h2 {
    margin: 0;
    font-size: 1.2rem;
  }
  
  .header button {
    padding: 0.25rem 0.5rem;
    background-color: var(--button-primary, #2196f3);
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    font-size: 0.8rem;
  }
  
  .sessions {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }
  
  .session {
    padding: 0.75rem;
    border-radius: 0.25rem;
    margin-bottom: 0.5rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .session:hover {
    background-color: var(--hover-bg, #e0e0e0);
  }
  
  .session.active {
    background-color: var(--active-bg, #e3f2fd);
  }
  
  .title {
    font-weight: 500;
    margin-bottom: 0.25rem;
  }
  
  .date {
    font-size: 0.8rem;
    color: var(--text-color, #757575);
    opacity: 0.7;
  }
  
  .empty {
    padding: 1rem;
    text-align: center;
    color: var(--text-color, #757575);
    opacity: 0.7;
    font-style: italic;
  }
  
  .footer {
    padding: 1rem;
    border-top: 1px solid var(--sidebar-border, #ddd);
    text-align: center;
  }
  
  .footer a {
    color: var(--button-primary, #2196f3);
    text-decoration: none;
  }
  
  .footer a:hover {
    text-decoration: underline;
  }
</style>