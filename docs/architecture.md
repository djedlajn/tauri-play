# Application Architecture

This document outlines the architecture of the Multi-Webview Sync application built with Tauri v2.

## Overview

The application follows a multi-window architecture with a clear separation between frontend components and Rust backend. It consists of two main windows that communicate with each other through the Tauri event system.

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Application                    │
│                                                         │
│  ┌───────────────┐                 ┌───────────────┐    │
│  │               │                 │               │    │
│  │  Left Window  │◄───Events────►  │  Right Window │    │
│  │ (Control Panel)│                 │ (Web Content) │    │
│  │               │                 │               │    │
│  └───────────────┘                 └───────────────┘    │
│          ▲                                 ▲            │
│          │                                 │            │
│          ▼                                 ▼            │
│  ┌─────────────────────────────────────────────────┐    │
│  │                                                 │    │
│  │           Rust Backend (lib.rs)                 │    │
│  │                                                 │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## Component Breakdown

### 1. Rust Backend (lib.rs)

The core of the application resides in `src-tauri/src/lib.rs`. It handles:

- Window creation and configuration
- Event registration and handling
- URL navigation commands
- Communication between windows

Key components:
- **Window Management**: Creating and configuring the left and right windows
- **Navigation Event Handling**: Using `on_navigation` to track URL changes
- **Command Handling**: Implementing Tauri commands like `navigate_webviews`

### 2. Left Window (Control Panel)

The left window serves as the control panel and history tracker. It:

- Displays browsing history
- Provides navigation controls
- Listens for `url-changed` events from the backend

### 3. Right Window (Web Content)

The right window displays external web content. It:

- Renders web pages
- Triggers navigation events when URLs change
- Receives navigation commands from the left window via the backend

## Communication Flow

### URL Change Handling

1. User navigates to a new URL in the right window
2. The `on_navigation` handler in Rust is triggered
3. The handler emits a `url-changed` event to the left window
4. The left window updates its history display

```
Right Window ──(Navigation)──► on_navigation handler ──(url-changed event)──► Left Window
```

### Navigation Command Flow

1. User clicks a history item or enters a URL in the left window
2. The left window calls the `navigate_webviews` command
3. The Rust backend processes the command
4. The right window navigates to the specified URL

```
Left Window ──(navigate_webviews)──► Rust Backend ──(location change)──► Right Window
```

## Technical Design Decisions

### Native Navigation Tracking

The application uses Tauri v2's native `on_navigation` handler instead of JavaScript polling. This provides:

- Better reliability
- Lower overhead
- Direct access to all navigation events
- Cleaner code implementation

### Event-Based Communication

Windows communicate through Tauri's event system, which:

- Decouples components
- Allows asynchronous communication
- Provides a consistent pattern for state updates
- Enables easy extension to more windows if needed 