# 状態管理設計

## 概要

React Context API + useReducerを使用した状態管理。
グローバル状態を3つのContextに分割し、各Contextが独立したReducerで状態を管理する。

## Context構成

```
App
 └── SettingsProvider
      └── TodoProvider
           └── ReminderProvider
                └── Router/Views
```

## 1. TodoContext

タスク管理の状態とアクション

### State定義

```typescript
// src/contexts/TodoContext.tsx

export interface TodoState {
  tasks: Task[];
  categories: Category[];
  loading: boolean;
  error: string | null;
}

const initialState: TodoState = {
  tasks: [],
  categories: [],
  loading: false,
  error: null,
};
```

### Action定義

```typescript
export type TodoAction =
  | { type: 'SET_LOADING'; payload: boolean }
  | { type: 'SET_ERROR'; payload: string | null }
  | { type: 'SET_TASKS'; payload: Task[] }
  | { type: 'SET_CATEGORIES'; payload: Category[] }
  | { type: 'ADD_TASK'; payload: Task }
  | { type: 'UPDATE_TASK'; payload: Task }
  | { type: 'DELETE_TASK'; payload: number } // task_id
  | { type: 'TOGGLE_TASK'; payload: Task }
  | { type: 'ADD_CATEGORY'; payload: Category }
  | { type: 'UPDATE_CATEGORY'; payload: Category }
  | { type: 'DELETE_CATEGORY'; payload: number }; // category_id
```

### Reducer実装

```typescript
const todoReducer = (state: TodoState, action: TodoAction): TodoState => {
  switch (action.type) {
    case 'SET_LOADING':
      return { ...state, loading: action.payload };

    case 'SET_ERROR':
      return { ...state, error: action.payload };

    case 'SET_TASKS':
      return { ...state, tasks: action.payload, loading: false };

    case 'SET_CATEGORIES':
      return { ...state, categories: action.payload, loading: false };

    case 'ADD_TASK':
      return {
        ...state,
        tasks: [...state.tasks, action.payload],
        loading: false,
      };

    case 'UPDATE_TASK':
      return {
        ...state,
        tasks: state.tasks.map((task) =>
          task.id === action.payload.id ? action.payload : task
        ),
        loading: false,
      };

    case 'DELETE_TASK':
      return {
        ...state,
        tasks: state.tasks.filter((task) => task.id !== action.payload),
        loading: false,
      };

    case 'TOGGLE_TASK':
      return {
        ...state,
        tasks: state.tasks.map((task) =>
          task.id === action.payload.id ? action.payload : task
        ),
        loading: false,
      };

    case 'ADD_CATEGORY':
      return {
        ...state,
        categories: [...state.categories, action.payload],
        loading: false,
      };

    case 'UPDATE_CATEGORY':
      return {
        ...state,
        categories: state.categories.map((cat) =>
          cat.id === action.payload.id ? action.payload : cat
        ),
        loading: false,
      };

    case 'DELETE_CATEGORY':
      return {
        ...state,
        categories: state.categories.filter((cat) => cat.id !== action.payload),
        tasks: state.tasks.filter((task) => task.category_id !== action.payload),
        loading: false,
      };

    default:
      return state;
  }
};
```

### Provider実装

