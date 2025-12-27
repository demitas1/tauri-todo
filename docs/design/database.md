# データベース設計

## 概要

SQLiteを使用したローカルデータベース設計。
Tauri APIの`app_data_dir()`を使用してプラットフォーム固有のデータディレクトリに配置。

**Linux保存先:** `~/.local/share/com.example.tauri-todo-app/app.db`

## 実装状況

| テーブル | 状態 | 説明 |
|---------|------|------|
| counter | ✅ 実装済み | カウンター値の永続化 |
| categories | 📋 計画中 | カテゴリ管理 |
| tasks | 📋 計画中 | タスク管理 |
| reminders | 📋 計画中 | リマインダー設定 |
| logs | 📋 計画中 | 操作履歴 |

## 実装済みテーブル

### counter テーブル

汎用的なkey-value形式のカウンター管理テーブル

```sql
CREATE TABLE IF NOT EXISTS counter (
    key TEXT PRIMARY KEY,
    value INTEGER NOT NULL DEFAULT 0
);

-- 初期データ
INSERT OR IGNORE INTO counter (key, value) VALUES ('main', 0);
```

**カラム説明:**
- `key`: カウンターの識別子（主キー）
- `value`: カウンター値

**Rust実装 (`src-tauri/src/db.rs`):**
```rust
pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_data_dir: PathBuf) -> Result<Self>;
    pub fn get_count(&self) -> Result<i32>;
    pub fn set_count(&self, value: i32) -> Result<i32>;
    pub fn increment(&self) -> Result<i32>;
    pub fn decrement(&self) -> Result<i32>;
    pub fn reset(&self) -> Result<i32>;
}
```

## 計画中テーブル

## ER図

```
┌──────────────┐         ┌──────────────┐
│  categories  │         │    tasks     │
├──────────────┤         ├──────────────┤
│ id (PK)      │◄───┐    │ id (PK)      │
│ name         │    │    │ title        │
│ color        │    └────│ category_id  │
│ created_at   │         │ completed    │
└──────────────┘         │ created_at   │
                         │ updated_at   │
                         └──────┬───────┘
                                │
                                │ 1:N
                                │
                         ┌──────▼───────┐
                         │  reminders   │
                         ├──────────────┤
                         │ id (PK)      │
                         │ task_id (FK) │
                         │ time         │
                         │ frequency    │
                         │ enabled      │
                         │ created_at   │
                         └──────┬───────┘
                                │
                                │ 1:N
                                │
                         ┌──────▼───────┐
                         │  logs        │
                         ├──────────────┤
                         │ id (PK)      │
                         │ task_id (FK) │
                         │ action       │
                         │ timestamp    │
                         └──────────────┘
```

## テーブル定義

### 1. categories テーブル

カテゴリ管理テーブル

```sql
CREATE TABLE categories (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    color       TEXT NOT NULL,           -- 16進数カラーコード (#FF5733)
    created_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);
```

**カラム説明:**
- `id`: カテゴリID（主キー）
- `name`: カテゴリ名（ユニーク制約）例: "仕事", "プライベート", "買い物"
- `color`: UIで使用する色（TailwindのカラーパレットまたはカスタムHEX）
- `created_at`: 作成日時（ISO8601形式）

**初期データ:**
```sql
INSERT INTO categories (name, color) VALUES
    ('仕事', '#3B82F6'),      -- blue-500
    ('プライベート', '#10B981'), -- green-500
    ('買い物', '#F59E0B');      -- amber-500
```

### 2. tasks テーブル

タスク管理テーブル

```sql
CREATE TABLE tasks (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL,
    category_id INTEGER NOT NULL,
    completed   INTEGER NOT NULL DEFAULT 0,  -- 0: 未完了, 1: 完了
    created_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- インデックス作成（検索パフォーマンス向上）
CREATE INDEX idx_tasks_category ON tasks(category_id);
CREATE INDEX idx_tasks_completed ON tasks(completed);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
```

**カラム説明:**
- `id`: タスクID（主キー）
- `title`: タスクのタイトル
- `category_id`: 所属カテゴリ（外部キー）
- `completed`: 完了状態（0: false, 1: true）
- `created_at`: 作成日時
- `updated_at`: 最終更新日時

**制約:**
- カテゴリ削除時は関連タスクも削除（CASCADE）

### 3. reminders テーブル

リマインダー設定テーブル

```sql
CREATE TABLE reminders (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id     INTEGER NOT NULL,
    time        TEXT NOT NULL,           -- HH:MM形式 (例: "14:30")
    frequency   TEXT NOT NULL,           -- "daily", "weekly", "monthly", "custom"
    custom_days TEXT,                    -- カスタム頻度用（JSON配列: ["月","水","金"]）
    enabled     INTEGER NOT NULL DEFAULT 1,  -- 0: 無効, 1: 有効
    created_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

-- インデックス作成
CREATE INDEX idx_reminders_task ON reminders(task_id);
CREATE INDEX idx_reminders_enabled ON reminders(enabled);
```

**カラム説明:**
- `id`: リマインダーID（主キー）
- `task_id`: 対象タスク（外部キー）
- `time`: 通知時刻（24時間形式）
- `frequency`: 頻度タイプ
  - `"daily"`: 毎日
  - `"weekly"`: 毎週
  - `"monthly"`: 毎月
  - `"custom"`: カスタム（`custom_days`を使用）
