// Tauri v2 桌面悬浮窗应用入口
// 课堂抽问随机点名应用

use tauri::Manager;

/// 自定义命令：彻底退出应用进程
/// 在 Tauri v2 中没有 core:process:allow-exit 权限，
/// 需要通过自定义 Rust 命令来实现进程退出
#[tauri::command]
fn exit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![exit_app])
        .setup(|app| {
            // 获取主窗口
            let window = app.get_webview_window("main").unwrap();
            
            // 设置窗口属性
            window.set_decorations(false).unwrap_or_default();
            window.set_always_on_top(true).unwrap_or_default();
            
            println!("[Tauri] 应用启动成功，窗口已设置为无边框和始终置顶");
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用时发生错误");
}