```typescript
import React, { createContext, useContext, useReducer, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Task, Category } from '../types';

interface TodoContextType {
  state: TodoState;
  dispatch: React.Dispatch<TodoAction>;
  // ヘルパー関数
  fetchTasks: () => Promise<void>;
  fetchCategories: () => Promise<void>;
  createTask: (title: string, categoryId: number) => Promise<void>;
  updateTask: (id: number, title: string, categoryId: number) => Promise<void>;
  toggleTask: (id: number) => Promise<void>;
  deleteTask: (id: number) => Promise<void>;
  createCategory: (name: string, color: string) => Promise<void>;
  updateCategory: (id: number, name: string, color: string) => Promise<void>;
  deleteCategory: (id: number) => Promise<void>;
}

const TodoContext = createContext<TodoContextType | undefined>(undefined);

export const TodoProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [state, dispatch] = useReducer(todoReducer, initialState);

  // 初回読み込み
  useEffect(() => {
    fetchTasks();
    fetchCategories();
  }, []);

  const fetchTasks = async () => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const tasks = await invoke<Task[]>('get_tasks', {
        category_id: null,
        completed: null,
      });
      dispatch({ type: 'SET_TASKS', payload: tasks });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  const fetchCategories = async () => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const categories = await invoke<Category[]>('get_categories');
      dispatch({ type: 'SET_CATEGORIES', payload: categories });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  const createTask = async (title: string, categoryId: number) => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const task = await invoke<Task>('create_task', {
        title,
        category_id: categoryId,
      });
      dispatch({ type: 'ADD_TASK', payload: task });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  const toggleTask = async (id: number) => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const task = await invoke<Task>('toggle_task', { id });
      dispatch({ type: 'TOGGLE_TASK', payload: task });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  const deleteTask = async (id: number) => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      await invoke('delete_task', { id });
      dispatch({ type: 'DELETE_TASK', payload: id });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  // 他のヘルパー関数も同様に実装...

  return (
    <TodoContext.Provider
      value={{
        state,
        dispatch,
        fetchTasks,
        fetchCategories,
        createTask,
        updateTask,
        toggleTask,
        deleteTask,
        createCategory,
        updateCategory,
        deleteCategory,
      }}
    >
      {children}
    </TodoContext.Provider>
  );
};

export const useTodo = () => {
  const context = useContext(TodoContext);
  if (!context) {
    throw new Error('useTodo must be used within TodoProvider');
  }
  return context;
};
```

## 2. ReminderContext

リマインダー管理の状態とアクション

### State定義

```typescript
// src/contexts/ReminderContext.tsx

export interface ReminderState {
  reminders: Reminder[];
  loading: boolean;
  error: string | null;
}

const initialState: ReminderState = {
  reminders: [],
  loading: false,
  error: null,
};
```

### Action定義

```typescript
export type ReminderAction =
  | { type: 'SET_LOADING'; payload: boolean }
  | { type: 'SET_ERROR'; payload: string | null }
  | { type: 'SET_REMINDERS'; payload: Reminder[] }
  | { type: 'ADD_REMINDER'; payload: Reminder }
  | { type: 'UPDATE_REMINDER'; payload: Reminder }
  | { type: 'DELETE_REMINDER'; payload: number } // reminder_id
  | { type: 'TOGGLE_REMINDER'; payload: Reminder };
```

### Reducer実装

```typescript
const reminderReducer = (
  state: ReminderState,
  action: ReminderAction
): ReminderState => {
  switch (action.type) {
    case 'SET_LOADING':
      return { ...state, loading: action.payload };

    case 'SET_ERROR':
      return { ...state, error: action.payload };

    case 'SET_REMINDERS':
      return { ...state, reminders: action.payload, loading: false };

    case 'ADD_REMINDER':
      return {
        ...state,
        reminders: [...state.reminders, action.payload],
        loading: false,
      };

    case 'UPDATE_REMINDER':
      return {
        ...state,
        reminders: state.reminders.map((reminder) =>
          reminder.id === action.payload.id ? action.payload : reminder
        ),
        loading: false,
      };

    case 'DELETE_REMINDER':
      return {
        ...state,
        reminders: state.reminders.filter(
          (reminder) => reminder.id !== action.payload
        ),
        loading: false,
      };

    case 'TOGGLE_REMINDER':
      return {
        ...state,
        reminders: state.reminders.map((reminder) =>
          reminder.id === action.payload.id ? action.payload : reminder
        ),
        loading: false,
      };

    default:
      return state;
  }
};
```

### Provider実装

