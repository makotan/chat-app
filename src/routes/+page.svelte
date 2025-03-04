<script lang="ts">
  import { onMount } from 'svelte';
  import { chatStore } from '$lib/stores/chat';
  import { settingsStore } from '$lib/stores/settings';
  import { sendMessage, getChatMessages, addChatMessage, createChatSession, getConfig, initializeMcp } from '$lib/api';
  import ChatMessage from '$lib/components/ChatMessage.svelte';
  import ChatInput from '$lib/components/ChatInput.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import type { Message } from '$lib/types';
  
  let messages: Message[] = [];
  let isLoading = false;
  let currentSessionId = '';
  
  onMount(async () => {
    try {
      // 設定を読み込む
      const config = await getConfig();
      settingsStore.set(config);
      
      // API Keyが設定されている場合、MCPクライアントを初期化
      if (config.apiKey) {
        await initializeMcp(config.apiKey);
      }
      
      // 最新のセッションを取得するか、新しいセッションを作成
      currentSessionId = $chatStore.currentSessionId;
      if (currentSessionId) {
        messages = await getChatMessages(currentSessionId);
      } else {
        // 新しいセッションを作成
        currentSessionId = await createChatSession('新しいチャット');
        chatStore.update(state => ({
          ...state,
          currentSessionId
        }));
      }
    } catch (error) {
      console.error('Error initializing chat:', error);
    }
  });
  
  // chatStoreのcurrentSessionIdが変更されたら、メッセージを読み込む
  $: if ($chatStore.currentSessionId && $chatStore.currentSessionId !== currentSessionId) {
    currentSessionId = $chatStore.currentSessionId;
    loadMessages(currentSessionId);
  }
  
  async function loadMessages(sessionId: string) {
    try {
      messages = await getChatMessages(sessionId);
    } catch (error) {
      console.error('Error loading messages:', error);
    }
  }
  
  async function handleSendMessage(event: CustomEvent<string>) {
    const content = event.detail;
    if (!content.trim() || isLoading) return;
    
    isLoading = true;
    
    try {
      // ユーザーメッセージをデータベースに保存
      await addChatMessage(currentSessionId, 'user', content);
      
      // UIを更新
      messages = [...messages, {
        id: 'temp-user',
        sessionId: currentSessionId,
        role: 'user',
        content,
        timestamp: new Date().toISOString()
      }];
      
      // Claudeに送信
      const response = await sendMessage(content, currentSessionId);
      
      // アシスタントの応答をデータベースに保存
      await addChatMessage(currentSessionId, 'assistant', response);
      
      // UIを更新
      messages = [...messages, {
        id: 'temp-assistant',
        sessionId: currentSessionId,
        role: 'assistant',
        content: response,
        timestamp: new Date().toISOString()
      }];
    } catch (error) {
      console.error('Error sending message:', error);
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="app-container">
  <Sidebar />
  
  <div class="chat-container">
    <div class="messages">
      {#each messages as message}
        <ChatMessage {message} />
      {/each}
      
      {#if isLoading}
        <div class="loading">応答を生成中...</div>
      {/if}
    </div>
    
    <ChatInput on:send={handleSendMessage} {isLoading} />
  </div>
</div>

<style>
  /* グローバルスタイルは+layout.svelteに移動 */
  
  .app-container {
    display: flex;
    height: 100vh;
  }
  
  .chat-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  
  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
  }
  
  .loading {
    text-align: center;
    color: var(--text-color, #757575);
    opacity: 0.7;
    font-style: italic;
    margin: 1rem 0;
  }
</style>
