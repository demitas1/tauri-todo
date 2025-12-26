import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

const messages = [
  "ToDoアプリケーションへようこそ",
  "今日も一日頑張りましょう！",
  "タスクを整理して効率アップ",
  "小さな一歩が大きな成果に",
  "やるべきことを見える化しよう",
];

function App() {
  const [message, setMessage] = useState(messages[0]);
  const [count, setCount] = useState(0);

  // 初期化時にRustからカウンター値を取得
  useEffect(() => {
    invoke<number>("get_count").then(setCount);
  }, []);

  const handleRandomMessage = () => {
    const randomIndex = Math.floor(Math.random() * messages.length);
    setMessage(messages[randomIndex]);
  };

  // カウンター操作
  const handleIncrement = async () => {
    const newCount = await invoke<number>("increment");
    setCount(newCount);
  };

  const handleDecrement = async () => {
    const newCount = await invoke<number>("decrement");
    setCount(newCount);
  };

  const handleReset = async () => {
    const newCount = await invoke<number>("reset");
    setCount(newCount);
  };

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-gray-900">
      {/* ヘッダー */}
      <header className="bg-white dark:bg-gray-800 shadow">
        <div className="max-w-4xl mx-auto px-4 py-6">
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
            Tauri ToDo
          </h1>
        </div>
      </header>

      {/* メインコンテンツ */}
      <main className="max-w-4xl mx-auto px-4 py-8 space-y-6">
        {/* メッセージセクション */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <p className="text-gray-600 dark:text-gray-300">
            {message}
          </p>
          <button
            onClick={handleRandomMessage}
            className="mt-4 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
          >
            メッセージを変更
          </button>
        </div>

        {/* カウンターセクション（Rustバックエンド連携） */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
            カウンター（Rust連携）
          </h2>
          <div className="flex items-center justify-center gap-4">
            <button
              onClick={handleDecrement}
              className="w-12 h-12 bg-red-500 hover:bg-red-600 text-white text-2xl font-bold rounded-lg transition-colors"
            >
              -
            </button>
            <span className="text-4xl font-bold text-gray-900 dark:text-white min-w-[80px] text-center">
              {count}
            </span>
            <button
              onClick={handleIncrement}
              className="w-12 h-12 bg-green-500 hover:bg-green-600 text-white text-2xl font-bold rounded-lg transition-colors"
            >
              +
            </button>
          </div>
          <div className="mt-4 text-center">
            <button
              onClick={handleReset}
              className="px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white rounded-lg transition-colors"
            >
              リセット
            </button>
          </div>
          <p className="mt-4 text-sm text-gray-500 dark:text-gray-400 text-center">
            状態はRustバックエンドで管理されています
          </p>
        </div>
      </main>
    </div>
  );
}

export default App;
