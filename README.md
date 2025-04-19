# Tauri Multi-Webview Sync Application

A Tauri application with two synchronized web views in a vertical split layout. The left panel (1/4 width) contains a control panel that allows navigation, while the right panel (3/4 width) displays web content.

## Features

- **Split View Layout**: 1/4 and 3/4 vertical split layout
- **URL Synchronization**: URL entered in the left panel navigates the right webview
- **Modern UI**: Clean, responsive design with dark mode support
- **Error Handling**: Proper validation and error messaging

## Screenshots

[Screenshots would be added here]

## Tech Stack

- **Frontend**: React with TypeScript
- **Backend**: Tauri 2 with Rust
- **Styling**: CSS3

## Installation and Running

1. Clone the repository
2. Install dependencies:
   ```
   pnpm install
   ```
3. Run the development server:
   ```
   pnpm tauri dev
   ```
4. Build for production:
   ```
   pnpm tauri build
   ```

## Project Structure

```
multi-webview-sync/
├─ src/                  # React frontend
│  ├─ App.tsx            # Main React component
│  ├─ App.css            # Styling
│  └─ main.tsx           # Entry point
├─ src-tauri/            # Tauri/Rust backend
│  ├─ src/
│  │  ├─ main.rs         # Rust entry point
│  │  └─ lib.rs          # Main application logic
│  ├─ Cargo.toml         # Rust dependencies
│  └─ tauri.conf.json    # Tauri configuration
└─ package.json          # JavaScript dependencies
```

## How It Works

1. The Rust backend creates two separate windows:
   - Left window: Control panel (React app)
   - Right window: Web content (initially Google.com)

2. The React app provides a URL input field and navigation button

3. When a URL is submitted, the Rust backend:
   - Validates the URL
   - Uses JavaScript injection to navigate the right window

## Detailed Build Prompt

Below is a detailed prompt that could be used to recreate this application:

```
I would like to create a Tauri multi-webview application with the following specifications:

1. Create a new Tauri project with React and TypeScript.

2. Layout Requirements:
   - The application view should be split vertically in a ratio of 1/4 and 3/4
   - The left panel (1/4 width) should contain a control interface
   - The right panel (3/4 width) should display web content
   - Initially, the right panel should show Google.com

3. Functionality:
   - The left panel should include a URL input field and a navigation button
   - When a URL is entered and submitted, the right panel should navigate to that URL
   - URLs should be validated and automatically have https:// added if missing

4. UI Requirements:
   - Clean, modern interface
   - Proper error handling and loading states
   - Dark mode support
   - Responsive design

5. Implementation Details:
   - Use Tauri v2 with unstable features
   - Create separate windows for the two panels positioned side by side
   - Handle URL navigation by using JavaScript evaluation
   - Provide user feedback for errors and loading states

The application should be simple and focused on this core functionality, with an emphasis on clean code and good error handling.
```

## Configuration Details

### Rust Dependencies (Cargo.toml)

```toml
[dependencies]
tauri = { version = "2", features = ["unstable"] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
url = "2.3"
```

### Tauri Configuration (tauri.conf.json)

The key configuration is to enable the Tauri window APIs via the unstable feature flag.

### Rust Implementation (lib.rs)

The Rust code:
1. Creates two WebviewWindow instances positioned side by side
2. Implements a command to navigate the right window using JavaScript evaluation
3. Handles errors and validates URLs

### React Implementation (App.tsx)

The React component:
1. Manages URL state and loading states
2. Provides form controls with validation
3. Invokes the Rust command to navigate the right window
4. Shows appropriate loading and error feedback

## License

[Your license information]

## Acknowledgements

- [Tauri](https://tauri.app/)
- [React](https://reactjs.org/)
- [TypeScript](https://www.typescriptlang.org/)
