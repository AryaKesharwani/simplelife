use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use rusqlite::{Connection, Result as SqliteResult, params};
use lazy_static::lazy_static;
use rfd::FileDialog;

lazy_static! {
    static ref DB: Mutex<Option<Connection>> = Mutex::new(None);
}

#[derive(Debug, Serialize, Deserialize)]
struct S3Config {
    bucket: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadRequest {
    file_path: String,
    file_name: String,
    bucket: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    s3_config: S3Config,
    theme: String,
    auto_upload: bool,
    user_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: Option<i32>,
    title: String,
    content: String,
    category: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct InvoiceItem {
    description: String,
    quantity: f64,
    unit_price: f64,
    total: f64,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct CommandTemplate {
    id: Option<i32>,
    title: String,
    description: String,
    command: String,
    category: String,
    tags: String,
    created_at: String,
}

// Initialize database
fn init_db() -> SqliteResult<()> {
    let conn = Connection::open("multi_tool_app.db")?;
    
    // Settings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            s3_bucket TEXT,
            s3_region TEXT,
            s3_access_key_id TEXT,
            s3_secret_access_key TEXT,
            theme TEXT DEFAULT 'light',
            auto_upload BOOLEAN DEFAULT 0,
            user_type TEXT DEFAULT 'normal'
        )",
        [],
    )?;
    
    // Notes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT,
            category TEXT DEFAULT 'general',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;
    
    // Invoices table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS invoices (
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
        )",
        [],
    )?;
    
    // Portfolio table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS portfolio (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            symbol TEXT NOT NULL,
            shares REAL NOT NULL,
            avg_price REAL NOT NULL,
            current_price REAL DEFAULT 0,
            total_value REAL DEFAULT 0,
            gain_loss REAL DEFAULT 0,
            gain_loss_percent REAL DEFAULT 0,
            notes TEXT
        )",
        [],
    )?;
    
    // Code snippets table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS code_snippets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            code TEXT NOT NULL,
            language TEXT DEFAULT 'text',
            tags TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;
    
    // Command templates table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS command_templates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            command TEXT NOT NULL,
            category TEXT DEFAULT 'general',
            tags TEXT,
            created_at TEXT NOT NULL
        )",
        [],
    )?;
    
    // Insert default settings if none exist
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))?;
    if count == 0 {
        conn.execute(
            "INSERT INTO settings (s3_bucket, s3_region, s3_access_key_id, s3_secret_access_key, theme, auto_upload, user_type) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params!["", "us-east-1", "", "", "light", 0, "normal"],
        )?;
    }
    
    *DB.lock().unwrap() = Some(conn);
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_file_dialog() -> Result<Vec<String>, String> {
    // Create a file dialog using rfd
    let dialog = FileDialog::new()
        .set_title("Select Files")
        .add_filter("All Files", &["*"])
        .add_filter("Text Files", &["txt", "md", "json", "csv"])
        .add_filter("Images", &["jpg", "jpeg", "png", "gif", "bmp", "svg"])
        .add_filter("Documents", &["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx"])
        .add_filter("Videos", &["mp4", "avi", "mov", "wmv", "flv", "mkv"])
        .add_filter("Audio", &["mp3", "wav", "flac", "aac", "ogg"]);

    match dialog.pick_files() {
        Some(paths) => {
            let string_paths: Vec<String> = paths.into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            Ok(string_paths)
        }
        None => Ok(vec![]) // User cancelled
    }
}

#[tauri::command]
async fn upload_to_s3(request: UploadRequest) -> Result<String, String> {
    // Read the file
    let file_path = Path::new(&request.file_path);
    if !file_path.exists() {
        return Err("File does not exist".to_string());
    }

    // Read file content
    let mut file = File::open(file_path).await
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // In a real implementation, you would upload to S3 here
    // For now, we'll just simulate the upload
    println!("Would upload {} bytes to S3 bucket: {}", buffer.len(), request.bucket);
    
    Ok(format!("Successfully uploaded {} to S3", request.file_name))
}

// Settings commands
#[tauri::command]
fn get_settings() -> Result<Settings, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let row = conn.query_row(
        "SELECT s3_bucket, s3_region, s3_access_key_id, s3_secret_access_key, theme, auto_upload, user_type FROM settings LIMIT 1",
        [],
        |row| {
            Ok(Settings {
                s3_config: S3Config {
                    bucket: row.get(0)?,
                    region: row.get(1)?,
                    access_key_id: row.get(2)?,
                    secret_access_key: row.get(3)?,
                },
                theme: row.get(4)?,
                auto_upload: row.get(5)?,
                user_type: row.get(6)?,
            })
        },
    ).map_err(|e| format!("Database error: {}", e))?;
    
    Ok(row)
}

