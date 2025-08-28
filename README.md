# claude-statusline

Claude Code IDE統合用のカスタムステータスライン生成Rustバイナリです。

## 概要

このツールは標準入力からJSONステータスイベントを処理し、開発環境で表示するためのフォーマット済みステータスラインを出力します。Claude Codeのステータスライン設定との連携を想定して設計されています。

## 機能

- Claude CodeステータスイベントをJSON入力から解析
- フォーマットされたステータスライン出力を生成
- 軽量でパフォーマンス最適化済み
- 簡単なインストールスクリプト付属

## インストール

付属のインストールスクリプトを実行してください：

```bash
./install.sh
```

このスクリプトは以下を実行します：
1. 最適化されたリリースバイナリをビルド
2. `~/.claude/bin/statusline`にインストール
3. 実行権限を付与

## 設定

`~/.claude/settings.json`を更新してステータスラインを使用するように設定：

```json
{
  "statusLine": {
    "type": "command",
    "command": "/Users/ユーザー名/.claude/bin/statusline"
  }
}
```
