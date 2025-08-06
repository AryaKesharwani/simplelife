# Storage Manager

A comprehensive desktop application built with Tauri, Svelte, and Rust for managing files, notes, invoices, portfolios, code snippets, and more.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## 🚀 Overview

Storage Manager is a powerful desktop application that adapts to different user types and provides specialized tools for personal use, freelancing, trading, and development. Built with modern web technologies and native desktop capabilities, it offers a seamless experience for managing various aspects of digital life and work.

## ✨ Features

### 🎯 User Type-Specific Tools

#### Personal User
- **Notes Management** - Create, organize, and search personal notes
- **Reminders** - Set and track important reminders
- **File Manager** - Upload and manage files in cloud storage
- **Calculator** - Simple calculator for quick calculations

#### Freelancer
- **Invoice Management** - Create professional invoices with line items and tax calculations
- **Time Tracker** - Track billable hours and project time
- **Client Manager** - Organize client information and contacts
- **Expense Tracker** - Monitor business expenses and costs

#### Trader
- **Portfolio Tracking** - Monitor investment positions and performance
- **Watchlist** - Track stocks and assets of interest
- **Trade Journal** - Log trading activities and decisions
- **Market News** - Stay updated with market information

#### Developer
- **Code Snippets** - Store and organize code examples with syntax highlighting
- **Command Templates** - Save frequently used commands and scripts
- **API Tester** - Test REST APIs and endpoints
- **Database Browser** - Browse and query databases

### 🔧 Core Features

- **Cross-Platform** - Works on Windows, macOS, and Linux
- **Dark/Light Theme** - Automatic theme switching with persistence
- **Local Database** - SQLite database for data persistence
- **Cloud Integration** - S3 upload capabilities for file storage
- **Responsive Design** - Adapts to different screen sizes
- **Keyboard Navigation** - Full keyboard accessibility
- **Search & Filter** - Advanced search and filtering capabilities

## 🏗️ Architecture

### Technology Stack

**Frontend:**
- Svelte 5.0.0 - Modern reactive framework
- SvelteKit 2.9.0 - Full-stack web framework
- Tailwind CSS 4.1.11 - Utility-first CSS framework
- TypeScript - Type-safe JavaScript

**Backend:**
- Rust - Systems programming language
- Tauri 2.x - Desktop application framework
- SQLite - Local database
- Serde - Serialization/deserialization
- Tokio - Async runtime

**Key Dependencies:**
- `@tauri-apps/api` - Tauri API bindings
- `rusqlite` - SQLite database driver
- `rfd` - Native file dialogs
- `chrono` - Date and time handling

### Project Structure

```
storage-manager/
├── src/                    # Frontend source code
│   ├── lib/               # Shared libraries
│   │   ├── components/    # Reusable components
│   │   └── stores/        # State management
│   ├── routes/            # Page components
│   └── app.css           # Global styles
├── src-tauri/             # Backend source code
│   ├── src/              # Rust source files
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── static/               # Static assets
└── docs/                # Documentation
```

## 📦 Installation

### Prerequisites

- Node.js 18+ and npm
- Rust 1.70+ and Cargo
- Platform-specific build tools

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/storage-manager.git
   cd storage-manager
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

4. **Build for production**
   ```bash
   npm run tauri build
   ```

### Development Setup

1. **Frontend Development**
   ```bash
   npm run dev          # Start Svelte dev server
   npm run build        # Build for production
   npm run preview      # Preview production build
   ```

2. **Backend Development**
   ```bash
   cd src-tauri
   cargo build          # Build Rust backend
   cargo test           # Run tests
   ```

3. **Type Checking**
   ```bash
   npm run check        # TypeScript checking
   npm run check:watch  # Watch mode
   ```

## 🎮 Usage

### Getting Started

1. **Launch the Application**
   - Run `npm run tauri dev` for development
   - Or launch the built executable

2. **Select Your User Type**
   - Choose from Personal, Freelancer, Trader, or Developer
   - The interface adapts to show relevant tools

3. **Configure Settings**
   - Set up S3 credentials for file uploads
   - Choose your preferred theme
   - Configure auto-upload settings

4. **Start Using Tools**
   - Navigate to different tools using the dashboard
   - Create, edit, and manage your data
   - Use search and filtering to find items quickly

### Key Features

#### Notes Management
- Create notes with rich text content
- Organize by categories (Work, Personal, Ideas, etc.)
- Search across all notes
- Export notes to various formats

#### Invoice Creation
- Professional invoice templates
- Line item management
- Automatic tax calculations
- Client information tracking
- Status tracking (Draft, Sent, Paid)

#### Portfolio Tracking
- Add investment positions
- Track current prices and performance
- Calculate gains/losses
- Add notes to positions
- Portfolio summary and analytics

#### Code Snippets
- Syntax highlighting for multiple languages
- Tag-based organization
- Search and filter capabilities
- Copy to clipboard functionality
- Version history tracking

## 🔧 Development

### Code Style

- **Frontend**: Follow Svelte 5 conventions and TypeScript best practices
- **Backend**: Follow Rust conventions and use `cargo fmt` for formatting
- **Database**: Use prepared statements and proper error handling

### Testing

```bash
# Frontend tests
npm run test

# Backend tests
cd src-tauri && cargo test

# Integration tests
npm run test:integration
```

### Building

```bash
# Development build
npm run tauri dev

# Production build
npm run tauri build

# Platform-specific builds
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### Debugging

- Use browser dev tools for frontend debugging
- Use `console.log` and `eprintln!` for logging
- Enable Tauri dev tools with `--devtools` flag

## 📚 Documentation

### Comprehensive Documentation

- **[API Documentation](API_DOCUMENTATION.md)** - Complete backend API reference
- **[Component Documentation](COMPONENT_DOCUMENTATION.md)** - Frontend components and pages
- **[Database Schema](DATABASE_SCHEMA.md)** - Database structure and relationships

### Quick References

#### Backend APIs
```rust
// Get all notes
let notes = get_notes()?;

// Save a note
let note_id = save_note(note)?;

// Get settings
let settings = get_settings()?;
```

#### Frontend Integration
```typescript
import { invoke } from '@tauri-apps/api/core';

// Call backend functions
const notes = await invoke('get_notes');
const noteId = await invoke('save_note', { note });
```

#### Theme Management
```typescript
import { theme } from '$lib/stores/theme';

// Toggle theme
theme.set('dark');
theme.set('light');
```

## 🤝 Contributing

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
4. **Add tests for new functionality**
5. **Run the test suite**
   ```bash
   npm run test
   cargo test
   ```
6. **Commit your changes**
   ```bash
   git commit -m 'Add amazing feature'
   ```
7. **Push to the branch**
   ```bash
   git push origin feature/amazing-feature
   ```
8. **Open a Pull Request**

### Contribution Guidelines

- Follow the existing code style and conventions
- Add comprehensive tests for new features
- Update documentation for API changes
- Ensure all tests pass before submitting
- Include clear commit messages

### Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow the project's coding standards

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - Desktop application framework
- [Svelte](https://svelte.dev/) - Frontend framework
- [Tailwind CSS](https://tailwindcss.com/) - CSS framework
- [SQLite](https://www.sqlite.org/) - Database engine

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/your-username/storage-manager/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/storage-manager/discussions)
- **Documentation**: [Wiki](https://github.com/your-username/storage-manager/wiki)

---

**Storage Manager** - Your all-in-one desktop productivity suite.
