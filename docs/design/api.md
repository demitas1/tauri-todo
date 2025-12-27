# Tauri Commands API設計

## 概要

フロントエンド（React）とバックエンド（Rust）間のインターフェース定義。
コマンドは`src-tauri/src/lib.rs`に定義され、`@tauri-apps/api`の`invoke()`で呼び出される。

## 実装状況

| コマンド | 状態 | 説明 |
|---------|------|------|
| get_count | ✅ 実装済み | カウンター値取得 |
| increment | ✅ 実装済み | カウンター増加 |
| decrement | ✅ 実装済み | カウンター減少 |
| reset | ✅ 実装済み | カウンターリセット |
| get_categories | 📋 計画中 | カテゴリ一覧取得 |
| create_category | 📋 計画中 | カテゴリ作成 |
| ... | 📋 計画中 | その他 |

## 実装済みコマンド

### カウンター関連コマンド

#### get_count

カウンター値を取得

**Rust (`src-tauri/src/lib.rs`):**
```rust
#[tauri::command]
fn get_count(db: State<Database>) -> Result<i32, String>
```

**TypeScript:**
```typescript
const count = await invoke<number>('get_count');
```

#### increment

カウンターを1増加

**Rust:**
```rust
#[tauri::command]
fn increment(db: State<Database>) -> Result<i32, String>
```

**TypeScript:**
```typescript
const newCount = await invoke<number>('increment');
```

#### decrement

カウンターを1減少

**Rust:**
```rust
#[tauri::command]
fn decrement(db: State<Database>) -> Result<i32, String>
```

**TypeScript:**
```typescript
const newCount = await invoke<number>('decrement');
```

#### reset

カウンターを0にリセット

**Rust:**
```rust
#[tauri::command]
fn reset(db: State<Database>) -> Result<i32, String>
```

**TypeScript:**
```typescript
const newCount = await invoke<number>('reset');
```

## 計画中コマンド

## 型定義

### Rust側の共通型

```rust
use serde::{Deserialize, Serialize};

// カテゴリ
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

// タスク
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub category_id: i64,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

// リマインダー
#[derive(Debug, Serialize, Deserialize)]
pub struct Reminder {
    pub id: i64,
    pub task_id: i64,
    pub time: String,              // "HH:MM"
    pub frequency: FrequencyType,
    pub custom_days: Option<Vec<String>>,
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FrequencyType {
    Daily,
    Weekly,
    Monthly,
    Custom,
}

// 統計データ
#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub daily: Vec<DailyStat>,
    pub weekly: Vec<WeeklyStat>,
    pub monthly: Vec<MonthlyStat>,
    pub by_category: Vec<CategoryStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyStat {
    pub date: String,              // "2025-11-13"
    pub completed_count: i32,
    pub total_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyStat {
    pub week_start: String,        // "2025-11-10"
    pub completed_count: i32,
    pub total_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyStat {
    pub month: String,             // "2025-11"
    pub completed_count: i32,
    pub total_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryStat {
    pub category_id: i64,
    pub category_name: String,
    pub color: String,
    pub completed_count: i32,
    pub total_count: i32,
}
```

### TypeScript側の型定義

```typescript
// src/types/index.ts

export interface Category {
  id: number;
  name: string;
  color: string;
  created_at: string;
}

export interface Task {
  id: number;
  title: string;
  category_id: number;
  completed: boolean;
  created_at: string;
  updated_at: string;
}

export interface Reminder {
  id: number;
  task_id: number;
  time: string;
  frequency: 'daily' | 'weekly' | 'monthly' | 'custom';
  custom_days?: string[];
  enabled: boolean;
  created_at: string;
}

export interface Statistics {
  daily: DailyStat[];
  weekly: WeeklyStat[];
  monthly: MonthlyStat[];
  by_category: CategoryStat[];
}

export interface DailyStat {
  date: string;
  completed_count: number;
  total_count: number;
}

export interface WeeklyStat {
  week_start: string;
  completed_count: number;
  total_count: number;
}

export interface MonthlyStat {
  month: string;
  completed_count: number;
  total_count: number;
}

export interface CategoryStat {
  category_id: number;
  category_name: string;
  color: string;
  completed_count: number;
  total_count: number;
}
```

## カテゴリ関連コマンド

### get_categories

すべてのカテゴリを取得

**Rust:**
```rust
#[tauri::command]
pub fn get_categories() -> Result<Vec<Category>, String> {
    // 実装
}
```

**TypeScript:**
```typescript
import { invoke } from '@tauri-apps/api/core';

const categories = await invoke<Category[]>('get_categories');
```

