# UIコンポーネント設計

## 概要

ミニマル＋モダンなデザインのReactコンポーネント構成。
Tailwind CSSを使用し、ダークモード対応、多言語対応を実装。

## デザインシステム

### カラーパレット

```typescript
// Tailwind設定で使用
const colors = {
  // ライトモード
  light: {
    primary: '#3B82F6',      // blue-500
    secondary: '#10B981',    // green-500
    accent: '#F59E0B',       // amber-500
    background: '#FFFFFF',
    surface: '#F3F4F6',      // gray-100
    text: '#1F2937',         // gray-800
    textSecondary: '#6B7280', // gray-500
    border: '#E5E7EB',       // gray-200
    error: '#EF4444',        // red-500
  },
  // ダークモード
  dark: {
    primary: '#60A5FA',      // blue-400
    secondary: '#34D399',    // green-400
    accent: '#FBBF24',       // amber-400
    background: '#111827',   // gray-900
    surface: '#1F2937',      // gray-800
    text: '#F9FAFB',         // gray-50
    textSecondary: '#9CA3AF', // gray-400
    border: '#374151',       // gray-700
    error: '#F87171',        // red-400
  },
};
```

### タイポグラフィ

```css
/* 見出し */
h1: text-3xl font-bold
h2: text-2xl font-semibold
h3: text-xl font-medium

/* 本文 */
body: text-base
small: text-sm
```

### スペーシング

```
小: p-2, m-2  (8px)
中: p-4, m-4  (16px)
大: p-6, m-6  (24px)
```

## コンポーネント階層

```
App
└── Layout
    ├── Header
    │   ├── Logo
    │   └── SettingsButton
    ├── Sidebar
    │   ├── CategoryList
    │   └── AddCategoryButton
    └── MainContent
        ├── TodoView (デフォルト)
        │   ├── TodoList
        │   │   ├── TodoItem
        │   │   └── AddTodoForm
        │   └── TodoDetail (選択時)
        │       ├── TaskInfo
        │       ├── ReminderSettings
        │       └── DeleteButton
        ├── StatisticsView
        │   ├── StatsSummary
        │   ├── DailyChart
        │   ├── WeeklyChart
        │   ├── MonthlyChart
        │   └── CategoryChart
        └── SettingsView
            ├── ThemeToggle
            ├── LanguageSelector
            ├── NotificationSettings
            └── BackgroundModeToggle
```

## 主要コンポーネント詳細

### 1. Layout

アプリ全体のレイアウト

```typescript
// src/components/Layout.tsx
import { FC, ReactNode } from 'react';
import Header from './Header';
import Sidebar from './Sidebar';

interface LayoutProps {
  children: ReactNode;
}

const Layout: FC<LayoutProps> = ({ children }) => {
  return (
    <div className="h-screen flex flex-col bg-background dark:bg-gray-900">
      {/* ヘッダー */}
      <Header />

      <div className="flex flex-1 overflow-hidden">
        {/* サイドバー */}
        <Sidebar />

        {/* メインコンテンツ */}
        <main className="flex-1 overflow-y-auto p-6">
          {children}
        </main>
      </div>
    </div>
  );
};

export default Layout;
```

### 2. Header

アプリケーションヘッダー

```typescript
// src/components/Header.tsx
import { FC } from 'react';
import { useTranslation } from 'react-i18next';
import { Settings } from 'lucide-react'; // アイコンライブラリ
import { useNavigate } from 'react-router-dom';

const Header: FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <header className="h-16 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 flex items-center justify-between">
      {/* ロゴ */}
      <div className="flex items-center space-x-2">
        <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
          <span className="text-white font-bold">T</span>
        </div>
        <h1 className="text-xl font-bold text-gray-800 dark:text-gray-100">
          {t('app.title')}
        </h1>
      </div>

      {/* 設定ボタン */}
      <button
        onClick={() => navigate('/settings')}
        className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition"
      >
        <Settings className="w-6 h-6 text-gray-600 dark:text-gray-300" />
      </button>
    </header>
  );
};

export default Header;
```

### 3. Sidebar

カテゴリ一覧サイドバー

