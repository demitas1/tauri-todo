import { useState } from "react";
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

  const handleRandomMessage = () => {
    const randomIndex = Math.floor(Math.random() * messages.length);
    setMessage(messages[randomIndex]);
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
      <main className="max-w-4xl mx-auto px-4 py-8">
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
      </main>
    </div>
  );
}

export default App;
