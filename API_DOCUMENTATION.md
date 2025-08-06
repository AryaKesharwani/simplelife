# Storage Manager API Documentation

## Overview

Storage Manager is a Tauri-based desktop application that provides a comprehensive suite of tools for file management, note-taking, invoicing, portfolio tracking, code snippet management, and command template storage. The application consists of a Rust backend with SQLite database and a Svelte frontend.

## Table of Contents

1. [Backend APIs (Rust/Tauri)](#backend-apis-rusttauri)
2. [Frontend Components (Svelte)](#frontend-components-svelte)
3. [Frontend Stores (Svelte)](#frontend-stores-svelte)
4. [Data Structures](#data-structures)
5. [Usage Examples](#usage-examples)

---

## Backend APIs (Rust/Tauri)

### Core Functions

#### `greet(name: &str) -> String`
A simple greeting function for testing purposes.

**Parameters:**
- `name: &str` - The name to greet

**Returns:**
- `String` - A greeting message

**Example:**
```rust
let result = greet("John");
// Returns: "Hello, John! You've been greeted from Rust!"
```

#### `open_file_dialog() -> Result<Vec<String>, String>`
Opens a native file dialog for selecting multiple files.

**Returns:**
- `Result<Vec<String>, String>` - Vector of selected file paths or error message

**Supported file types:**
- All Files (*)
- Text Files (txt, md, json, csv)
- Images (jpg, jpeg, png, gif, bmp, svg)
- Documents (pdf, doc, docx, xls, xlsx, ppt, pptx)
- Videos (mp4, avi, mov, wmv, flv, mkv)
- Audio (mp3, wav, flac, aac, ogg)

**Example:**
```rust
let files = open_file_dialog().await?;
// Returns: ["/path/to/file1.txt", "/path/to/file2.pdf"]
```

#### `upload_to_s3(request: UploadRequest) -> Result<String, String>`
Uploads a file to Amazon S3.

**Parameters:**
- `request: UploadRequest` - Upload configuration and file details

**Returns:**
- `Result<String, String>` - Success message or error

**Example:**
```rust
let request = UploadRequest {
    file_path: "/path/to/file.txt".to_string(),
    file_name: "file.txt".to_string(),
    bucket: "my-bucket".to_string(),
    region: "us-east-1".to_string(),
    access_key_id: "AKIA...".to_string(),
    secret_access_key: "secret...".to_string(),
};
let result = upload_to_s3(request).await?;
```

### Settings Management

#### `get_settings() -> Result<Settings, String>`
Retrieves application settings from the database.

**Returns:**
- `Result<Settings, String>` - Settings object or error

**Example:**
```rust
let settings = get_settings()?;
println!("Theme: {}", settings.theme);
```

#### `save_settings(settings: Settings) -> Result<(), String>`
Saves application settings to the database.

**Parameters:**
- `settings: Settings` - Settings object to save

**Returns:**
- `Result<(), String>` - Success or error

**Example:**
```rust
let settings = Settings {
    s3_config: S3Config { /* ... */ },
    theme: "dark".to_string(),
    auto_upload: true,
    user_type: "premium".to_string(),
};
save_settings(settings)?;
```

### Notes Management

#### `get_notes() -> Result<Vec<Note>, String>`
Retrieves all notes from the database, ordered by last updated.

**Returns:**
- `Result<Vec<Note>, String>` - Vector of notes or error

**Example:**
```rust
let notes = get_notes()?;
for note in notes {
    println!("Title: {}", note.title);
}
```

#### `save_note(note: Note) -> Result<i32, String>`
Saves a note to the database. Creates new note if ID is None, updates existing if ID is Some.

**Parameters:**
- `note: Note` - Note object to save

**Returns:**
- `Result<i32, String>` - Note ID or error

**Example:**
```rust
let note = Note {
    id: None, // New note
    title: "My Note".to_string(),
    content: "Note content".to_string(),
    category: "work".to_string(),
    created_at: "2024-01-01T00:00:00Z".to_string(),
    updated_at: "2024-01-01T00:00:00Z".to_string(),
};
let note_id = save_note(note)?;
```

#### `delete_note(id: i32) -> Result<(), String>`
Deletes a note by ID.

**Parameters:**
- `id: i32` - Note ID to delete

**Returns:**
- `Result<(), String>` - Success or error

**Example:**
```rust
delete_note(123)?;
```

### Invoice Management

#### `get_invoices() -> Result<Vec<Invoice>, String>`
Retrieves all invoices from the database, ordered by creation date.

**Returns:**
- `Result<Vec<Invoice>, String>` - Vector of invoices or error

**Example:**
```rust
let invoices = get_invoices()?;
for invoice in invoices {
    println!("Invoice #{}: {}", invoice.invoice_number, invoice.client_name);
}
```

#### `save_invoice(invoice: Invoice) -> Result<i32, String>`
Saves an invoice to the database. Creates new invoice if ID is None, updates existing if ID is Some.

**Parameters:**
- `invoice: Invoice` - Invoice object to save

**Returns:**
- `Result<i32, String>` - Invoice ID or error

**Example:**
```rust
let invoice = Invoice {
    id: None,
    invoice_number: "INV-001".to_string(),
    client_name: "John Doe".to_string(),
    client_email: "john@example.com".to_string(),
    items: vec![InvoiceItem {
        description: "Web Development".to_string(),
        quantity: 10.0,
        unit_price: 100.0,
        total: 1000.0,
    }],
    subtotal: 1000.0,
    tax_rate: 0.1,
    tax_amount: 100.0,
    total: 1100.0,
    status: "draft".to_string(),
    due_date: "2024-02-01".to_string(),
    created_at: "2024-01-01T00:00:00Z".to_string(),
};
let invoice_id = save_invoice(invoice)?;
```

### Portfolio Management

#### `get_portfolio() -> Result<Vec<PortfolioItem>, String>`
Retrieves all portfolio items from the database, ordered by symbol.

**Returns:**
- `Result<Vec<PortfolioItem>, String>` - Vector of portfolio items or error

**Example:**
```rust
let portfolio = get_portfolio()?;
for item in portfolio {
    println!("{}: {} shares at ${}", item.symbol, item.shares, item.avg_price);
}
```

#### `save_portfolio_item(item: PortfolioItem) -> Result<i32, String>`
Saves a portfolio item to the database. Creates new item if ID is None, updates existing if ID is Some.

**Parameters:**
- `item: PortfolioItem` - Portfolio item to save

**Returns:**
- `Result<i32, String>` - Item ID or error

**Example:**
```rust
let item = PortfolioItem {
    id: None,
    symbol: "AAPL".to_string(),
    shares: 10.0,
    avg_price: 150.0,
    current_price: 160.0,
    total_value: 1600.0,
    gain_loss: 100.0,
    gain_loss_percent: 6.67,
    notes: "Long-term investment".to_string(),
};
let item_id = save_portfolio_item(item)?;
```

### Code Snippet Management

#### `get_code_snippets() -> Result<Vec<CodeSnippet>, String>`
Retrieves all code snippets from the database, ordered by last updated.

**Returns:**
- `Result<Vec<CodeSnippet>, String>` - Vector of code snippets or error

**Example:**
```rust
let snippets = get_code_snippets()?;
for snippet in snippets {
    println!("{} ({})", snippet.title, snippet.language);
}
```

#### `save_code_snippet(snippet: CodeSnippet) -> Result<i32, String>`
Saves a code snippet to the database. Creates new snippet if ID is None, updates existing if ID is Some.

**Parameters:**
- `snippet: CodeSnippet` - Code snippet to save

**Returns:**
- `Result<i32, String>` - Snippet ID or error

**Example:**
```rust
let snippet = CodeSnippet {
    id: None,
    title: "Hello World".to_string(),
    description: "Simple hello world example".to_string(),
    code: "console.log('Hello, World!');".to_string(),
    language: "javascript".to_string(),
    tags: "example,hello".to_string(),
    created_at: "2024-01-01T00:00:00Z".to_string(),
    updated_at: "2024-01-01T00:00:00Z".to_string(),
};
let snippet_id = save_code_snippet(snippet)?;
```

### Command Template Management

#### `get_command_templates() -> Result<Vec<CommandTemplate>, String>`
Retrieves all command templates from the database, ordered by creation date.

**Returns:**
- `Result<Vec<CommandTemplate>, String>` - Vector of command templates or error

**Example:**
```rust
let templates = get_command_templates()?;
for template in templates {
    println!("{}: {}", template.title, template.command);
}
```

#### `save_command_template(template: CommandTemplate) -> Result<i32, String>`
Saves a command template to the database. Creates new template if ID is None, updates existing if ID is Some.

**Parameters:**
- `template: CommandTemplate` - Command template to save

**Returns:**
- `Result<i32, String>` - Template ID or error

**Example:**
```rust
let template = CommandTemplate {
    id: None,
    title: "Git Status".to_string(),
    description: "Check git status".to_string(),
    command: "git status".to_string(),
    category: "git".to_string(),
    tags: "git,status".to_string(),
    created_at: "2024-01-01T00:00:00Z".to_string(),
};
let template_id = save_command_template(template)?;
```

---

## Frontend Components (Svelte)

### ThemeSwitcher Component

**File:** `src/lib/components/ThemeSwitcher.svelte`

A reusable component for toggling between light and dark themes.

**Props:**
- None (uses global theme store)

**Events:**
- None (internal state management)

**Features:**
- Smooth animations for icon transitions
- Hover effects with gradient overlay
- Accessible with ARIA labels
- Responsive design

**Usage:**
```svelte
<script>
  import ThemeSwitcher from '$lib/components/ThemeSwitcher.svelte';
</script>

<ThemeSwitcher />
```

**Styling:**
- Uses Tailwind CSS classes
- Dark mode support with `dark:` prefixes
- Transition animations for smooth theme switching
- Hover states with background color changes

---

## Frontend Stores (Svelte)

### Theme Store

**File:** `src/lib/stores/theme.ts`

A Svelte writable store for managing application theme state.

**Exports:**
- `theme: WritableStore<string>` - The theme store
- `initializeTheme(): void` - Function to initialize theme on app startup

**Features:**
- Persists theme preference in localStorage
- Automatically applies theme to document root
- Browser-safe with environment checks
- Console logging for debugging

**Usage:**
```typescript
import { theme, initializeTheme } from '$lib/stores/theme';

// Subscribe to theme changes
theme.subscribe(value => {
  console.log('Theme changed to:', value);
});

// Set theme
theme.set('dark');

// Initialize theme on app startup
initializeTheme();
```

**API:**
- `theme.set(value: string)` - Set theme to 'light' or 'dark'
- `theme.subscribe(callback)` - Subscribe to theme changes
- `initializeTheme()` - Initialize theme from localStorage

---

## Data Structures

### Backend Structures (Rust)

#### `S3Config`
```rust
struct S3Config {
    bucket: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
}
```

#### `UploadRequest`
```rust
struct UploadRequest {
    file_path: String,
    file_name: String,
    bucket: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
}
```

#### `Settings`
```rust
struct Settings {
    s3_config: S3Config,
    theme: String,
    auto_upload: bool,
    user_type: String,
}
```

#### `Note`
```rust
struct Note {
    id: Option<i32>,
    title: String,
    content: String,
    category: String,
    created_at: String,
    updated_at: String,
}
```

#### `Invoice`
```rust
struct Invoice {
    id: Option<i32>,
    invoice_number: String,
    client_name: String,
    client_email: String,
    items: Vec<InvoiceItem>,
    subtotal: f64,
    tax_rate: f64,
    tax_amount: f64,
    total: f64,
    status: String,
    due_date: String,
    created_at: String,
}
```

#### `InvoiceItem`
```rust
struct InvoiceItem {
    description: String,
    quantity: f64,
    unit_price: f64,
    total: f64,
}
```

#### `PortfolioItem`
```rust
struct PortfolioItem {
    id: Option<i32>,
    symbol: String,
    shares: f64,
    avg_price: f64,
    current_price: f64,
    total_value: f64,
    gain_loss: f64,
    gain_loss_percent: f64,
    notes: String,
}
```

#### `CodeSnippet`
```rust
struct CodeSnippet {
    id: Option<i32>,
    title: String,
    description: String,
    code: String,
    language: String,
    tags: String,
    created_at: String,
    updated_at: String,
}
```

#### `CommandTemplate`
```rust
struct CommandTemplate {
    id: Option<i32>,
    title: String,
    description: String,
    command: String,
    category: String,
    tags: String,
    created_at: String,
}
```

---

## Usage Examples

### Frontend Integration

#### Using Tauri Commands in Svelte
```typescript
import { invoke } from '@tauri-apps/api/core';

// Get all notes
const notes = await invoke('get_notes');

// Save a new note
const noteId = await invoke('save_note', {
  note: {
    id: null,
    title: 'My Note',
    content: 'Note content',
    category: 'work',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString()
  }
});

// Delete a note
await invoke('delete_note', { id: 123 });

// Get settings
const settings = await invoke('get_settings');

// Save settings
await invoke('save_settings', {
  settings: {
    s3_config: {
      bucket: 'my-bucket',
      region: 'us-east-1',
      access_key_id: 'AKIA...',
      secret_access_key: 'secret...'
    },
    theme: 'dark',
    auto_upload: true,
    user_type: 'premium'
  }
});
```

#### Theme Management
```typescript
import { theme } from '$lib/stores/theme';
import { initializeTheme } from '$lib/stores/theme';

// Initialize theme on app startup
initializeTheme();

// Subscribe to theme changes
theme.subscribe(value => {
  console.log('Theme is now:', value);
});

// Toggle theme
function toggleTheme() {
  theme.update(current => current === 'light' ? 'dark' : 'light');
}
```

### Backend Integration

#### Database Operations
```rust
// Initialize database (called automatically on startup)
init_db()?;

// Get all notes
let notes = get_notes()?;

// Save a note
let note = Note {
    id: None,
    title: "New Note".to_string(),
    content: "Content here".to_string(),
    category: "general".to_string(),
    created_at: chrono::Utc::now().to_rfc3339(),
    updated_at: chrono::Utc::now().to_rfc3339(),
};
let note_id = save_note(note)?;

// Delete a note
delete_note(note_id)?;
```

#### File Operations
```rust
// Open file dialog
let selected_files = open_file_dialog().await?;

// Upload to S3
let request = UploadRequest {
    file_path: "/path/to/file.txt".to_string(),
    file_name: "file.txt".to_string(),
    bucket: "my-bucket".to_string(),
    region: "us-east-1".to_string(),
    access_key_id: "AKIA...".to_string(),
    secret_access_key: "secret...".to_string(),
};
let result = upload_to_s3(request).await?;
```

---

## Error Handling

All Tauri commands return `Result<T, String>` where:
- `Ok(T)` - Successful operation with return value
- `Err(String)` - Error message describing what went wrong

Common error scenarios:
- Database connection issues
- File not found errors
- Invalid data format
- Permission denied errors
- Network connectivity issues (for S3 uploads)

---

## Security Considerations

1. **API Keys**: S3 credentials are stored in the local database. Consider encryption for production use.
2. **File Access**: The application can access files through the file dialog. Users should be cautious about which files they select.
3. **Data Validation**: All input data should be validated before processing.
4. **Error Messages**: Error messages should not expose sensitive information.

---

## Performance Considerations

1. **Database Connections**: Uses a single SQLite connection with mutex protection.
2. **Async Operations**: File operations and S3 uploads are asynchronous.
3. **Memory Management**: Large files are read into memory for S3 uploads.
4. **UI Responsiveness**: Theme switching and store updates are optimized for smooth transitions.

---

## Development Setup

1. **Frontend**: `npm install` then `npm run dev`
2. **Backend**: `cargo build` in `src-tauri/`
3. **Full App**: `npm run tauri dev`

---

## Dependencies

### Frontend
- Svelte 5.0.0
- SvelteKit 2.9.0
- Tailwind CSS 4.1.11
- Tauri API 2.x

### Backend
- Tauri 2.x
- SQLite (via rusqlite)
- Serde for serialization
- Tokio for async operations
- RFD for file dialogs
- Chrono for date/time handling

---

This documentation covers all public APIs, functions, and components in the Storage Manager application. For additional information or specific implementation details, refer to the source code comments and inline documentation.