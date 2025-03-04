<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let isLoading = false;
  
  let message = '';
  const dispatch = createEventDispatcher<{ send: string }>();
  
  function handleSubmit() {
    if (!message.trim() || isLoading) return;
    
    dispatch('send', message);
    message = '';
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="chat-input">
  <textarea
    bind:value={message}
    placeholder="メッセージを入力..."
    disabled={isLoading}
    on:keydown={(e) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        handleSubmit();
      }
    }}
  ></textarea>
  <button type="submit" disabled={isLoading || !message.trim()}>
    送信
  </button>
</form>

<style>
  .chat-input {
    display: flex;
    padding: 1rem;
    border-top: 1px solid var(--divider, #eee);
  }
  
  textarea {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid var(--input-border, #ddd);
    border-radius: 0.25rem;
    margin-right: 0.5rem;
    resize: none;
    height: 60px;
    background-color: var(--bg-color, #ffffff);
    color: var(--text-color, #333333);
  }
  
  button {
    padding: 0.5rem 1rem;
    background-color: var(--button-primary, #2196f3);
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    align-self: flex-end;
  }
  
  button:disabled {
    background-color: var(--hover-bg, #bdbdbd);
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>