# Event Communication System

This document details the event-based communication system used in the Multi-Webview Sync application.

## Overview

Tauri v2 provides a powerful event system that enables communication:
- Between Rust backend and frontend windows
- Between different frontend windows
- From frontend to backend

The Multi-Webview Sync application leverages this event system to maintain synchronization between the left (control panel) and right (web content) windows.

## Event Flow Diagram

```
┌──────────────────┐                       ┌──────────────────┐
│                  │                       │                  │
│   Left Window    │                       │   Right Window   │
│  (Control Panel) │                       │  (Web Content)   │
│                  │                       │                  │
└────────┬─────────┘                       └────────┬─────────┘
         │                                          │
         │                                          │
         │ 1. navigate_webviews                     │ 2. on_navigation
         │    command                               │    event
         │                                          │
         ▼                                          ▼
┌──────────────────────────────────────────────────────────────┐
│                                                              │
│                     Rust Backend (lib.rs)                    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
         │                                          ▲
         │                                          │
         │ 3. URL change                            │
         │    command execution                     │
         │                                          │
         ▼                                          │
┌──────────────────┐                       ┌──────────────────┐
│                  │                       │                  │
│   Left Window    │                       │   Right Window   │
│  (Control Panel) │◄──────────────────────│  (Web Content)   │
│                  │  4. url-changed event │                  │
└──────────────────┘                       └──────────────────┘
```

## Event Types

### 1. Command-based Communication

Commands are Rust functions annotated with `#[tauri::command]` that can be invoked from JavaScript.

**Example: navigate_webviews Command**

1. **Frontend Invocation (Left Window):**
   ```javascript
   import { invoke } from '@tauri-apps/api/core';
   
   // User clicks on a history item or submits a URL
   async function navigateTo(url) {
     try {
       await invoke('navigate_webviews', { url });
     } catch (error) {
       console.error('Navigation failed:', error);
     }
   }
   ```

2. **Backend Implementation:**
   ```rust
   #[tauri::command]
   fn navigate_webviews(url: String, window: tauri::Window) -> Result<(), String> {
       // Implementation details...
       
       // Navigate right window to URL
       right_window.eval(&js).map_err(|e| e.to_string())?;
       
       // Notify left window
       left_window.emit("url-changed", url.clone()).ok();
       
       Ok(())
   }
   ```

### 2. Native Event-based Communication

Events are used for asynchronous communication between components.

**Example: URL Change Events**

1. **Emitting Events (Backend):**
   ```rust
   // Inside on_navigation handler
   left_window_handle.emit("url-changed", url.to_string()).ok();
   ```

2. **Listening for Events (Frontend):**
   ```javascript
   import { listen } from '@tauri-apps/api/event';
   
   // Set up event listener when the component mounts
   async function setupEventListeners() {
     const unlisten = await listen('url-changed', (event) => {
       const newUrl = event.payload;
       // Update UI with the new URL
       addToHistory(newUrl);
     });
     
     // Save the unlisten function to clean up later
     return unlisten;
   }
   
   // Clean up when component unmounts
   function componentWillUnmount(unlisten) {
     if (unlisten) {
       unlisten();
     }
   }
   ```

## Native Navigation Tracking

Tauri v2 provides a native way to track navigation through the `on_navigation` handler:

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

This handler is triggered for all navigation events in the webview, including:
- Link clicks
- Form submissions
- JavaScript-initiated navigation
- History API usage
- Redirects

## Best Practices

1. **Error Handling**
   - Always use `.ok()` or proper error handling when emitting events
   - Handle potential failures in event listeners

2. **Event Naming**
   - Use descriptive, hyphenated names for events (e.g., `url-changed`)
   - Be consistent with naming conventions

3. **Cleanup**
   - Always save and call the `unlisten` function to prevent memory leaks
   - Unregister event listeners when components unmount

4. **Payload Design**
   - Keep payloads simple and serializable
   - Use structured data for complex information

5. **Testing**
   - Mock events for testing frontend components
   - Verify event emission in backend tests 