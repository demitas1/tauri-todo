use std::sync::Mutex;
use tauri::State;

// カウンターの状態を管理する構造体
struct CounterState {
    count: i32,
}

// カウンターの値を取得
#[tauri::command]
fn get_count(state: State<Mutex<CounterState>>) -> i32 {
    state.lock().unwrap().count
}

// カウンターを増加
#[tauri::command]
fn increment(state: State<Mutex<CounterState>>) -> i32 {
    let mut counter = state.lock().unwrap();
    counter.count += 1;
    counter.count
}

// カウンターを減少
#[tauri::command]
fn decrement(state: State<Mutex<CounterState>>) -> i32 {
    let mut counter = state.lock().unwrap();
    counter.count -= 1;
    counter.count
}

// カウンターをリセット
#[tauri::command]
fn reset(state: State<Mutex<CounterState>>) -> i32 {
    let mut counter = state.lock().unwrap();
    counter.count = 0;
    counter.count
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(CounterState { count: 0 }))
        .invoke_handler(tauri::generate_handler![get_count, increment, decrement, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
