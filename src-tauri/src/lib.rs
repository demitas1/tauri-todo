mod db;

use db::Database;
use tauri::{Manager, State};

/// カウンターの値を取得
#[tauri::command]
fn get_count(db: State<Database>) -> Result<i32, String> {
    db.get_count().map_err(|e| e.to_string())
}

/// カウンターを増加
#[tauri::command]
fn increment(db: State<Database>) -> Result<i32, String> {
    db.increment().map_err(|e| e.to_string())
}

/// カウンターを減少
#[tauri::command]
fn decrement(db: State<Database>) -> Result<i32, String> {
    db.decrement().map_err(|e| e.to_string())
}

/// カウンターをリセット
#[tauri::command]
fn reset(db: State<Database>) -> Result<i32, String> {
    db.reset().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // アプリのデータディレクトリを取得
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("アプリデータディレクトリの取得に失敗");

            // データベースを初期化
            let database = Database::new(app_data_dir)
                .expect("データベースの初期化に失敗");

            // 状態として管理
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_count,
            increment,
            decrement,
            reset
        ])
        .run(tauri::generate_context!())
        .expect("アプリケーションの実行中にエラーが発生");
}
