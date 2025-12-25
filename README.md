# Tauri ToDo

リマインダー機能付きToDoアプリケーション。Tauri 2.xを使用し、デスクトップ（Windows/macOS/Linux）およびモバイル（iOS/Android）に対応。

## 技術スタック

- **フロントエンド**: React 19, TypeScript, Tailwind CSS
- **バックエンド**: Rust (Tauri 2.x)
- **データベース**: SQLite

## 必要な環境

### 共通
- [Rust](https://www.rust-lang.org/tools/install) 1.70以上
- [Node.js](https://nodejs.org/) 18以上
- [pnpm](https://pnpm.io/installation)

### プラットフォーム別

**Linux**
```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
```

**Windows**
- WebView2（通常プリインストール済み）

**macOS**
```bash
xcode-select --install
```

## セットアップ

```bash
# リポジトリのクローン
git clone <repository-url>
cd tauri-todo-app

# 依存関係のインストール
pnpm install
```

## 開発

```bash
# 開発サーバーの起動（ホットリロード対応）
pnpm tauri dev
```

## ビルド

### デスクトップ

```bash
pnpm tauri build
```

出力先: `src-tauri/target/release/bundle/`
- **Windows**: `.exe`, `.msi`
- **macOS**: `.app`, `.dmg`
- **Linux**: `.AppImage`, `.deb`

### モバイル

**Android**
```bash
# 初回のみ
pnpm tauri android init

# ビルド
pnpm tauri android build
```

**iOS**（macOSのみ）
```bash
# 初回のみ
pnpm tauri ios init

# ビルド
pnpm tauri ios build
```

## プロジェクト構成

```
tauri-todo-app/
├── src/                    # フロントエンド（React）
│   ├── components/         # UIコンポーネント
│   ├── contexts/           # React Context
│   ├── hooks/              # カスタムフック
│   └── types/              # 型定義
├── src-tauri/              # バックエンド（Rust）
│   ├── src/
│   │   ├── main.rs         # エントリーポイント
│   │   ├── commands.rs     # Tauriコマンド
│   │   ├── db.rs           # SQLite操作
│   │   └── scheduler.rs    # リマインダー
│   └── tauri.conf.json     # Tauri設定
└── docs/                   # ドキュメント
```

## 推奨エディタ設定

- [VS Code](https://code.visualstudio.com/)
- [Tauri拡張機能](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## ライセンス

MIT
