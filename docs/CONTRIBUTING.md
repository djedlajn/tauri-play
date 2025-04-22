# Contributing to Multi-Webview Sync

Thank you for your interest in contributing to the Multi-Webview Sync project! This document provides guidelines and instructions for contributing.

## Getting Started

### Prerequisites

Ensure you have the following installed:

- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri CLI](https://tauri.app/v2/guides/getting-started/setup/)

### Development Setup

1. Fork the repository and clone your fork:
   ```bash
   git clone https://github.com/your-username/multi-webview-sync.git
   cd multi-webview-sync
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run the development server:
   ```bash
   npm run tauri dev
   ```

## Project Structure

Please familiarize yourself with the [Architecture Documentation](./architecture.md) to understand the project structure.

## Code Style

### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Document public functions and types with rustdoc comments

### Frontend Code

- Use TypeScript where possible
- Follow a consistent naming convention
- Document components and functions
- Keep components small and focused

## Making Changes

### Branching Strategy

- Create a feature branch for your changes:
  ```bash
  git checkout -b feature/your-feature-name
  ```

### Commit Guidelines

- Write clear, concise commit messages
- Use present tense ("Add feature" not "Added feature")
- Reference issues in commit messages where applicable

### Testing

- Test your changes thoroughly
- Ensure existing functionality isn't broken
- Add new tests for new features

## Pull Requests

1. Update your fork to include the latest changes from the upstream repository
2. Push your changes to your fork
3. Create a pull request to the main repository
4. Fill out the pull request template with details about your changes
5. Respond to any feedback from code reviewers

## Documentation

When making changes, please update the relevant documentation:

- Update API documentation for any changed commands
- Add examples for new features
- Update architecture diagrams if necessary

## Using AI Assistance

The project has been configured with AI assistance in mind. Please see [AI Integration Guide](./ai-integration.md) for tips on leveraging AI tools during development.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## Resources

- [Development Guide](./development-guide.md)
- [API Reference](./api-reference.md)
- [Event System Documentation](./event-system.md)
- [Navigation Handling Documentation](./navigation-handling.md)
- [AI Integration Guide](./ai-integration.md)
- [Tauri v2 Documentation](https://tauri.app/v2/) 