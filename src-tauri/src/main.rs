// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, port: &str) -> String {
    format!("Begin to connect to server {}:{}...", name, port)
}
#[tauri::command]
fn add_rule(src_ip: &str, dst_ip: &str,src_port:&str,dst_port:&str,protocol:&str) -> String {
    format!("receive rule {},{},{},{},{}...", src_ip, dst_ip,src_port,dst_port,protocol)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,open_new_window,add_rule])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
//参数handle这样写才能成功调用
#[tauri::command]
async fn open_new_window(handle:tauri::AppHandle) {
    let new_window = tauri::WindowBuilder::new(
        &handle,
        "new_window",
        tauri::WindowUrl::App("rulecfg.html".into()),
    )
   .title("规则配置")
   .build()
   .expect("Failed to create new window");
    new_window.show().expect("Failed to show new window");
}
