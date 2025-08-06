# Storage Manager Database Schema

## Overview

The Storage Manager application uses SQLite as its local database engine. The database is automatically initialized when the application starts and contains tables for all major features including settings, notes, invoices, portfolio items, code snippets, and command templates.

## Database File

- **Location**: `src-tauri/multi_tool_app.db`
- **Engine**: SQLite 3
- **Driver**: rusqlite with bundled SQLite
- **Initialization**: Automatic on application startup

## Table Schema

### Settings Table

Stores application-wide settings and configuration.

```sql
CREATE TABLE settings (
    id INTEGER PRIMARY KEY,
    s3_bucket TEXT,
    s3_region TEXT,
    s3_access_key_id TEXT,
    s3_secret_access_key TEXT,
    theme TEXT DEFAULT 'light',
    auto_upload BOOLEAN DEFAULT 0,
    user_type TEXT DEFAULT 'normal'
);
```

**Columns:**
- `id` - Primary key (always 1)
- `s3_bucket` - AWS S3 bucket name for file uploads
- `s3_region` - AWS region (default: 'us-east-1')
- `s3_access_key_id` - AWS access key ID
- `s3_secret_access_key` - AWS secret access key
- `theme` - Application theme ('light' or 'dark')
- `auto_upload` - Auto-upload setting (0 or 1)
- `user_type` - User type ('normal', 'freelancer', 'trader', 'coder')

**Default Values:**
```sql
INSERT INTO settings (
    s3_bucket, s3_region, s3_access_key_id, s3_secret_access_key, 
    theme, auto_upload, user_type
) VALUES ('', 'us-east-1', '', '', 'light', 0, 'normal');
```

### Notes Table

Stores user notes with categorization and timestamps.

```sql
CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT,
    category TEXT DEFAULT 'general',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Columns:**
- `id` - Auto-incrementing primary key
- `title` - Note title (required)
- `content` - Note content (text)
- `category` - Note category (default: 'general')
- `created_at` - Creation timestamp (ISO 8601 format)
- `updated_at` - Last update timestamp (ISO 8601 format)

**Categories:**
- `general` - General notes
- `work` - Work-related notes
- `personal` - Personal notes
- `ideas` - Ideas and concepts
- `todo` - Todo items

**Indexes:**
```sql
CREATE INDEX idx_notes_category ON notes(category);
CREATE INDEX idx_notes_updated_at ON notes(updated_at DESC);
```

### Invoices Table

Stores invoice information with line items as JSON.

```sql
CREATE TABLE invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_number TEXT NOT NULL,
    client_name TEXT NOT NULL,
    client_email TEXT,
    items TEXT,
    subtotal REAL DEFAULT 0,
    tax_rate REAL DEFAULT 0,
    tax_amount REAL DEFAULT 0,
    total REAL DEFAULT 0,
    status TEXT DEFAULT 'draft',
    due_date TEXT,
    created_at TEXT NOT NULL
);
```

**Columns:**
- `id` - Auto-incrementing primary key
- `invoice_number` - Unique invoice number (required)
- `client_name` - Client name (required)
- `client_email` - Client email address
- `items` - JSON array of invoice items
- `subtotal` - Subtotal amount (decimal)
- `tax_rate` - Tax rate percentage (decimal)
- `tax_amount` - Calculated tax amount (decimal)
- `total` - Total invoice amount (decimal)
- `status` - Invoice status (default: 'draft')
- `due_date` - Due date (ISO 8601 format)
- `created_at` - Creation timestamp (ISO 8601 format)

**Status Values:**
- `draft` - Draft invoice
- `sent` - Sent to client
- `paid` - Paid by client
- `overdue` - Past due date
- `cancelled` - Cancelled invoice

**Invoice Items JSON Structure:**
```json
[
  {
    "description": "Web Development",
    "quantity": 10.0,
    "unit_price": 100.0,
    "total": 1000.0
  }
]
```

**Indexes:**
```sql
CREATE INDEX idx_invoices_status ON invoices(status);
CREATE INDEX idx_invoices_created_at ON invoices(created_at DESC);
CREATE INDEX idx_invoices_client_name ON invoices(client_name);
```

### Portfolio Table

Stores investment portfolio positions and performance data.

```sql
CREATE TABLE portfolio (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    shares REAL NOT NULL,
    avg_price REAL NOT NULL,
    current_price REAL DEFAULT 0,
    total_value REAL DEFAULT 0,
    gain_loss REAL DEFAULT 0,
    gain_loss_percent REAL DEFAULT 0,
    notes TEXT
);
```

**Columns:**
- `id` - Auto-incrementing primary key
- `symbol` - Stock/asset symbol (required)
- `shares` - Number of shares owned (decimal)
- `avg_price` - Average purchase price (decimal)
- `current_price` - Current market price (decimal)
- `total_value` - Current total value (decimal)
- `gain_loss` - Absolute gain/loss amount (decimal)
- `gain_loss_percent` - Percentage gain/loss (decimal)
- `notes` - Additional notes about the position

**Calculated Fields:**
- `total_value = shares * current_price`
- `gain_loss = (current_price - avg_price) * shares`
- `gain_loss_percent = ((current_price - avg_price) / avg_price) * 100`

**Indexes:**
```sql
CREATE INDEX idx_portfolio_symbol ON portfolio(symbol);
CREATE INDEX idx_portfolio_gain_loss ON portfolio(gain_loss DESC);
```

### Code Snippets Table

Stores code snippets with language and tag information.

```sql
CREATE TABLE code_snippets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    code TEXT NOT NULL,
    language TEXT DEFAULT 'text',
    tags TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Columns:**