#[tauri::command]
fn save_settings(settings: Settings) -> Result<(), String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    conn.execute(
        "UPDATE settings SET 
         s3_bucket = ?, s3_region = ?, s3_access_key_id = ?, s3_secret_access_key = ?, 
         theme = ?, auto_upload = ?, user_type = ?",
        params![
            settings.s3_config.bucket,
            settings.s3_config.region,
            settings.s3_config.access_key_id,
            settings.s3_config.secret_access_key,
            settings.theme,
            if settings.auto_upload { 1 } else { 0 },
            settings.user_type,
        ],
    ).map_err(|e| format!("Database error: {}", e))?;
    
    Ok(())
}

// Notes commands
#[tauri::command]
fn get_notes() -> Result<Vec<Note>, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let mut stmt = conn.prepare("SELECT id, title, content, category, created_at, updated_at FROM notes ORDER BY updated_at DESC")
        .map_err(|e| format!("Database error: {}", e))?;
    
    let notes = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            category: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }).map_err(|e| format!("Database error: {}", e))?;
    
    let mut result = Vec::new();
    for note in notes {
        result.push(note.map_err(|e| format!("Database error: {}", e))?);
    }
    
    Ok(result)
}

#[tauri::command]
fn save_note(note: Note) -> Result<i32, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let now = chrono::Utc::now().to_rfc3339();
    
    if let Some(id) = note.id {
        // Update existing note
        conn.execute(
            "UPDATE notes SET title = ?, content = ?, category = ?, updated_at = ? WHERE id = ?",
            params![note.title, note.content, note.category, now, id],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(id)
    } else {
        // Insert new note
        let id = conn.execute(
            "INSERT INTO notes (title, content, category, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            params![note.title, note.content, note.category, now, now],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(conn.last_insert_rowid() as i32)
    }
}

#[tauri::command]
fn delete_note(id: i32) -> Result<(), String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    conn.execute("DELETE FROM notes WHERE id = ?", params![id])
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(())
}

// Invoice commands
#[tauri::command]
fn get_invoices() -> Result<Vec<Invoice>, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let mut stmt = conn.prepare("SELECT id, invoice_number, client_name, client_email, items, subtotal, tax_rate, tax_amount, total, status, due_date, created_at FROM invoices ORDER BY created_at DESC")
        .map_err(|e| format!("Database error: {}", e))?;
    
    let invoices = stmt.query_map([], |row| {
        let items_json: String = row.get(4)?;
        let items: Vec<InvoiceItem> = serde_json::from_str(&items_json).unwrap_or_default();
        
        Ok(Invoice {
            id: row.get(0)?,
            invoice_number: row.get(1)?,
            client_name: row.get(2)?,
            client_email: row.get(3)?,
            items,
            subtotal: row.get(5)?,
            tax_rate: row.get(6)?,
            tax_amount: row.get(7)?,
            total: row.get(8)?,
            status: row.get(9)?,
            due_date: row.get(10)?,
            created_at: row.get(11)?,
        })
    }).map_err(|e| format!("Database error: {}", e))?;
    
    let mut result = Vec::new();
    for invoice in invoices {
        result.push(invoice.map_err(|e| format!("Database error: {}", e))?);
    }
    
    Ok(result)
}

#[tauri::command]
fn save_invoice(invoice: Invoice) -> Result<i32, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let now = chrono::Utc::now().to_rfc3339();
    let items_json = serde_json::to_string(&invoice.items)
        .map_err(|e| format!("JSON error: {}", e))?;
    
    if let Some(id) = invoice.id {
        // Update existing invoice
        conn.execute(
            "UPDATE invoices SET invoice_number = ?, client_name = ?, client_email = ?, items = ?, subtotal = ?, tax_rate = ?, tax_amount = ?, total = ?, status = ?, due_date = ? WHERE id = ?",
            params![invoice.invoice_number, invoice.client_name, invoice.client_email, items_json, invoice.subtotal, invoice.tax_rate, invoice.tax_amount, invoice.total, invoice.status, invoice.due_date, id],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(id)
    } else {
        // Insert new invoice
        conn.execute(
            "INSERT INTO invoices (invoice_number, client_name, client_email, items, subtotal, tax_rate, tax_amount, total, status, due_date, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![invoice.invoice_number, invoice.client_name, invoice.client_email, items_json, invoice.subtotal, invoice.tax_rate, invoice.tax_amount, invoice.total, invoice.status, invoice.due_date, now],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(conn.last_insert_rowid() as i32)
    }
}

// Portfolio commands
#[tauri::command]
fn get_portfolio() -> Result<Vec<PortfolioItem>, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let mut stmt = conn.prepare("SELECT id, symbol, shares, avg_price, current_price, total_value, gain_loss, gain_loss_percent, notes FROM portfolio ORDER BY symbol")
        .map_err(|e| format!("Database error: {}", e))?;
    
    let items = stmt.query_map([], |row| {
        Ok(PortfolioItem {
            id: row.get(0)?,
            symbol: row.get(1)?,
            shares: row.get(2)?,
            avg_price: row.get(3)?,
            current_price: row.get(4)?,
            total_value: row.get(5)?,
            gain_loss: row.get(6)?,
            gain_loss_percent: row.get(7)?,
            notes: row.get(8)?,
        })
    }).map_err(|e| format!("Database error: {}", e))?;
    
    let mut result = Vec::new();
    for item in items {
        result.push(item.map_err(|e| format!("Database error: {}", e))?);
    }
    
    Ok(result)
}

