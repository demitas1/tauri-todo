use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

/// データベース接続を管理する構造体
pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    /// データベースを初期化して接続を返す
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        // ディレクトリが存在しない場合は作成
        std::fs::create_dir_all(&app_data_dir).ok();

        let db_path = app_data_dir.join("app.db");
        let conn = Connection::open(db_path)?;

        // スキーマを初期化
        Self::init_schema(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// データベーススキーマを初期化
    fn init_schema(conn: &Connection) -> Result<()> {
        // カウンターテーブル（key-value形式で汎用的に使用可能）
        conn.execute(
            "CREATE TABLE IF NOT EXISTS counter (
                key TEXT PRIMARY KEY,
                value INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        // デフォルトカウンターが存在しない場合は作成
        conn.execute(
            "INSERT OR IGNORE INTO counter (key, value) VALUES ('main', 0)",
            [],
        )?;

        Ok(())
    }

    /// カウンター値を取得
    pub fn get_count(&self) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT value FROM counter WHERE key = 'main'",
            [],
            |row| row.get(0),
        )
    }

    /// カウンター値を設定
    pub fn set_count(&self, value: i32) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE counter SET value = ?1 WHERE key = 'main'",
            [value],
        )?;
        Ok(value)
    }

    /// カウンターをインクリメント
    pub fn increment(&self) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE counter SET value = value + 1 WHERE key = 'main'",
            [],
        )?;
        conn.query_row(
            "SELECT value FROM counter WHERE key = 'main'",
            [],
            |row| row.get(0),
        )
    }

    /// カウンターをデクリメント
    pub fn decrement(&self) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE counter SET value = value - 1 WHERE key = 'main'",
            [],
        )?;
        conn.query_row(
            "SELECT value FROM counter WHERE key = 'main'",
            [],
            |row| row.get(0),
        )
    }

    /// カウンターをリセット
    pub fn reset(&self) -> Result<i32> {
        self.set_count(0)
    }
}
