# API Reference

This document provides a comprehensive reference for the commands and events used in the Multi-Webview Sync application.

## Commands

Commands are functions defined in the Rust backend that can be invoked from the frontend.

### `greet`

A simple greeting command for demonstration purposes.

**Signature:**
```rust
#[tauri::command]
fn greet(name: &str) -> String
```

**Parameters:**
- `name` (string): The name to include in the greeting

**Returns:**
- A greeting message string

**Example:**
```javascript
import { invoke } from '@tauri-apps/api/core';
await invoke('greet', { name: 'John' });
// Returns: "Hello, John! You've been greeted from Rust!"
```

### `navigate_webviews`

Navigates the right webview to a specified URL and notifies the left webview about the navigation.

**Signature:**
```rust
#[tauri::command]
fn navigate_webviews(url: String, window: tauri::Window) -> Result<(), String>
```

**Parameters:**
- `url` (string): The URL to navigate to
- `window` (tauri::Window): The window from which the command was invoked (automatically provided by Tauri)

**Returns:**
- `Ok(())` if navigation was successful
- `Err(String)` with an error message if navigation failed

**Example:**
```javascript
import { invoke } from '@tauri-apps/api/core';
try {
  await invoke('navigate_webviews', { url: 'https://tauri.app' });
  console.log('Navigation successful');
} catch (error) {
  console.error('Navigation failed:', error);
}
```

## Events

Events are used for communication between the backend and frontend, as well as between different windows.

### `url-changed`

Emitted when the URL in the right window changes. The left window listens for this event to update its history display.

**Direction:** Backend → Left Window

**Payload:** String containing the new URL

**Example (Listening in Frontend):**
```javascript
import { listen } from '@tauri-apps/api/event';

await listen('url-changed', (event) => {
  console.log('URL changed to:', event.payload);
  // Update history UI with the new URL
  updateHistoryList(event.payload);
});
```

**Example (Emitting in Backend):**
```rust
// Inside the on_navigation handler
left_window_handle.emit("url-changed", url.to_string()).ok();
```

## WebviewWindow Builder

The application creates two windows with specific configurations:

### Left Window (Control Panel)

```rust
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
```

### Right Window (Web Content)

```rust
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
```

## Navigation Handlers

### `on_navigation`

The `on_navigation` handler is attached to the right window to track URL changes:

**Signature:**
```rust
.on_navigation(|url| -> bool)
```

**Parameters:**
- `url` (String): The URL being navigated to

**Returns:**
- `true` to allow the navigation
- `false` to block the navigation

This handler is used to detect all navigation events in the right window and notify the left window by emitting the `url-changed` event. 