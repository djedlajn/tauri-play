// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
use url::Url;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn navigate_webviews(url: String, window: tauri::Window) -> Result<(), String> {
    // Get app handle
    let app = window.app_handle();
    
    // Validate URL
    if Url::parse(&url).is_err() {
        return Err("Invalid URL format".to_string());
    }
    
    // Find the right window and navigate to the URL
    if let Some(right_window) = app.get_webview_window("right") {
        // Use JavaScript to navigate to the URL safely
        let js = format!("window.location.href = '{}';", url.replace("'", "\\'"));
        right_window.eval(&js)
            .map_err(|e| e.to_string())?;
        
        println!("Navigating right window to: {}", url);
    } else {
        return Err("Right window not found".to_string());
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Create a window for the left panel (control panel)
            let _left_window = tauri::WebviewWindowBuilder::new(
                app,
                "left", // the window label
                tauri::WebviewUrl::App(Default::default())
            )
            .title("Control Panel")
            .inner_size(300.0, 800.0)
            .position(0.0, 0.0)
            .build()
            .expect("failed to build left window");
            
            // Create a window for the right panel (web content)
            let _right_window = tauri::WebviewWindowBuilder::new(
                app,
                "right", // the window label
                tauri::WebviewUrl::External(Url::parse("https://google.com").unwrap())
            )
            .title("Web Content")
            .inner_size(900.0, 800.0)
            .position(300.0, 0.0)
            .build()
            .expect("failed to build right window");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, navigate_webviews])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