```typescript
interface ReminderContextType {
  state: ReminderState;
  dispatch: React.Dispatch<ReminderAction>;
  fetchReminders: (taskId: number) => Promise<void>;
  createReminder: (
    taskId: number,
    time: string,
    frequency: Reminder['frequency'],
    customDays?: string[]
  ) => Promise<void>;
  updateReminder: (
    id: number,
    time: string,
    frequency: Reminder['frequency'],
    customDays?: string[]
  ) => Promise<void>;
  toggleReminder: (id: number) => Promise<void>;
  deleteReminder: (id: number) => Promise<void>;
}

const ReminderContext = createContext<ReminderContextType | undefined>(
  undefined
);

export const ReminderProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [state, dispatch] = useReducer(reminderReducer, initialState);

  const fetchReminders = async (taskId: number) => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const reminders = await invoke<Reminder[]>('get_reminders', {
        task_id: taskId,
      });
      dispatch({ type: 'SET_REMINDERS', payload: reminders });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  const createReminder = async (
    taskId: number,
    time: string,
    frequency: Reminder['frequency'],
    customDays?: string[]
  ) => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const reminder = await invoke<Reminder>('create_reminder', {
        task_id: taskId,
        time,
        frequency,
        custom_days: customDays || null,
      });
      dispatch({ type: 'ADD_REMINDER', payload: reminder });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  const toggleReminder = async (id: number) => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const reminder = await invoke<Reminder>('toggle_reminder', { id });
      dispatch({ type: 'TOGGLE_REMINDER', payload: reminder });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  // 他のヘルパー関数も実装...

  return (
    <ReminderContext.Provider
      value={{
        state,
        dispatch,
        fetchReminders,
        createReminder,
        updateReminder,
        toggleReminder,
        deleteReminder,
      }}
    >
      {children}
    </ReminderContext.Provider>
  );
};

export const useReminder = () => {
  const context = useContext(ReminderContext);
  if (!context) {
    throw new Error('useReminder must be used within ReminderProvider');
  }
  return context;
};
```

## 3. SettingsContext

アプリケーション設定の状態とアクション

### State定義

```typescript
// src/contexts/SettingsContext.tsx

export interface SettingsState {
  theme: 'light' | 'dark';
  language: 'ja' | 'en';
  notificationsEnabled: boolean;
  backgroundMode: boolean;
  loading: boolean;
  error: string | null;
}

const initialState: SettingsState = {
  theme: 'light',
  language: 'ja',
  notificationsEnabled: true,
  backgroundMode: false,
  loading: false,
  error: null,
};
```

### Action定義

```typescript
export type SettingsAction =
  | { type: 'SET_LOADING'; payload: boolean }
  | { type: 'SET_ERROR'; payload: string | null }
  | { type: 'SET_SETTINGS'; payload: Partial<SettingsState> }
  | { type: 'SET_THEME'; payload: 'light' | 'dark' }
  | { type: 'SET_LANGUAGE'; payload: 'ja' | 'en' }
  | { type: 'TOGGLE_NOTIFICATIONS' }
  | { type: 'TOGGLE_BACKGROUND_MODE' };
```

### Reducer実装

```typescript
const settingsReducer = (
  state: SettingsState,
  action: SettingsAction
): SettingsState => {
  switch (action.type) {
    case 'SET_LOADING':
      return { ...state, loading: action.payload };

    case 'SET_ERROR':
      return { ...state, error: action.payload };

    case 'SET_SETTINGS':
      return { ...state, ...action.payload, loading: false };

    case 'SET_THEME':
      return { ...state, theme: action.payload };

    case 'SET_LANGUAGE':
      return { ...state, language: action.payload };

    case 'TOGGLE_NOTIFICATIONS':
      return { ...state, notificationsEnabled: !state.notificationsEnabled };

    case 'TOGGLE_BACKGROUND_MODE':
      return { ...state, backgroundMode: !state.backgroundMode };

    default:
      return state;
  }
};
```

### Provider実装