#[tauri::command]
fn save_portfolio_item(item: PortfolioItem) -> Result<i32, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    if let Some(id) = item.id {
        // Update existing item
        conn.execute(
            "UPDATE portfolio SET symbol = ?, shares = ?, avg_price = ?, current_price = ?, total_value = ?, gain_loss = ?, gain_loss_percent = ?, notes = ? WHERE id = ?",
            params![item.symbol, item.shares, item.avg_price, item.current_price, item.total_value, item.gain_loss, item.gain_loss_percent, item.notes, id],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(id)
    } else {
        // Insert new item
        conn.execute(
            "INSERT INTO portfolio (symbol, shares, avg_price, current_price, total_value, gain_loss, gain_loss_percent, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![item.symbol, item.shares, item.avg_price, item.current_price, item.total_value, item.gain_loss, item.gain_loss_percent, item.notes],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(conn.last_insert_rowid() as i32)
    }
}

// Code snippets commands
#[tauri::command]
fn get_code_snippets() -> Result<Vec<CodeSnippet>, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let mut stmt = conn.prepare("SELECT id, title, description, code, language, tags, created_at, updated_at FROM code_snippets ORDER BY updated_at DESC")
        .map_err(|e| format!("Database error: {}", e))?;
    
    let snippets = stmt.query_map([], |row| {
        Ok(CodeSnippet {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            code: row.get(3)?,
            language: row.get(4)?,
            tags: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }).map_err(|e| format!("Database error: {}", e))?;
    
    let mut result = Vec::new();
    for snippet in snippets {
        result.push(snippet.map_err(|e| format!("Database error: {}", e))?);
    }
    
    Ok(result)
}

#[tauri::command]
fn save_code_snippet(snippet: CodeSnippet) -> Result<i32, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let now = chrono::Utc::now().to_rfc3339();
    
    if let Some(id) = snippet.id {
        // Update existing snippet
        conn.execute(
            "UPDATE code_snippets SET title = ?, description = ?, code = ?, language = ?, tags = ?, updated_at = ? WHERE id = ?",
            params![snippet.title, snippet.description, snippet.code, snippet.language, snippet.tags, now, id],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(id)
    } else {
        // Insert new snippet
        conn.execute(
            "INSERT INTO code_snippets (title, description, code, language, tags, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![snippet.title, snippet.description, snippet.code, snippet.language, snippet.tags, now, now],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(conn.last_insert_rowid() as i32)
    }
}

// Command templates commands
#[tauri::command]
fn get_command_templates() -> Result<Vec<CommandTemplate>, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let mut stmt = conn.prepare("SELECT id, title, description, command, category, tags, created_at FROM command_templates ORDER BY created_at DESC")
        .map_err(|e| format!("Database error: {}", e))?;
    
    let templates = stmt.query_map([], |row| {
        Ok(CommandTemplate {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            command: row.get(3)?,
            category: row.get(4)?,
            tags: row.get(5)?,
            created_at: row.get(6)?,
        })
    }).map_err(|e| format!("Database error: {}", e))?;
    
    let mut result = Vec::new();
    for template in templates {
        result.push(template.map_err(|e| format!("Database error: {}", e))?);
    }
    
    Ok(result)
}

#[tauri::command]
fn save_command_template(template: CommandTemplate) -> Result<i32, String> {
    let db_guard = DB.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let now = chrono::Utc::now().to_rfc3339();
    
    if let Some(id) = template.id {
        // Update existing template
        conn.execute(
            "UPDATE command_templates SET title = ?, description = ?, command = ?, category = ?, tags = ? WHERE id = ?",
            params![template.title, template.description, template.command, template.category, template.tags, id],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(id)
    } else {
        // Insert new template
        conn.execute(
            "INSERT INTO command_templates (title, description, command, category, tags, created_at) VALUES (?, ?, ?, ?, ?, ?)",
            params![template.title, template.description, template.command, template.category, template.tags, now],
        ).map_err(|e| format!("Database error: {}", e))?;
        Ok(conn.last_insert_rowid() as i32)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize database
    if let Err(e) = init_db() {
        eprintln!("Failed to initialize database: {}", e);
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            open_file_dialog, 
            upload_to_s3, 
            get_settings, 
            save_settings,
            get_notes,
            save_note,
            delete_note,
            get_invoices,
            save_invoice,
            get_portfolio,
            save_portfolio_item,
            get_code_snippets,
            save_code_snippet,
            get_command_templates,
            save_command_template
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
