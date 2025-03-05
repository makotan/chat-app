<script lang="ts">
  export let isOpen = false;
  
  function handleClose() {
    isOpen = false;
  }
</script>

{#if isOpen}
  <div class="shortcut-help-overlay" on:click={handleClose}>
    <div class="shortcut-help-container" on:click|stopPropagation>
      <div class="shortcut-help-header">
        <h2>ショートカットキー</h2>
        <button class="close-btn" on:click={handleClose}>×</button>
      </div>
      
      <div class="shortcut-help-content">
        <table>
          <thead>
            <tr>
              <th>キー</th>
              <th>機能</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td><kbd>Ctrl/Cmd</kbd> + <kbd>N</kbd></td>
              <td>新規チャット</td>
            </tr>
            <tr>
              <td><kbd>Ctrl/Cmd</kbd> + <kbd>H</kbd></td>
              <td>チャット履歴</td>
            </tr>
            <tr>
              <td><kbd>Ctrl/Cmd</kbd> + <kbd>S</kbd></td>
              <td>設定</td>
            </tr>
            <tr>
              <td><kbd>Ctrl/Cmd</kbd> + <kbd>E</kbd></td>
              <td>チャット履歴のエクスポート</td>
            </tr>
            <tr>
              <td><kbd>Ctrl/Cmd</kbd> + <kbd>I</kbd></td>
              <td>チャット履歴のインポート</td>
            </tr>
            <tr>
              <td><kbd>Ctrl/Cmd</kbd> + <kbd>?</kbd></td>
              <td>ショートカットキーヘルプ</td>
            </tr>
            <tr>
              <td><kbd>Enter</kbd></td>
              <td>メッセージ送信</td>
            </tr>
            <tr>
              <td><kbd>Shift</kbd> + <kbd>Enter</kbd></td>
              <td>改行</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
{/if}

<style>
  .shortcut-help-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }
  
  .shortcut-help-container {
    background-color: var(--bg-color, #ffffff);
    border-radius: 0.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    width: 90%;
    max-width: 600px;
    max-height: 80vh;
    overflow-y: auto;
  }
  
  .shortcut-help-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    border-bottom: 1px solid var(--divider, #eee);
  }
  
  .shortcut-help-header h2 {
    margin: 0;
  }
  
  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: var(--text-color, #333);
  }
  
  .shortcut-help-content {
    padding: 1rem;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
  }
  
  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid var(--divider, #eee);
  }
  
  th {
    font-weight: bold;
  }
  
  kbd {
    display: inline-block;
    padding: 0.2rem 0.4rem;
    font-size: 0.85rem;
    font-family: monospace;
    line-height: 1;
    color: var(--text-color, #333);
    background-color: var(--hover-bg, #f7f7f7);
    border: 1px solid var(--divider, #ccc);
    border-radius: 0.25rem;
    box-shadow: 0 1px 0 rgba(0, 0, 0, 0.2);
  }
</style>