**レスポンス例:**
```json
[
  {
    "id": 1,
    "name": "仕事",
    "color": "#3B82F6",
    "created_at": "2025-11-13 10:00:00"
  },
  {
    "id": 2,
    "name": "プライベート",
    "color": "#10B981",
    "created_at": "2025-11-13 10:00:00"
  }
]
```

### create_category

新規カテゴリを作成

**Rust:**
```rust
#[tauri::command]
pub fn create_category(name: String, color: String) -> Result<Category, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const category = await invoke<Category>('create_category', {
  name: '趣味',
  color: '#F59E0B'
});
```

### update_category

カテゴリを更新

**Rust:**
```rust
#[tauri::command]
pub fn update_category(id: i64, name: String, color: String) -> Result<Category, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const updated = await invoke<Category>('update_category', {
  id: 1,
  name: '仕事（重要）',
  color: '#EF4444'
});
```

### delete_category

カテゴリを削除（関連タスクも削除）

**Rust:**
```rust
#[tauri::command]
pub fn delete_category(id: i64) -> Result<(), String> {
    // 実装
}
```

**TypeScript:**
```typescript
await invoke('delete_category', { id: 1 });
```

## タスク関連コマンド

### get_tasks

すべてのタスクを取得（オプションでフィルタリング）

**Rust:**
```rust
#[tauri::command]
pub fn get_tasks(
    category_id: Option<i64>,
    completed: Option<bool>
) -> Result<Vec<Task>, String> {
    // 実装
}
```

**TypeScript:**
```typescript
// すべて取得
const allTasks = await invoke<Task[]>('get_tasks', {
  category_id: null,
  completed: null
});

// カテゴリでフィルタ
const workTasks = await invoke<Task[]>('get_tasks', {
  category_id: 1,
  completed: null
});

// 未完了のみ
const incompleteTasks = await invoke<Task[]>('get_tasks', {
  category_id: null,
  completed: false
});
```

### get_task

特定のタスクを取得

**Rust:**
```rust
#[tauri::command]
pub fn get_task(id: i64) -> Result<Task, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const task = await invoke<Task>('get_task', { id: 1 });
```

### create_task

新規タスクを作成

**Rust:**
```rust
#[tauri::command]
pub fn create_task(title: String, category_id: i64) -> Result<Task, String> {
    // 実装（logsテーブルにも記録）
}
```

**TypeScript:**
```typescript
const newTask = await invoke<Task>('create_task', {
  title: 'プロジェクト資料作成',
  category_id: 1
});
```

### update_task

タスクを更新

**Rust:**
```rust
#[tauri::command]
pub fn update_task(
    id: i64,
    title: String,
    category_id: i64
) -> Result<Task, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const updated = await invoke<Task>('update_task', {
  id: 1,
  title: '修正後のタイトル',
  category_id: 2
});
```

### toggle_task

タスクの完了状態を切り替え

**Rust:**
```rust
#[tauri::command]
pub fn toggle_task(id: i64) -> Result<Task, String> {
    // 実装（logsテーブルに'completed'または'uncompleted'を記録）
}
```

**TypeScript:**
```typescript
const toggled = await invoke<Task>('toggle_task', { id: 1 });
```

### delete_task

タスクを削除

**Rust:**
```rust
#[tauri::command]
pub fn delete_task(id: i64) -> Result<(), String> {
    // 実装（logsテーブルに'deleted'を記録）
}
```

**TypeScript:**
```typescript
await invoke('delete_task', { id: 1 });
```

## リマインダー関連コマンド

### get_reminders

タスクのリマインダー一覧を取得

**Rust:**
```rust
#[tauri::command]
pub fn get_reminders(task_id: i64) -> Result<Vec<Reminder>, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const reminders = await invoke<Reminder[]>('get_reminders', { task_id: 1 });
```

### create_reminder

リマインダーを作成

**Rust:**
```rust
#[tauri::command]
pub fn create_reminder(
    task_id: i64,
    time: String,
    frequency: FrequencyType,
    custom_days: Option<Vec<String>>
) -> Result<Reminder, String> {
    // 実装
}
```

**TypeScript:**
```typescript
// 毎日リマインダー
const daily = await invoke<Reminder>('create_reminder', {
  task_id: 1,
  time: '09:00',
  frequency: 'daily',
  custom_days: null
});

// カスタムリマインダー（月・水・金）
const custom = await invoke<Reminder>('create_reminder', {
  task_id: 1,
  time: '14:30',
  frequency: 'custom',
  custom_days: ['月', '水', '金']
});
```

### update_reminder

リマインダーを更新

**Rust:**
```rust
#[tauri::command]
pub fn update_reminder(
    id: i64,
    time: String,
    frequency: FrequencyType,
    custom_days: Option<Vec<String>>
) -> Result<Reminder, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const updated = await invoke<Reminder>('update_reminder', {
  id: 1,
  time: '10:00',
  frequency: 'weekly',
  custom_days: null
});
```