```typescript
// src/components/Sidebar.tsx
import { FC, useState } from 'react';
import { useTodo } from '../contexts/TodoContext';
import { useTranslation } from 'react-i18next';
import { Plus, BarChart3 } from 'lucide-react';
import CategoryList from './CategoryList';
import AddCategoryModal from './AddCategoryModal';

const Sidebar: FC = () => {
  const { state } = useTodo();
  const { t } = useTranslation();
  const [isModalOpen, setIsModalOpen] = useState(false);

  return (
    <aside className="w-64 bg-gray-50 dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 overflow-y-auto">
      {/* ナビゲーション */}
      <nav className="p-4 space-y-2">
        <button className="w-full text-left px-4 py-2 rounded-lg bg-primary text-white">
          {t('nav.tasks')}
        </button>
        <button className="w-full text-left px-4 py-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 flex items-center space-x-2">
          <BarChart3 className="w-5 h-5" />
          <span>{t('nav.statistics')}</span>
        </button>
      </nav>

      {/* カテゴリリスト */}
      <div className="p-4">
        <div className="flex items-center justify-between mb-2">
          <h3 className="text-sm font-semibold text-gray-600 dark:text-gray-400">
            {t('categories.title')}
          </h3>
          <button
            onClick={() => setIsModalOpen(true)}
            className="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700"
          >
            <Plus className="w-4 h-4" />
          </button>
        </div>
        <CategoryList categories={state.categories} />
      </div>

      <AddCategoryModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
      />
    </aside>
  );
};

export default Sidebar;
```

### 4. TodoList

タスク一覧表示

```typescript
// src/components/TodoList.tsx
import { FC, useState } from 'react';
import { useTodo } from '../contexts/TodoContext';
import { useTranslation } from 'react-i18next';
import TodoItem from './TodoItem';
import AddTodoForm from './AddTodoForm';

const TodoList: FC = () => {
  const { state } = useTodo();
  const { t } = useTranslation();
  const [filter, setFilter] = useState<'all' | 'active' | 'completed'>('all');

  const filteredTasks = state.tasks.filter((task) => {
    if (filter === 'active') return !task.completed;
    if (filter === 'completed') return task.completed;
    return true;
  });

  return (
    <div className="max-w-4xl mx-auto">
      {/* フィルターボタン */}
      <div className="flex space-x-2 mb-4">
        {(['all', 'active', 'completed'] as const).map((f) => (
          <button
            key={f}
            onClick={() => setFilter(f)}
            className={`px-4 py-2 rounded-lg transition ${
              filter === f
                ? 'bg-primary text-white'
                : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
            }`}
          >
            {t(`filter.${f}`)}
          </button>
        ))}
      </div>

      {/* タスク追加フォーム */}
      <AddTodoForm />

      {/* タスクリスト */}
      <div className="space-y-2 mt-4">
        {state.loading ? (
          <p className="text-center text-gray-500">{t('loading')}</p>
        ) : filteredTasks.length === 0 ? (
          <p className="text-center text-gray-500">{t('tasks.empty')}</p>
        ) : (
          filteredTasks.map((task) => <TodoItem key={task.id} task={task} />)
        )}
      </div>
    </div>
  );
};

export default TodoList;
```

### 5. TodoItem

個別タスクアイテム

```typescript
// src/components/TodoItem.tsx
import { FC } from 'react';
import { Task } from '../types';
import { useTodo } from '../contexts/TodoContext';
import { Trash2, Bell } from 'lucide-react';

interface TodoItemProps {
  task: Task;
}

const TodoItem: FC<TodoItemProps> = ({ task }) => {
  const { toggleTask, deleteTask, state } = useTodo();

  const category = state.categories.find((c) => c.id === task.category_id);

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700 flex items-center space-x-4 hover:shadow-md transition">
      {/* チェックボックス */}
      <input
        type="checkbox"
        checked={task.completed}
        onChange={() => toggleTask(task.id)}
        className="w-5 h-5 rounded border-gray-300 text-primary focus:ring-primary"
      />

      {/* タスク情報 */}
      <div className="flex-1">
        <h4
          className={`font-medium ${
            task.completed
              ? 'line-through text-gray-400'
              : 'text-gray-800 dark:text-gray-100'
          }`}
        >
          {task.title}
        </h4>
        {category && (
          <span
            className="inline-block px-2 py-1 text-xs rounded mt-1"
            style={{ backgroundColor: category.color + '20', color: category.color }}
          >
            {category.name}
          </span>
        )}
      </div>

      {/* アクションボタン */}
      <div className="flex space-x-2">
        <button className="p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700">
          <Bell className="w-5 h-5 text-gray-500" />
        </button>
        <button
          onClick={() => deleteTask(task.id)}
          className="p-2 rounded hover:bg-red-50 dark:hover:bg-red-900/20"
        >
          <Trash2 className="w-5 h-5 text-red-500" />
        </button>
      </div>
    </div>
  );
};

export default TodoItem;
```