- `id` - Auto-incrementing primary key
- `title` - Snippet title (required)
- `description` - Snippet description
- `code` - Code content (required)
- `language` - Programming language (default: 'text')
- `tags` - Comma-separated tags
- `created_at` - Creation timestamp (ISO 8601 format)
- `updated_at` - Last update timestamp (ISO 8601 format)

**Supported Languages:**
- `javascript`, `typescript`, `python`, `java`, `cpp`, `csharp`
- `php`, `ruby`, `go`, `rust`, `html`, `css`, `sql`
- `json`, `yaml`, `markdown`, `shell`, `powershell`, `text`

**Indexes:**
```sql
CREATE INDEX idx_code_snippets_language ON code_snippets(language);
CREATE INDEX idx_code_snippets_updated_at ON code_snippets(updated_at DESC);
CREATE INDEX idx_code_snippets_tags ON code_snippets(tags);
```

### Command Templates Table

Stores frequently used commands and scripts.

```sql
CREATE TABLE command_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    command TEXT NOT NULL,
    category TEXT DEFAULT 'general',
    tags TEXT,
    created_at TEXT NOT NULL
);
```

**Columns:**
- `id` - Auto-incrementing primary key
- `title` - Template title (required)
- `description` - Template description
- `command` - Command content (required)
- `category` - Command category (default: 'general')
- `tags` - Comma-separated tags
- `created_at` - Creation timestamp (ISO 8601 format)

**Categories:**
- `git` - Git commands
- `docker` - Docker commands
- `database` - Database commands
- `system` - System commands
- `general` - General commands

**Indexes:**
```sql
CREATE INDEX idx_command_templates_category ON command_templates(category);
CREATE INDEX idx_command_templates_created_at ON command_templates(created_at DESC);
CREATE INDEX idx_command_templates_tags ON command_templates(tags);
```

## Data Types

### SQLite Data Types

- **INTEGER** - Integer values (auto-increment, IDs, counts)
- **REAL** - Floating-point numbers (prices, percentages, quantities)
- **TEXT** - String values (names, content, JSON)
- **BOOLEAN** - Boolean values (stored as INTEGER 0 or 1)

### JSON Storage

Complex data structures are stored as JSON strings:
- Invoice items as JSON array
- Tags as comma-separated strings
- Timestamps as ISO 8601 strings

### Timestamp Format

