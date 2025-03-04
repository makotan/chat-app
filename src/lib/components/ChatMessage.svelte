<script lang="ts">
  import type { Message } from '$lib/types';
  
  export let message: Message;
</script>

<div class="message {message.role}">
  <div class="content">{message.content}</div>
</div>

<style>
  .message {
    margin-bottom: 1rem;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    max-width: 80%;
  }
  
  .user {
    align-self: flex-end;
    background-color: #e3f2fd;
    margin-left: auto;
  }
  
  .assistant {
    align-self: flex-start;
    background-color: #f5f5f5;
  }
  
  .content {
    white-space: pre-wrap;
  }
</style>