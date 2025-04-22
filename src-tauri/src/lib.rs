// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Manager, Emitter, Listener};
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
        
        // Emit the URL change event to the left window
        if let Some(left_window) = app.get_webview_window("left") {
            left_window.emit("url-changed", url.clone()).ok();
        }
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
            let left_window = tauri::WebviewWindowBuilder::new(
                app,
                "left", // the window label
                tauri::WebviewUrl::App(Default::default())
            )
            .title("Control Panel")
            .inner_size(300.0, 800.0)
            .position(0.0, 0.0)
            .build()
            .expect("failed to build left window");
            
            // Get a handle to the left window for later use
            let left_window_handle = left_window.clone();
            
            // Create a window for the right panel (web content)
            let right_window = tauri::WebviewWindowBuilder::new(
                app,
                "right", // the window label
                tauri::WebviewUrl::External(Url::parse("https://google.com").unwrap())
            )
            .title("Web Content")
            .inner_size(900.0, 800.0)
            .position(300.0, 0.0)
            .on_navigation(move |url| {
                // This callback is triggered whenever navigation happens in the right window
                println!("Navigation detected to: {}", url);
                
                // Emit the URL change event to the left window
                left_window_handle.emit("url-changed", url.to_string()).ok();
                
                // Return true to allow the navigation to proceed
                true
            })
            .build()
            .expect("failed to build right window");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, navigate_webviews])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
