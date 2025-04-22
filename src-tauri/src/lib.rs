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
            
            // Create a window for the right panel (web content)
            let right_window = tauri::WebviewWindowBuilder::new(
                app,
                "right", // the window label
                tauri::WebviewUrl::External(Url::parse("https://google.com").unwrap())
            )
            .title("Web Content")
            .inner_size(900.0, 800.0)
            .position(300.0, 0.0)
            .build()
            .expect("failed to build right window");
            
            // Clone the app handle for use in the events
            let app_handle = app.app_handle();
            
            // Add JavaScript to monitor URL changes in the right window
            right_window.eval(r#"
                // Monitor URL changes
                let lastUrl = window.location.href;
                
                // Function to check URL and notify if changed
                function checkUrlChange() {
                    if (lastUrl !== window.location.href) {
                        lastUrl = window.location.href;
                        console.log('URL changed to: ' + lastUrl);
                        
                        // Send message to Rust backend
                        window.__TAURI__.event.emit('internal-url-changed', lastUrl);
                    }
                }
                
                // Set intervals to check URL changes
                setInterval(checkUrlChange, 500);
                
                // Listen for link clicks
                document.addEventListener('click', function(e) {
                    const link = e.target.closest('a');
                    if (link && link.href && !link.href.startsWith('javascript:')) {
                        console.log('Link clicked: ' + link.href);
                    }
                });
                
                // Also listen for history API changes
                const originalPushState = history.pushState;
                history.pushState = function() {
                    originalPushState.apply(this, arguments);
                    checkUrlChange();
                };
                
                const originalReplaceState = history.replaceState;
                history.replaceState = function() {
                    originalReplaceState.apply(this, arguments);
                    checkUrlChange();
                };
                
                // Listen for popstate event (back/forward navigation)
                window.addEventListener('popstate', function() {
                    checkUrlChange();
                });
            "#).expect("Failed to inject URL change monitor");
            
            // Listen for URL change events from the JavaScript
            let left_window_clone = left_window.clone();
            
            // Use the app_handle instead of the app reference to register the event listener
            app_handle.listen("internal-url-changed", move |event| {
                let url = event.payload();
                println!("URL changed event received: {}", url);
                
                // Forward the URL to the left window
                left_window_clone.emit("url-changed", url.to_string()).ok();
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, navigate_webviews])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