All timestamps use ISO 8601 format: `YYYY-MM-DDTHH:MM:SS.sssZ`

Example: `2024-01-15T14:30:00.000Z`

## Database Operations

### Initialization

The database is automatically initialized when the application starts:

```rust
fn init_db() -> SqliteResult<()> {
    let conn = Connection::open("multi_tool_app.db")?;
    
    // Create tables
    conn.execute("CREATE TABLE IF NOT EXISTS settings (...)", [])?;
    conn.execute("CREATE TABLE IF NOT EXISTS notes (...)", [])?;
    // ... other tables
    
    // Insert default settings if none exist
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))?;
    if count == 0 {
        conn.execute("INSERT INTO settings (...) VALUES (...)", params![...])?;
    }
    
    Ok(())
}
```

### Connection Management

The application uses a single database connection with mutex protection:

```rust
lazy_static! {
    static ref DB: Mutex<Option<Connection>> = Mutex::new(None);
}
```

### Error Handling

All database operations return `Result<T, String>` with descriptive error messages:

```rust
fn get_notes() -> Result<Vec<Note>, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    // Database operations with error handling
    let mut stmt = conn.prepare("SELECT ... FROM notes")
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Process results
    Ok(notes)
}
```

## Backup and Recovery

### Database Backup

The SQLite database file can be backed up by copying `multi_tool_app.db`:

```bash
# Backup database
cp src-tauri/multi_tool_app.db backup/multi_tool_app_backup.db

# Restore database
cp backup/multi_tool_app_backup.db src-tauri/multi_tool_app.db
```

### Data Export

Data can be exported using SQLite commands:

```bash
# Export all data
sqlite3 multi_tool_app.db ".dump" > backup.sql

# Export specific table
sqlite3 multi_tool_app.db "SELECT * FROM notes;" > notes_export.csv
```

### Migration Strategy

For future schema changes, implement migration functions:

```rust
fn migrate_database() -> Result<(), String> {
    let conn = Connection::open("multi_tool_app.db")?;
    
    // Check current version
    let version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap_or(0);
    
    match version {
        0 => {
            // Initial schema
            conn.execute("PRAGMA user_version = 1", [])?;
        }
        1 => {
            // Add new column
            conn.execute("ALTER TABLE notes ADD COLUMN priority INTEGER DEFAULT 0", [])?;
            conn.execute("PRAGMA user_version = 2", [])?;
        }
        _ => {
            // Already up to date
        }
    }
    
    Ok(())
}
```

## Performance Considerations

### Indexing Strategy

- Primary keys are automatically indexed
- Foreign keys (if added) should be indexed
- Frequently queried columns are indexed
- Text search columns may benefit from full-text search indexes

### Query Optimization

- Use prepared statements for repeated queries
- Limit result sets with LIMIT clauses
- Use appropriate WHERE clauses to filter data
- Consider pagination for large datasets

### Memory Management

- Close prepared statements after use
- Use transactions for multiple related operations
- Avoid loading large datasets into memory at once

## Security Considerations

### Data Protection

- Database file should be protected from unauthorized access
- Consider encryption for sensitive data (S3 credentials)
- Validate all input data before database operations
- Use parameterized queries to prevent SQL injection

### Access Control

- Application-level access control
- No direct database access for end users
- All operations go through the Rust backend

## Monitoring and Maintenance

### Database Size

Monitor database file size:
```bash
ls -lh src-tauri/multi_tool_app.db
```

### Performance Monitoring

Use SQLite's built-in performance tools:
```sql
-- Check query performance
EXPLAIN QUERY PLAN SELECT * FROM notes WHERE category = 'work';

-- Analyze table statistics
ANALYZE;

-- Check database integrity
PRAGMA integrity_check;
```

### Regular Maintenance

- Periodically run `VACUUM` to reclaim space
- Update table statistics with `ANALYZE`
- Check for database corruption with `PRAGMA integrity_check`

---

This schema documentation provides a complete reference for the Storage Manager database structure, operations, and maintenance procedures.