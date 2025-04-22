# Multi-Webview Sync Documentation

This documentation provides comprehensive information about the Multi-Webview Sync project built with Tauri v2.

## Overview

Multi-Webview Sync is a Tauri v2 application that demonstrates synchronization between multiple webview windows. It implements a two-panel layout:

- **Left Panel**: Control panel that shows navigation history and provides controls
- **Right Panel**: Web content viewer that displays external web content

The application showcases how to:
- Track and synchronize URL changes between windows
- Implement navigation commands
- Communicate between windows using Tauri's event system
- Handle webview navigation events natively in Rust

## Documentation Contents

- [Architecture Overview](./architecture.md) - Detailed explanation of the application architecture
- [API Reference](./api-reference.md) - Documentation of the commands and events
- [Development Guide](./development-guide.md) - Guide for developers working on the project
- [Event System](./event-system.md) - Explanation of the event communication system
- [Navigation Handling](./navigation-handling.md) - Details on URL tracking and navigation

## Key Features

- Two-panel layout with synchronized state
- Native URL tracking using Tauri v2's `on_navigation` handler
- Cross-window communication using events
- Browsing history tracking
- Clean separation between UI and backend logic

## Getting Started

Refer to the [Development Guide](./development-guide.md) for setting up and running the project. 