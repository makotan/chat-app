# デスクトップチャットアプリ

**注意: このツールはメンテナンスされません。**  
**Note: This tool is not maintained.**

Claude AIモデルとMCP（Model Context Protocol）を使用して対話し、チャット履歴と環境設定をローカルに保存するデスクトップチャットアプリケーションです。

## 機能

- クロスプラットフォーム対応（Mac/Windows、ARM/Intel）
- Claude AIモデルとのMCP連携
- チャット履歴と環境設定のローカルファイル保存
- シンプルなユーザーインターフェース
- ショートカットキー対応

## ビルド方法

### 必要なツール

- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/)
- [VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 開発環境のセットアップ

1. リポジトリをクローンします
2. 依存関係をインストールします
   ```bash
   npm install
   ```

### 開発サーバーの起動

```bash
npm run tauri dev
```

### ビルド

```bash
npm run tauri build
```

ビルドが完了すると、`src-tauri/target/release`ディレクトリにアプリケーションが生成されます。

## 使い方

### 初期設定

1. アプリケーションを起動します
2. 設定画面（Ctrl/Cmd + S）で以下の項目を設定します：
   - Claude API Key: Claude APIを使用するためのキー（必須）
   - モデル: 使用するClaudeのモデル（Claude 3 Opus、Claude 3 Sonnet、Claude 3 Haiku）
   - テーマ: ライトモードまたはダークモード
   - 履歴の最大保存数: チャット履歴の保存数（10〜1000）
   - 新規チャットの自動作成: アプリ起動時に新規チャットを自動作成するかどうか
3. 「保存」ボタンをクリックして設定を保存します

### チャット

1. メイン画面でメッセージを入力し、Enterキーを押すか送信ボタンをクリックしてメッセージを送信します
2. Shift + Enterで改行できます
3. 左側のサイドバーから新規チャットの作成やチャット履歴の表示ができます

### チャット履歴

1. チャット履歴画面（Ctrl/Cmd + H）でこれまでのチャットセッションを確認できます
2. 各セッションに対して以下の操作が可能です：
   - 開く: セッションを開いてチャットを続行
   - 削除: セッションを削除
3. 「エクスポート」ボタンでチャット履歴をJSONファイルとしてエクスポートできます
4. 「インポート」ボタンで以前エクスポートしたチャット履歴をインポートできます

## ショートカットキー

アプリケーションでは以下のショートカットキーが利用可能です：

| キー | 機能 |
|------|------|
| Ctrl/Cmd + N | 新規チャット |
| Ctrl/Cmd + H | チャット履歴 |
| Ctrl/Cmd + S | 設定 |
| Ctrl/Cmd + E | チャット履歴のエクスポート |
| Ctrl/Cmd + I | チャット履歴のインポート |
| Ctrl/Cmd + ? | ショートカットキーヘルプ |
| Enter | メッセージ送信 |
| Shift + Enter | 改行 |

ショートカットキーヘルプは、アプリケーション内で Ctrl/Cmd + ? を押すことでいつでも表示できます。