### 6. AddTodoForm

タスク追加フォーム

```typescript
// src/components/AddTodoForm.tsx
import { FC, useState } from 'react';
import { useTodo } from '../contexts/TodoContext';
import { useTranslation } from 'react-i18next';
import { Plus } from 'lucide-react';

const AddTodoForm: FC = () => {
  const { createTask, state } = useTodo();
  const { t } = useTranslation();
  const [title, setTitle] = useState('');
  const [categoryId, setCategoryId] = useState<number>(1);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim()) return;

    await createTask(title, categoryId);
    setTitle('');
  };

  return (
    <form onSubmit={handleSubmit} className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700">
      <div className="flex space-x-2">
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder={t('tasks.addPlaceholder')}
          className="flex-1 px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-primary"
        />
        <select
          value={categoryId}
          onChange={(e) => setCategoryId(Number(e.target.value))}
          className="px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-primary"
        >
          {state.categories.map((cat) => (
            <option key={cat.id} value={cat.id}>
              {cat.name}
            </option>
          ))}
        </select>
        <button
          type="submit"
          className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-blue-600 transition flex items-center space-x-1"
        >
          <Plus className="w-5 h-5" />
          <span>{t('tasks.add')}</span>
        </button>
      </div>
    </form>
  );
};

export default AddTodoForm;
```

### 7. StatisticsView

統計グラフビュー

```typescript
// src/components/StatisticsView.tsx
import { FC, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { Statistics } from '../types';
import {
  BarChart,
  Bar,
  LineChart,
  Line,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';

const StatisticsView: FC = () => {
  const { t } = useTranslation();
  const [stats, setStats] = useState<Statistics | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadStats();
  }, []);

  const loadStats = async () => {
    try {
      const endDate = new Date().toISOString().split('T')[0];
      const startDate = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000)
        .toISOString()
        .split('T')[0];

      const data = await invoke<Statistics>('get_statistics', {
        start_date: startDate,
        end_date: endDate,
      });
      setStats(data);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) return <p>{t('loading')}</p>;
  if (!stats) return <p>{t('stats.noData')}</p>;

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      {/* サマリー */}
      <div className="grid grid-cols-3 gap-4">
        <div className="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
          <h3 className="text-sm text-gray-600 dark:text-gray-400">{t('stats.daily')}</h3>
          <p className="text-3xl font-bold text-primary mt-2">
            {stats.daily[0]?.completed_count || 0}
          </p>
        </div>
        {/* 他のサマリーカードも同様 */}
      </div>

      {/* 日別グラフ */}
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
        <h3 className="text-lg font-semibold mb-4">{t('stats.dailyChart')}</h3>
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={stats.daily}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="date" />
            <YAxis />
            <Tooltip />
            <Legend />
            <Line type="monotone" dataKey="completed_count" stroke="#3B82F6" />
          </LineChart>
        </ResponsiveContainer>
      </div>

      {/* カテゴリ別円グラフ */}
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
        <h3 className="text-lg font-semibold mb-4">{t('stats.byCategory')}</h3>
        <ResponsiveContainer width="100%" height={300}>
          <PieChart>
            <Pie
              data={stats.by_category}
              dataKey="completed_count"
              nameKey="category_name"
              cx="50%"
              cy="50%"
              outerRadius={100}
              label
            >
              {stats.by_category.map((entry, index) => (
                <Cell key={`cell-${index}`} fill={entry.color} />
              ))}
            </Pie>
            <Tooltip />
            <Legend />
          </PieChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};

export default StatisticsView;
```

### 8. SettingsView

設定画面