### toggle_reminder

リマインダーの有効/無効を切り替え

**Rust:**
```rust
#[tauri::command]
pub fn toggle_reminder(id: i64) -> Result<Reminder, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const toggled = await invoke<Reminder>('toggle_reminder', { id: 1 });
```

### delete_reminder

リマインダーを削除

**Rust:**
```rust
#[tauri::command]
pub fn delete_reminder(id: i64) -> Result<(), String> {
    // 実装
}
```

**TypeScript:**
```typescript
await invoke('delete_reminder', { id: 1 });
```

## 統計関連コマンド

### get_statistics

統計データを取得

**Rust:**
```rust
#[tauri::command]
pub fn get_statistics(
    start_date: String,  // "2025-11-01"
    end_date: String     // "2025-11-30"
) -> Result<Statistics, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const stats = await invoke<Statistics>('get_statistics', {
  start_date: '2025-11-01',
  end_date: '2025-11-30'
});
```

**レスポンス例:**
```json
{
  "daily": [
    { "date": "2025-11-13", "completed_count": 5, "total_count": 10 },
    { "date": "2025-11-14", "completed_count": 3, "total_count": 8 }
  ],
  "weekly": [
    { "week_start": "2025-11-10", "completed_count": 25, "total_count": 50 }
  ],
  "monthly": [
    { "month": "2025-11", "completed_count": 120, "total_count": 200 }
  ],
  "by_category": [
    {
      "category_id": 1,
      "category_name": "仕事",
      "color": "#3B82F6",
      "completed_count": 80,
      "total_count": 120
    }
  ]
}
```

### get_task_statistics

特定タスクの統計を取得

**Rust:**
```rust
#[tauri::command]
pub fn get_task_statistics(task_id: i64) -> Result<TaskStatistics, String> {
    // 実装
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStatistics {
    pub task_id: i64,
    pub title: String,
    pub total_completions: i32,
    pub completion_history: Vec<CompletionRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRecord {
    pub date: String,
    pub count: i32,
}
```

**TypeScript:**
```typescript
const taskStats = await invoke<TaskStatistics>('get_task_statistics', {
  task_id: 1
});
```

## 設定関連コマンド

### get_settings

アプリケーション設定を取得

**Rust:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,              // "light" | "dark"
    pub language: String,           // "ja" | "en"
    pub notifications_enabled: bool,
    pub background_mode: bool,      // バックグラウンド動作
}

#[tauri::command]
pub fn get_settings() -> Result<AppSettings, String> {
    // 実装（設定ファイルまたはDBから読み込み）
}
```

**TypeScript:**
```typescript
const settings = await invoke<AppSettings>('get_settings');
```

### update_settings

設定を更新

**Rust:**
```rust
#[tauri::command]
pub fn update_settings(settings: AppSettings) -> Result<AppSettings, String> {
    // 実装
}
```

**TypeScript:**
```typescript
const updated = await invoke<AppSettings>('update_settings', {
  settings: {
    theme: 'dark',
    language: 'en',
    notifications_enabled: true,
    background_mode: false
  }
});
```

## エラーハンドリング

### Rust側

```rust
// エラーは常にStringで返す
#[tauri::command]
pub fn get_task(id: i64) -> Result<Task, String> {
    db::get_task(id)
        .map_err(|e| format!("タスクの取得に失敗しました: {}", e))
}
```

### TypeScript側

```typescript
try {
  const task = await invoke<Task>('get_task', { id: 1 });
} catch (error) {
  console.error('エラー:', error);
  // ユーザーに通知表示
}
```

## 命名規則

- **Rust:** スネークケース (`get_tasks`, `create_category`)
- **TypeScript:** スネークケース（invoke時）、キャメルケース（変数名）
- **パラメータ:** すべてスネークケース
- **戻り値:** Rust型とTypeScript型の一貫性を保つ

## パフォーマンス考慮事項

- 大量データ取得時はページネーション実装を検討
- 統計クエリは集計済みデータをキャッシュ
- リアルタイム更新が必要な場合はTauriのEventシステムを使用

```rust
// イベント発行例
use tauri::Manager;

#[tauri::command]
pub fn create_task(
    app: tauri::AppHandle,
    title: String,
    category_id: i64
) -> Result<Task, String> {
    let task = db::create_task(title, category_id)?;

    // フロントエンドに通知
    app.emit_all("task-created", &task).unwrap();

    Ok(task)
}
```

```typescript
// TypeScript側でリッスン
import { listen } from '@tauri-apps/api/event';

listen<Task>('task-created', (event) => {
  console.log('新しいタスク:', event.payload);
  // Contextを更新
});
```