- `custom_days`: カスタム頻度の詳細（JSON形式）
  - 例: `["月", "水", "金"]` または `[1, 3, 5]` (曜日番号)
- `enabled`: 有効/無効フラグ
- `created_at`: 作成日時

**制約:**
- タスク削除時はリマインダーも削除（CASCADE）

**frequency別の動作例:**
```
daily:    毎日14:30に通知
weekly:   毎週同じ曜日の14:30に通知（リマインダー作成時の曜日）
monthly:  毎月同じ日の14:30に通知（リマインダー作成時の日付）
custom:   指定された曜日の14:30に通知
```

### 4. logs テーブル

タスク操作履歴テーブル（統計用）

```sql
CREATE TABLE logs (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id     INTEGER NOT NULL,
    action      TEXT NOT NULL,           -- "created", "completed", "uncompleted", "deleted"
    timestamp   TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL
);

-- インデックス作成
CREATE INDEX idx_logs_task ON logs(task_id);
CREATE INDEX idx_logs_timestamp ON logs(timestamp);
CREATE INDEX idx_logs_action ON logs(action);
```

**カラム説明:**
- `id`: ログID（主キー）
- `task_id`: 対象タスク（タスク削除後もログは残す）
- `action`: 実行されたアクション
  - `"created"`: タスク作成
  - `"completed"`: タスク完了
  - `"uncompleted"`: 完了解除
  - `"deleted"`: タスク削除
- `timestamp`: アクション実行日時

**用途:**
- 日別/週別/月別の完了率計算
- カテゴリ別の統計
- タスクごとの操作履歴

## マイグレーション戦略

### 初回起動時

```rust
// db.rs内で実装
pub fn initialize_database() -> Result<(), rusqlite::Error> {
    let db_path = get_db_path(); // app_data_dir() + "data.db"
    let conn = Connection::open(db_path)?;

    // テーブル作成
    conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))?;

    // 初期データ投入
    conn.execute_batch(include_str!("../migrations/002_seed_categories.sql"))?;

    Ok(())
}
```

### バージョン管理

```sql
-- schema_versionテーブルでマイグレーション管理
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);
```

**マイグレーションファイル構成:**
```
src-tauri/
└── migrations/
    ├── 001_initial_schema.sql
    ├── 002_seed_categories.sql
    └── (将来的に追加)
```

## 統計クエリ例

### 1. 日別完了率

```sql
SELECT
    DATE(timestamp) as date,
    COUNT(*) as completed_count
FROM logs
WHERE action = 'completed'
  AND timestamp >= datetime('now', '-30 days')
GROUP BY DATE(timestamp)
ORDER BY date;
```

### 2. カテゴリ別完了数

```sql
SELECT
    c.name,
    c.color,
    COUNT(l.id) as completed_count
FROM categories c
LEFT JOIN tasks t ON c.id = t.category_id
LEFT JOIN logs l ON t.id = l.task_id AND l.action = 'completed'
WHERE l.timestamp >= datetime('now', '-7 days')
GROUP BY c.id
ORDER BY completed_count DESC;
```

### 3. タスク別の完了回数

```sql
SELECT
    t.title,
    c.name as category,
    COUNT(l.id) as completion_count
FROM tasks t
LEFT JOIN categories c ON t.category_id = c.id
LEFT JOIN logs l ON t.id = l.task_id AND l.action = 'completed'
GROUP BY t.id
ORDER BY completion_count DESC
LIMIT 10;
```

## パフォーマンス最適化

### インデックス戦略
- 外部キー列にインデックス作成
- WHERE句で頻繁に使用される列にインデックス作成
- 統計クエリでよく使うtimestampにインデックス作成

### データ保守
- logsテーブルは定期的にアーカイブ（例: 1年以上前のデータを削除）
- VACUUM実行でディスク容量最適化

```sql
-- 古いログの削除（1年以上前）
DELETE FROM logs WHERE timestamp < datetime('now', '-365 days');

-- データベース最適化
VACUUM;
```

## バックアップ戦略

Rustコードでのバックアップ実装例:

```rust
pub fn backup_database() -> Result<(), std::io::Error> {
    let source = get_db_path();
    let backup_path = format!("{}.backup.{}",
        source.display(),
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    std::fs::copy(source, backup_path)?;
    Ok(())
}
```

## トランザクション管理

### 複数操作の一貫性保証

```rust
// タスク作成 + ログ記録をトランザクションで実行
pub fn create_task_with_log(conn: &Connection, task: &Task) -> Result<i64> {
    let tx = conn.transaction()?;

    // タスク挿入
    tx.execute(
        "INSERT INTO tasks (title, category_id, completed) VALUES (?1, ?2, ?3)",
        params![task.title, task.category_id, 0],
    )?;

    let task_id = tx.last_insert_rowid();

    // ログ記録
    tx.execute(
        "INSERT INTO logs (task_id, action) VALUES (?1, 'created')",
        params![task_id],
    )?;

    tx.commit()?;
    Ok(task_id)
}
```