```typescript
interface SettingsContextType {
  state: SettingsState;
  dispatch: React.Dispatch<SettingsAction>;
  loadSettings: () => Promise<void>;
  saveSettings: () => Promise<void>;
  setTheme: (theme: 'light' | 'dark') => void;
  setLanguage: (language: 'ja' | 'en') => void;
  toggleNotifications: () => void;
  toggleBackgroundMode: () => void;
}

const SettingsContext = createContext<SettingsContextType | undefined>(
  undefined
);

export const SettingsProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [state, dispatch] = useReducer(settingsReducer, initialState);

  // 初回読み込み
  useEffect(() => {
    loadSettings();
  }, []);

  // 設定変更時に自動保存
  useEffect(() => {
    if (!state.loading) {
      saveSettings();
      // テーマ適用
      document.documentElement.classList.toggle('dark', state.theme === 'dark');
    }
  }, [state.theme, state.language, state.notificationsEnabled, state.backgroundMode]);

  const loadSettings = async () => {
    try {
      dispatch({ type: 'SET_LOADING', payload: true });
      const settings = await invoke<AppSettings>('get_settings');
      dispatch({
        type: 'SET_SETTINGS',
        payload: {
          theme: settings.theme as 'light' | 'dark',
          language: settings.language as 'ja' | 'en',
          notificationsEnabled: settings.notifications_enabled,
          backgroundMode: settings.background_mode,
        },
      });
    } catch (error) {
      dispatch({ type: 'SET_ERROR', payload: String(error) });
    }
  };

  const saveSettings = async () => {
    try {
      await invoke('update_settings', {
        settings: {
          theme: state.theme,
          language: state.language,
          notifications_enabled: state.notificationsEnabled,
          background_mode: state.backgroundMode,
        },
      });
    } catch (error) {
      console.error('設定の保存に失敗:', error);
    }
  };

  const setTheme = (theme: 'light' | 'dark') => {
    dispatch({ type: 'SET_THEME', payload: theme });
  };

  const setLanguage = (language: 'ja' | 'en') => {
    dispatch({ type: 'SET_LANGUAGE', payload: language });
  };

  const toggleNotifications = () => {
    dispatch({ type: 'TOGGLE_NOTIFICATIONS' });
  };

  const toggleBackgroundMode = () => {
    dispatch({ type: 'TOGGLE_BACKGROUND_MODE' });
  };

  return (
    <SettingsContext.Provider
      value={{
        state,
        dispatch,
        loadSettings,
        saveSettings,
        setTheme,
        setLanguage,
        toggleNotifications,
        toggleBackgroundMode,
      }}
    >
      {children}
    </SettingsContext.Provider>
  );
};

export const useSettings = () => {
  const context = useContext(SettingsContext);
  if (!context) {
    throw new Error('useSettings must be used within SettingsProvider');
  }
  return context;
};
```

## 使用例

### コンポーネントでの使用

```typescript
// src/components/TodoList.tsx
import { useTodo } from '../contexts/TodoContext';
import { useSettings } from '../contexts/SettingsContext';

const TodoList: React.FC = () => {
  const { state, createTask, toggleTask, deleteTask } = useTodo();
  const { state: settings } = useSettings();

  const handleAddTask = async () => {
    await createTask('新しいタスク', 1);
  };

  return (
    <div>
      {state.loading && <p>読み込み中...</p>}
      {state.error && <p className="text-red-500">{state.error}</p>}

      <button onClick={handleAddTask}>タスク追加</button>

      <ul>
        {state.tasks.map((task) => (
          <li key={task.id}>
            <input
              type="checkbox"
              checked={task.completed}
              onChange={() => toggleTask(task.id)}
            />
            <span>{task.title}</span>
            <button onClick={() => deleteTask(task.id)}>削除</button>
          </li>
        ))}
      </ul>
    </div>
  );
};
```

## 設計原則

### 1. 単一責任の原則
- 各Contextは特定のドメインのみを管理
- TodoContext: タスクとカテゴリ
- ReminderContext: リマインダー
- SettingsContext: アプリ設定

### 2. 予測可能な状態更新
- すべての状態変更はActionを通じて行う
- Reducerは純粋関数として実装

### 3. 型安全性
- TypeScriptの型システムを活用
- すべてのActionに明示的な型定義

### 4. エラーハンドリング
- すべての非同期処理でtry-catch
- エラーはstateに保存してUIで表示

### 5. パフォーマンス最適化
- Context分割により不要な再レンダリングを防止
- useMemoやuseCallbackを適宜使用

```typescript
// 最適化例
const filteredTasks = useMemo(() => {
  return state.tasks.filter((task) => !task.completed);
}, [state.tasks]);
```

## テスト戦略

### Reducerのユニットテスト

```typescript
// __tests__/todoReducer.test.ts
import { todoReducer } from '../contexts/TodoContext';

describe('todoReducer', () => {
  it('ADD_TASK should add a new task', () => {
    const state = { tasks: [], categories: [], loading: false, error: null };
    const action = {
      type: 'ADD_TASK' as const,
      payload: { id: 1, title: 'Test', category_id: 1, completed: false },
    };

    const newState = todoReducer(state, action);

    expect(newState.tasks).toHaveLength(1);
    expect(newState.tasks[0].title).toBe('Test');
  });
});
```
