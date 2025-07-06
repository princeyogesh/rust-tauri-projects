// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn inital_expense(dummyarg: &str) -> i32 {
    50
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![inital_expense])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
