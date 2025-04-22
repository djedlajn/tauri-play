# AI Integration Guide

This document provides guidance on using AI tools effectively with the Multi-Webview Sync project, with a focus on Cursor IDE integration.

## Overview

The Multi-Webview Sync project is set up with custom AI integration rules to enhance development efficiency. These configurations enable AI assistants to understand the project structure and provide context-aware recommendations.

## Cursor Configuration

The project includes custom Cursor IDE settings in `.cursor/settings.json` that configure AI behavior for code assistance.

### Project Context

The configuration provides the AI with the following context:

```json
"config": {
  "projectTitle": "Tauri v2 Multi-Webview Sync",
  "projectDescription": "A Tauri v2 application demonstrating multi-webview communication and navigation synchronization",
  "aiContext": {
    "includeDocs": true,
    "docsWeight": 0.8,
    "includeComments": true,
    "preferredDocuments": [
      "docs/README.md",
      "docs/architecture.md",
      "docs/api-reference.md",
      "src-tauri/src/lib.rs"
    ],
    "ignoredDirectories": [
      "node_modules",
      "target",
      "dist",
      ".git"
    ]
  }
}
```

This configuration ensures that:
- Documentation files are given high priority in the AI context
- Key files are preferred for context building
- Large or irrelevant directories are excluded from AI indexing

### Custom Rules

The project defines specific rules to guide AI assistance:

#### 1. Tauri v2 API Usage

```json
{
  "name": "Tauri v2 API Usage",
  "description": "Ensures Tauri v2 APIs are used correctly",
  "pattern": {
    "includes": ["src-tauri/src/**/*.rs"],
    "excludes": []
  },
  "tags": ["tauri", "best-practices"],
  "aiPrompt": "Make sure all Tauri v2 APIs are used correctly. Focus on proper event handling with Listener trait, use of the manager trait, and the WebviewWindowBuilder with the on_navigation handler."
}
```

This rule helps maintain correct usage of Tauri v2 APIs, with special focus on:
- Event handling via the Listener trait
- Manager trait for app and window management
- WebviewWindowBuilder with the on_navigation handler

#### 2. Event Communication

```json
{
  "name": "Event Communication",
  "description": "Ensures proper event communication between windows",
  "pattern": {
    "includes": ["src-tauri/src/**/*.rs", "src/**/*.{ts,tsx,js,jsx}"],
    "excludes": []
  },
  "tags": ["events", "communication"],
  "aiPrompt": "Check that events are properly emitted and listened for both in Rust backend and in frontend code. Ensure event names match between emitters and listeners."
}
```

This rule verifies proper event communication, ensuring:
- Events are correctly emitted and listened for
- Event names are consistent between backend and frontend
- Proper error handling for events

#### 3. Navigation Handler

```json
{
  "name": "Navigation Handler",
  "description": "Ensures navigation is handled properly and securely",
  "pattern": {
    "includes": ["src-tauri/src/**/*.rs"],
    "excludes": []
  },
  "tags": ["navigation", "security"],
  "aiPrompt": "Verify that navigation is handled using Tauri v2's on_navigation handler and that URLs are properly validated and sanitized before navigation."
}
```

This rule focuses on secure navigation handling:
- Using the native on_navigation handler
- Proper URL validation and sanitization
- Safe navigation between windows

## AI-Assisted Development Workflows

### Getting Context-Aware Code Suggestions

When working in Cursor, you can leverage the AI context by:

1. **Project-Specific Completions**: The AI will consider the project structure and Tauri v2 best practices
2. **Documentation-Aware Suggestions**: Get suggestions informed by the project's documentation
3. **Rule-Based Guidance**: Receive hints aligned with the project's custom rules

### Using AI for Code Reviews

The AI can help with code reviews by checking:

1. Proper event communication patterns
2. Correct usage of Tauri v2 APIs
3. Secure navigation handling
4. Consistency with project architecture

### Extending AI Integration

To extend the AI integration:

1. **Add New Rules**: Extend `.cursor/settings.json` with additional rules
2. **Update Documentation**: Keep documentation updated to improve context quality
3. **Add Comments**: Strategic comments will be indexed by the AI

## Best Practices

1. **Use Tagged Questions**: When asking the AI for help, use tags like #tauri, #events, or #navigation to trigger specific rules
2. **Reference Documentation**: Mention specific documentation files to ensure they're included in the context
3. **Update AI Context**: When making significant architectural changes, update the AI configuration
4. **Ignore Large Files**: Add large generated files to `.cursorignore` to optimize AI performance 