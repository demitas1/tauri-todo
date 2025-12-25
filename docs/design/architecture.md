# システムアーキテクチャ設計

## 概要

Tauri 2.x + React 19 + TypeScriptを使用したクロスプラットフォームToDoアプリケーション（Linuxデスクトップ版）

## アーキテクチャ全体図

```
┌─────────────────────────────────────────────────────────┐
│                     UI Layer (React)                     │
├─────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ TodoContext  │  │ReminderContext│  │SettingsContext│ │
│  │              │  │               │  │               │  │
│  │ - useReducer │  │ - useReducer  │  │ - useReducer  │  │
│  └──────┬───────┘  └──────┬────────┘  └──────┬────────┘  │
│         │                 │                   │           │
│         └─────────────────┴───────────────────┘           │
│                           │                               │
│                    invoke() from                          │
│                   @tauri-apps/api                         │
└───────────────────────────┬───────────────────────────────┘
                            │
                    ┌───────▼────────┐
                    │  Tauri Bridge  │
                    └───────┬────────┘
                            │
┌───────────────────────────▼───────────────────────────────┐
│                  Backend Layer (Rust)                     │
├─────────────────────────────────────────────────────────  ┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  commands.rs │  │  scheduler.rs │  │    db.rs     │    │
│  │              │  │               │  │              │    │
│  │ - Tauri      │  │ - Tokio       │  │ - rusqlite   │    │
│  │   Commands   │  │ - Reminder    │  │ - CRUD       │    │
│  │              │  │   Scheduler   │  │   Operations │    │
│  └──────┬───────┘  └──────┬────────┘  └──────┬────────┘   │
│         │                 │                   │            │
│         └─────────────────┴───────────────────┘            │
│                           │                                │
└───────────────────────────┼────────────────────────────────┘
                            │
                    ┌───────▼────────┐
                    │  SQLite DB     │
                    │  (app_data_dir)│
                    └────────────────┘
```

## レイヤー構成

### 1. UIレイヤー (React + TypeScript)

**責務:**
- ユーザーインタラクションの処理
- 状態管理（Context API + useReducer）
- UIレンダリング
- 多言語対応（i18n）
- テーマ切り替え（ダーク/ライト）

**主要技術:**
- React 19
- TypeScript 5.5+
- Tailwind CSS
- Recharts（統計グラフ）
- i18next（多言語対応）

### 2. Tauriブリッジレイヤー

**責務:**
- フロントエンドとバックエンド間の通信
- 型安全なAPI呼び出し
- エラーハンドリング

**使用API:**
- `@tauri-apps/api` の `invoke()`
- `@tauri-apps/plugin-notification`

### 3. バックエンドレイヤー (Rust)

**責務:**
- ビジネスロジックの実装
- データベース操作
- リマインダースケジューリング
- システム通知の発行

**モジュール構成:**

#### commands.rs
- Tauri Commandsの定義
- フロントエンドから呼び出される関数群
- リクエスト/レスポンスの型定義

#### db.rs
- SQLite接続管理
- CRUD操作の実装
- トランザクション管理
- マイグレーション処理

#### scheduler.rs
- Tokioベースのバックグラウンドタスク
- リマインダーの定期チェック
- 通知トリガーの実行
- バックグラウンド動作ON/OFF制御

### 4. データレイヤー (SQLite)

**責務:**
- タスクデータの永続化
- リマインダー設定の保存
- 統計データの集計
- ログ記録

**保存場所:**
- Tauri API `app_data_dir()` を使用
- Linux: `~/.local/share/tauri-todo/`

## データフロー

### タスク追加の例

```
1. [UI] TodoForm
   ↓ dispatch({ type: 'ADD_TODO', payload: {...} })

2. [Context] TodoContext.reducer
   ↓ invoke('create_task', { ... })

3. [Tauri Bridge] IPC通信
   ↓

4. [Rust] commands::create_task()
   ↓ db::insert_task()

5. [SQLite] INSERT INTO tasks ...
   ↓ 成功/失敗を返す

6. [Rust] Result<Task, String>をシリアライズ
   ↓

7. [Tauri Bridge] JSON形式で返却
   ↓

8. [Context] stateを更新
   ↓

9. [UI] 再レンダリング
```

### リマインダー通知の例

```
1. [Rust] scheduler.rs - バックグラウンドループ
   ↓ 毎分チェック

2. [SQLite] SELECT * FROM reminders WHERE enabled = 1
   ↓

3. [Rust] 現在時刻と比較
   ↓ 通知タイミングに達した場合

4. [Rust] tauri-plugin-notification::send()
   ↓

5. [Linux] デスクトップ通知表示
```

## モジュール間の依存関係

```
main.rs
  ├── commands.rs
  │     └── db.rs
  ├── scheduler.rs
  │     ├── db.rs
  │     └── tauri-plugin-notification
  └── db.rs
        └── rusqlite
```

## 非機能要件

### パフォーマンス
- アプリ起動時間: 2秒以内
- タスク追加/削除: 100ms以内
- 統計グラフ描画: 500ms以内

### セキュリティ
- SQLインジェクション対策: rusqliteのパラメータバインディング使用
- XSS対策: Reactの自動エスケープ機能
- ファイルアクセス: Tauri APIの制限範囲内のみ

### 拡張性
- 新規カテゴリの追加が容易
- リマインダー頻度タイプの追加が容易
- 新言語の追加が容易（i18n対応）
- モバイル版への拡張を考慮

### 保守性
- SOLID原則に従った設計
- TypeScriptによる型安全性
- Rustの所有権システムによるメモリ安全性
- 各モジュールの責務を明確に分離

## 開発フェーズ

### Phase 1: 基盤構築
1. Tauriプロジェクト初期化
2. React + Vite + TypeScript セットアップ
3. Tailwind CSS導入
4. SQLiteスキーマ作成

### Phase 2: コア機能実装
1. タスクCRUD操作
2. カテゴリ管理
3. 基本UI実装

### Phase 3: リマインダー機能
1. リマインダー設定UI
2. スケジューラー実装
3. 通知機能実装

### Phase 4: 統計・設定
1. 統計グラフ実装
2. テーマ切り替え
3. 多言語対応

### Phase 5: 最適化・テスト
1. パフォーマンスチューニング
2. エラーハンドリング強化
3. ユーザビリティ向上

## 技術的制約・前提条件

### 開発環境
- Rust 1.70+
- Node.js 18+
- pnpm（推奨）

### Linux固有の依存関係
- webkit2gtk
- libgtk-3-dev
- libayatana-appindicator3-dev

### ビルド成果物
- AppImage（単一実行ファイル、依存関係同梱）
- .deb パッケージ（Debian/Ubuntu向け）
