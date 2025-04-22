# Development Guide

This guide provides instructions for setting up and working on the Multi-Webview Sync project.

## Prerequisites

Ensure you have the following installed:

- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri CLI](https://tauri.app/v2/guides/getting-started/setup/)

## Project Setup

1. Clone the repository
   ```bash
   git clone <repository-url>
   cd multi-webview-sync
   ```

2. Install frontend dependencies
   ```bash
   npm install
   ```

3. Run the development server
   ```bash
   npm run tauri dev
   ```

## Project Structure

```
multi-webview-sync/
├── docs/                   # Documentation files
├── src/                    # Frontend source code
│   ├── components/         # UI components
│   ├── App.tsx             # Main App component
│   └── main.tsx            # Frontend entry point
├── src-tauri/              # Tauri backend code
│   ├── src/                # Rust source code
│   │   └── lib.rs          # Main Rust backend code
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── public/                 # Static assets
└── package.json            # NPM dependencies
```

## Backend Development (Rust)

The Rust backend code is located in `src-tauri/src/lib.rs`. It contains:

1. **Command definitions** - Tauri commands that can be invoked from the frontend
2. **Window setup** - Configuration for the left and right windows
3. **Event handlers** - Handlers for navigation and other events

### Adding a New Command

To add a new command:

1. Define the command in `lib.rs`:
   ```rust
   #[tauri::command]
   fn my_new_command(param: String) -> Result<String, String> {
       // Command implementation
       Ok(format!("Processed: {}", param))
   }
   ```

2. Register the command in the `invoke_handler`:
   ```rust
   .invoke_handler(tauri::generate_handler![
       greet, 
       navigate_webviews,
       my_new_command // Add your new command here
   ])
   ```

### Working with Window Events

To add a new event:

1. Emit events from backend to frontend:
   ```rust
   window.emit("event-name", payload).ok();
   ```

2. Listen for events in the frontend (see Frontend Development section)

## Frontend Development

The frontend code is a standard web application that communicates with the Tauri backend.

### Invoking Commands

To call a backend command from the frontend:

```typescript
import { invoke } from '@tauri-apps/api/core';

// Call a command
try {
  const result = await invoke('my_command', { param: 'value' });
  console.log('Command result:', result);
} catch (error) {
  console.error('Command failed:', error);
}
```

### Listening for Events

To listen for events from the backend:

```typescript
import { listen } from '@tauri-apps/api/event';

// Listen for an event
const unlisten = await listen('event-name', (event) => {
  console.log('Received event:', event.payload);
  // Process the event
});

// Later, to stop listening
unlisten();
```

## Building for Production

To build the application for production:

```bash
npm run tauri build
```

This will create distributable packages in `src-tauri/target/release/bundle/`.

## Testing

### Unit Tests

Run Rust unit tests:

```bash
cd src-tauri
cargo test
```

### Integration Tests

To add integration tests for the combined frontend and backend:

1. Create test files in a `tests/` directory
2. Use Tauri's testing utilities to test cross-process functionality

## Troubleshooting

### Common Issues

1. **Window not opening**
   - Check for errors in the terminal output
   - Verify the window configuration in `lib.rs`

2. **Events not being received**
   - Ensure the event name matches exactly
   - Check that the listener is set up before the event is emitted

3. **Command not found**
   - Verify the command is registered in the `invoke_handler`
   - Check for typos in the command name

### Debugging

- Use `println!` statements in Rust code to log to the terminal
- Check the browser console in development mode for frontend errors
- Enable Tauri's debug features in `tauri.conf.json` 