```typescript
// src/components/SettingsView.tsx
import { FC } from 'react';
import { useSettings } from '../contexts/SettingsContext';
import { useTranslation } from 'react-i18next';
import { Sun, Moon } from 'lucide-react';

const SettingsView: FC = () => {
  const { state, setTheme, setLanguage, toggleNotifications, toggleBackgroundMode } = useSettings();
  const { t, i18n } = useTranslation();

  const handleLanguageChange = (lang: 'ja' | 'en') => {
    setLanguage(lang);
    i18n.changeLanguage(lang);
  };

  return (
    <div className="max-w-2xl mx-auto space-y-6">
      <h2 className="text-2xl font-bold">{t('settings.title')}</h2>

      {/* テーマ設定 */}
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
        <h3 className="font-semibold mb-4">{t('settings.theme')}</h3>
        <div className="flex space-x-2">
          <button
            onClick={() => setTheme('light')}
            className={`flex-1 px-4 py-2 rounded-lg flex items-center justify-center space-x-2 ${
              state.theme === 'light'
                ? 'bg-primary text-white'
                : 'bg-gray-200 dark:bg-gray-700'
            }`}
          >
            <Sun className="w-5 h-5" />
            <span>{t('settings.light')}</span>
          </button>
          <button
            onClick={() => setTheme('dark')}
            className={`flex-1 px-4 py-2 rounded-lg flex items-center justify-center space-x-2 ${
              state.theme === 'dark'
                ? 'bg-primary text-white'
                : 'bg-gray-200 dark:bg-gray-700'
            }`}
          >
            <Moon className="w-5 h-5" />
            <span>{t('settings.dark')}</span>
          </button>
        </div>
      </div>

      {/* 言語設定 */}
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
        <h3 className="font-semibold mb-4">{t('settings.language')}</h3>
        <select
          value={state.language}
          onChange={(e) => handleLanguageChange(e.target.value as 'ja' | 'en')}
          className="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700"
        >
          <option value="ja">日本語</option>
          <option value="en">English</option>
        </select>
      </div>

      {/* 通知設定 */}
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="font-semibold">{t('settings.notifications')}</h3>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {t('settings.notificationsDesc')}
            </p>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={state.notificationsEnabled}
              onChange={toggleNotifications}
              className="sr-only peer"
            />
            <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary/30 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-primary"></div>
          </label>
        </div>
      </div>

      {/* バックグラウンドモード */}
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="font-semibold">{t('settings.backgroundMode')}</h3>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {t('settings.backgroundModeDesc')}
            </p>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={state.backgroundMode}
              onChange={toggleBackgroundMode}
              className="sr-only peer"
            />
            <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary/30 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-primary"></div>
          </label>
        </div>
      </div>
    </div>
  );
};

export default SettingsView;
```

## 共通コンポーネント

### Button

```typescript
// src/components/common/Button.tsx
import { FC, ReactNode } from 'react';

interface ButtonProps {
  children: ReactNode;
  onClick?: () => void;
  variant?: 'primary' | 'secondary' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
}

const Button: FC<ButtonProps> = ({
  children,
  onClick,
  variant = 'primary',
  size = 'md',
  disabled = false,
}) => {
  const baseClass = 'rounded-lg font-medium transition';

  const variantClass = {
    primary: 'bg-primary text-white hover:bg-blue-600',
    secondary: 'bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-100 hover:bg-gray-300',
    danger: 'bg-red-500 text-white hover:bg-red-600',
  }[variant];

  const sizeClass = {
    sm: 'px-3 py-1 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg',
  }[size];

  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`${baseClass} ${variantClass} ${sizeClass} ${
        disabled ? 'opacity-50 cursor-not-allowed' : ''
      }`}
    >
      {children}
    </button>
  );
};

export default Button;
```

## アイコンライブラリ

**lucide-react** を使用

```bash
pnpm add lucide-react
```

主要アイコン:
- Plus: 追加
- Trash2: 削除
- Bell: リマインダー
- Settings: 設定
- Sun/Moon: テーマ切り替え
- BarChart3: 統計
- Check: 完了

## レスポンシブ対応

デスクトップ優先設計だが、将来的なモバイル対応を考慮:

```typescript
// ブレークポイント
sm: 640px  (タブレット)
md: 768px  (小型ノートPC)
lg: 1024px (デスクトップ)
```

## アニメーション

Tailwind CSSのtransitionユーティリティを使用:

```css
/* ホバー効果 */
hover:bg-gray-100 transition

/* フェードイン */
animate-fade-in

/* スライドイン */
animate-slide-in
```

## アクセシビリティ

- すべてのボタンにaria-label
- キーボード操作対応（Tab, Enter, Space）
- スクリーンリーダー対応
- 十分なコントラスト比（WCAG AA準拠）

```typescript
<button
  aria-label={t('tasks.delete')}
  onClick={handleDelete}
>
  <Trash2 />
</button>
```
