# Navigation Handling

This document explains how URL tracking and navigation are handled in the Multi-Webview Sync application.

## Overview

In a multi-webview application, tracking URL changes and synchronizing navigation state between windows is essential. The Multi-Webview Sync application uses Tauri v2's native navigation APIs to efficiently track and respond to URL changes.

## Navigation Mechanisms

The application handles navigation through two primary mechanisms:

1. **Native Navigation Tracking**: Using Tauri v2's `on_navigation` handler
2. **Command-Based Navigation**: Using the `navigate_webviews` command

## Native Navigation Tracking

Tauri v2 provides a native way to track navigation events through the `on_navigation` handler. This approach is more reliable and efficient than JavaScript-based methods.

### Implementation

The handler is attached to the right window during window creation:

```rust
.on_navigation(move |url| {
    // This callback is triggered whenever navigation happens in the right window
    println!("Navigation detected to: {}", url);
    
    // Emit the URL change event to the left window
    left_window_handle.emit("url-changed", url.to_string()).ok();
    
    // Return true to allow the navigation to proceed
    true
})
```

### Advantages of Native Tracking

- **Comprehensive Detection**: Catches all navigation events including redirects, form submissions, and JavaScript-initiated navigation
- **Lower Overhead**: No need for JavaScript polling or event listeners
- **More Reliable**: Direct integration with the webview engine
- **Simpler Code**: Eliminates the need for complex JavaScript tracking logic

## Command-Based Navigation

The application also allows the left window to initiate navigation in the right window through the `navigate_webviews` command.

### Implementation

The command is defined in the Rust backend:

```rust
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
```

### Invocation from Frontend

The command is invoked from the left window when a user selects a URL from history or enters a new URL:

```javascript
import { invoke } from '@tauri-apps/api/core';

async function navigateTo(url) {
  try {
    await invoke('navigate_webviews', { url });
  } catch (error) {
    console.error('Navigation failed:', error);
  }
}
```

## Complete Navigation Flow

1. **User Action**: User clicks a link in the right window or selects a URL from history in the left window
2. **Event Triggering**:
   - If clicking in right window: `on_navigation` handler is triggered
   - If selecting from left window: `navigate_webviews` command is executed
3. **URL Update**: Right window navigates to the new URL
4. **Event Emission**: The `url-changed` event is emitted to the left window
5. **UI Update**: Left window updates its history display with the new URL

## URL Validation

The application includes URL validation to ensure proper formatting:

```rust
// Validate URL
if Url::parse(&url).is_err() {
    return Err("Invalid URL format".to_string());
}
```

This prevents navigation to invalid URLs and provides clear error messages.

## History Management

While not shown in the current implementation, this architecture allows for easy addition of browsing history management:

1. **History Storage**: The left window can maintain a list of visited URLs
2. **History Navigation**: Users can select URLs from history to revisit pages
3. **Persistent Storage**: History can be saved locally for persistence between sessions

## Security Considerations

- **URL Sanitization**: URLs are properly sanitized before navigation
- **Cross-Window Communication**: Only essential data is shared between windows
- **Error Handling**: Navigation errors are properly caught and reported

## Future Enhancements

The current navigation system can be extended with:

1. **Navigation Controls**: Add back/forward/refresh functionality
2. **Deep Linking**: Enable launching the app with specific URLs
3. **Tab Management**: Support multiple tabs in the right window
4. **Bookmarking**: Allow users to bookmark favorite URLs 