## Tauri-ToDo 開発要件まとめ

### 開発環境

**必須ツール**
- Rust 1.70+ (rustup経由推奨)
- Node.js 18+ (フロントエンド用)
- pnpm/npm/yarn

**デスクトップ向け追加要件**
- **Linux**: webkit2gtk, libgtk-3-dev, libayatana-appindicator3-dev
- **Windows**: WebView2 (通常プリインストール済)
- **macOS**: Xcode Command Line Tools

**モバイル向け追加要件**
- **iOS**: macOS + Xcode + CocoaPods
- **Android**: Android Studio + SDK/NDK + Java 17+

---

### プロジェクト構成

```
project/
├── src-tauri/              # Rustバックエンド
│   ├── src/
│   │   ├── main.rs         # エントリーポイント
│   │   ├── commands.rs     # Tauriコマンド定義
│   │   ├── db.rs           # SQLite操作
│   │   └── scheduler.rs    # リマインダースケジューラー
│   ├── Cargo.toml          # Rust依存関係
│   └── tauri.conf.json     # Tauri設定
├── src/                    # フロントエンド
│   ├── contexts/
│   │   ├── TodoContext.tsx
│   │   ├── ReminderContext.tsx
│   │   └── SettingsContext.tsx
│   ├── components/
│   │   ├── TodoList.tsx
│   │   ├── ReminderSettings.tsx
│   │   └── StatsChart.tsx
│   ├── hooks/
│   │   └── useTauri.ts     # Tauri invoke wrapper
│   ├── types/
│   │   └── index.ts
│   ├── App.tsx
│   └── main.tsx
└── package.json
```

---

### 技術スタック

**フロントエンド**
- **React 19**
- TypeScript
- Tailwind CSS
- Recharts / Chart.js (グラフ)
- **React Context API + useReducer** (状態管理)

**バックエンド (Rust)**
- tauri 2.x
- rusqlite (SQLite)
- serde (JSON変換)
- tokio (非同期処理)
- chrono (日時処理)

---

### 状態管理設計

**Context構成**

**1. TodoContext**
```typescript
State: {
  todos: Todo[]
  loading: boolean
  error: string | null
}

Actions:
  - ADD_TODO
  - UPDATE_TODO
  - DELETE_TODO
  - TOGGLE_COMPLETE
  - SET_TODOS
```

**2. ReminderContext**
```typescript
State: {
  reminders: Reminder[]
  activeReminders: string[]
}

Actions:
  - ADD_REMINDER
  - UPDATE_REMINDER
  - DELETE_REMINDER
  - TOGGLE_REMINDER
```

**3. SettingsContext**
```typescript
State: {
  theme: 'light' | 'dark'
  notifications: boolean
  soundEnabled: boolean
}

Actions:
  - UPDATE_SETTINGS
```

---

### 実装する主要機能

**1. リマインダーシステム**
- Rustで定期タスクスケジューラー実装
- tauri-plugin-notification でシステム通知
- バックグラウンド実行対応
- Context経由で設定管理

**2. ToDoデータ管理**
- SQLiteでローカル保存
- Tauri CommandsでCRUD実装
- フロントエンドからinvoke()で呼び出し
- TodoContext/useReducerで状態管理

**3. UI実装**
- ToDo一覧・編集画面
- リマインダー設定画面
- 統計・グラフダッシュボード
- Context Provider構造で状態共有

**4. データ永続化**
- app_data_dir()でクロスプラットフォーム対応
- SQLiteスキーマ設計:
  - tasks (id, title, completed, created_at, updated_at)
  - reminders (id, task_id, time, frequency, enabled)
  - logs (id, task_id, action, timestamp)

---

### データフロー

```
UI Component
    ↓ dispatch(action)
useReducer in Context
    ↓ invoke Tauri command
Rust Backend
    ↓ SQLite operation
Database
    ↓ return data
Rust Backend
    ↓ serde serialize
Context state update
    ↓ re-render
UI Component
```

---

### ビルド・配布

**デスクトップ**
```bash
pnpm tauri build
```
- Windows: .exe + .msi
- macOS: .app + .dmg
- Linux: .AppImage / .deb

**モバイル**
```bash
pnpm tauri android init
pnpm tauri android build

pnpm tauri ios init
pnpm tauri ios build
```
- Android: .apk / .aab
- iOS: .app (要Xcode署名)

---

### 依存パッケージ

**package.json**
```json
{
  "dependencies": {
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-notification": "^2.0.0",
    "recharts": "^2.12.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "typescript": "^5.5.0",
    "vite": "^5.0.0",
    "tailwindcss": "^3.4.0"
  }
}
```

**Cargo.toml**
```toml
[dependencies]
tauri = "2.0"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled"] }
tokio = { version = "1", features = ["full"] }
chrono = "0.4"
tauri-plugin-notification = "2.0"
```

---

### 学習リソース

- Tauri公式: https://tauri.app/
- React 19ドキュメント: https://react.dev/
- Rust基礎: The Rust Book (日本語版)
- useReducerパターン: React公式ドキュメント

---

### 開発期間目安

- **環境構築**: 1-2日
- **Rust基礎学習**: 1週間
- **React 19 + Context設計**: 2-3日
- **デスクトップ版MVP**: 1-2週間
- **モバイル対応**: 追加1週間
- **UI/UX洗練**: 1-2週間

**合計: 4-6週間** (Rust未経験の場合)

---

### 注意点

- React 19は手動セットアップ必要（Tauriテンプレートは18ベース）
- Context構造は事前設計推奨（後から変更は手間）
- useReducerのaction型定義を厳密に
- モバイルビルドは各OS上でのみ可能
- SQLiteマイグレーション戦略を事前設計
