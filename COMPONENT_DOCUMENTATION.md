# Storage Manager Component Documentation

## Overview

This document provides comprehensive documentation for all frontend components, pages, and user interfaces in the Storage Manager application. The application is built with Svelte 5 and uses Tailwind CSS for styling.

## Table of Contents

1. [Core Components](#core-components)
2. [Page Components](#page-components)
3. [Layout Components](#layout-components)
4. [Store Management](#store-management)
5. [Styling and Theming](#styling-and-theming)
6. [Component Architecture](#component-architecture)

---

## Core Components

### ThemeSwitcher Component

**File:** `src/lib/components/ThemeSwitcher.svelte`

A reusable component for toggling between light and dark themes with smooth animations.

**Features:**
- Smooth icon transitions between sun and moon
- Hover effects with gradient overlay
- Accessible with ARIA labels
- Responsive design
- Automatic theme persistence

**Props:**
- None (uses global theme store)

**Events:**
- None (internal state management)

**State:**
- `isDark: boolean` - Current theme state

**Methods:**
- `toggleTheme()` - Toggles between light and dark themes

**Usage:**
```svelte
<script>
  import ThemeSwitcher from '$lib/components/ThemeSwitcher.svelte';
</script>

<ThemeSwitcher />
```

**Styling Classes:**
- Container: `relative p-2 text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200`
- Icons: `w-5 h-5 transition-all duration-300`
- Hover overlay: `absolute inset-0 rounded-lg bg-gradient-to-r from-yellow-400/20 to-orange-500/20`

---

## Page Components

### Main Dashboard Page

**File:** `src/routes/+page.svelte`

The main application dashboard that adapts based on user type and provides access to different tools.

**Features:**
- User type selection (Personal, Freelancer, Trader, Developer)
- Dynamic tool grid based on user type
- Settings management
- Theme switching
- Responsive design

**State Variables:**
- `settings: Settings` - Application settings
- `isLoading: boolean` - Loading state
- `selectedUserType: string` - Current user type
- `showContextMenu: boolean` - Context menu visibility

**User Types:**
1. **Personal** - Notes, reminders, file management, calculator
2. **Freelancer** - Invoices, time tracking, client management, expenses
3. **Trader** - Portfolio tracking, watchlist, trade journal, market news
4. **Developer** - Code snippets, command templates, API testing, database browser

**Tool Categories by User Type:**

#### Personal Tools
- Notes (`/notes`) - Create and manage personal notes
- Reminders (`/reminders`) - Set and track reminders
- File Manager (`/files`) - Upload files to cloud storage
- Calculator (`/calculator`) - Simple calculator tool

#### Freelancer Tools
- Invoices (`/invoices`) - Create and manage client invoices
- Time Tracker (`/time-tracker`) - Track billable hours
- Client Manager (`/clients`) - Manage client information
- Expense Tracker (`/expenses`) - Track business expenses

#### Trader Tools
- Portfolio (`/portfolio`) - Track your investment portfolio
- Watchlist (`/watchlist`) - Monitor stocks and assets
- Trade Journal (`/trade-journal`) - Log your trading activities
- Market News (`/news`) - Stay updated with market news

#### Developer Tools
- Code Snippets (`/snippets`) - Store and manage code snippets
- Command Templates (`/commands`) - Save frequently used commands
- API Tester (`/api-tester`) - Test REST APIs
- Database Browser (`/database`) - Browse and query databases

**Methods:**
- `loadSettings()` - Load application settings
- `saveSettings()` - Save application settings
- `selectUserType(type: string)` - Change user type
- `navigateToTool(route: string)` - Navigate to tool page

### Notes Page

**File:** `src/routes/notes/+page.svelte`

A comprehensive note-taking interface with categorization and search functionality.

**Features:**
- Create, edit, and delete notes
- Category-based filtering
- Search functionality
- Modal-based editing
- Responsive grid layout

**State Variables:**
- `notes: Note[]` - Array of notes
- `isLoading: boolean` - Loading state
- `showCreateModal: boolean` - Modal visibility
- `selectedNote: Note | null` - Currently selected note
- `searchTerm: string` - Search query
- `selectedCategory: string` - Selected category filter
- `newNote: Note` - Note being created/edited

**Categories:**
- All Notes (📝)
- General (📄)
- Work (💼)
- Personal (👤)
- Ideas (💡)
- Todo (✅)

**Methods:**
- `loadNotes()` - Load all notes from database
- `saveNote()` - Save current note
- `deleteNote(id: number)` - Delete note by ID
- `openCreateModal()` - Open create note modal
- `openEditModal(note: Note)` - Open edit note modal
- `closeModal()` - Close modal
- `filterNotes()` - Filter notes by search and category

**Note Interface:**
```typescript
interface Note {
  id?: number;
  title: string;
  content: string;
  category: string;
  created_at: string;
  updated_at: string;
}
```

**UI Elements:**
- Search bar with real-time filtering
- Category filter buttons
- Note grid with cards
- Create/Edit modal with form
- Delete confirmation dialog

### Invoices Page

**File:** `src/routes/invoices/+page.svelte`

Invoice management interface for creating and managing client invoices.

**Features:**
- Create and edit invoices
- Client information management
- Line item management
- Tax calculation
- Status tracking
- PDF generation (planned)

**State Variables:**
- `invoices: Invoice[]` - Array of invoices
- `isLoading: boolean` - Loading state
- `showCreateModal: boolean` - Modal visibility
- `selectedInvoice: Invoice | null` - Currently selected invoice
- `newInvoice: Invoice` - Invoice being created/edited

**Invoice Interface:**
```typescript
interface Invoice {
  id?: number;
  invoice_number: string;
  client_name: string;
  client_email: string;
  items: InvoiceItem[];
  subtotal: number;
  tax_rate: number;
  tax_amount: number;
  total: number;
  status: string;
  due_date: string;
  created_at: string;
}

interface InvoiceItem {
  description: string;
  quantity: number;
  unit_price: number;
  total: number;
}
```

**Methods:**
- `loadInvoices()` - Load all invoices
- `saveInvoice()` - Save current invoice
- `deleteInvoice(id: number)` - Delete invoice
- `calculateTotals()` - Calculate invoice totals
- `addLineItem()` - Add new line item
- `removeLineItem(index: number)` - Remove line item

### Portfolio Page

**File:** `src/routes/portfolio/+page.svelte`

Portfolio tracking interface for managing investment positions.

**Features:**
- Add and edit portfolio positions
- Real-time price tracking (planned)
- Gain/loss calculations
- Position notes
- Portfolio summary

**State Variables:**
- `portfolio: PortfolioItem[]` - Array of portfolio items
- `isLoading: boolean` - Loading state
- `showCreateModal: boolean` - Modal visibility
- `selectedItem: PortfolioItem | null` - Currently selected item
- `newItem: PortfolioItem` - Item being created/edited

**PortfolioItem Interface:**
```typescript
interface PortfolioItem {
  id?: number;
  symbol: string;
  shares: number;
  avg_price: number;
  current_price: number;
  total_value: number;
  gain_loss: number;
  gain_loss_percent: number;
  notes: string;
}
```

**Methods:**
- `loadPortfolio()` - Load all portfolio items
- `savePortfolioItem()` - Save current item
- `deletePortfolioItem(id: number)` - Delete item
- `calculateGainLoss()` - Calculate gain/loss
- `updateCurrentPrices()` - Update current prices (planned)

### Code Snippets Page

**File:** `src/routes/snippets/+page.svelte`

Code snippet management interface for storing and organizing code examples.

**Features:**
- Create and edit code snippets
- Language-based syntax highlighting
- Tag-based organization
- Search functionality
- Copy to clipboard

**State Variables:**
- `snippets: CodeSnippet[]` - Array of code snippets
- `isLoading: boolean` - Loading state
- `showCreateModal: boolean` - Modal visibility
- `selectedSnippet: CodeSnippet | null` - Currently selected snippet
- `newSnippet: CodeSnippet` - Snippet being created/edited
- `searchTerm: string` - Search query
- `selectedLanguage: string` - Selected language filter

**CodeSnippet Interface:**
```typescript
interface CodeSnippet {
  id?: number;
  title: string;
  description: string;
  code: string;
  language: string;
  tags: string;
  created_at: string;
  updated_at: string;
}
```

**Supported Languages:**
- JavaScript, TypeScript, Python, Java, C++, C#, PHP, Ruby, Go, Rust, HTML, CSS, SQL, JSON, YAML, Markdown, Shell, PowerShell

**Methods:**
- `loadSnippets()` - Load all code snippets
- `saveSnippet()` - Save current snippet
- `deleteSnippet(id: number)` - Delete snippet
- `copyToClipboard(code: string)` - Copy code to clipboard
- `filterSnippets()` - Filter snippets by search and language

### Command Templates Page

**File:** `src/routes/commands/+page.svelte`

Command template management interface for storing frequently used commands.

**Features:**
- Create and edit command templates
- Category-based organization
- Tag-based filtering
- Copy to clipboard
- Command execution (planned)

**State Variables:**
- `templates: CommandTemplate[]` - Array of command templates
- `isLoading: boolean` - Loading state
- `showCreateModal: boolean` - Modal visibility
- `selectedTemplate: CommandTemplate | null` - Currently selected template
- `newTemplate: CommandTemplate` - Template being created/edited
- `searchTerm: string` - Search query
- `selectedCategory: string` - Selected category filter

**CommandTemplate Interface:**
```typescript
interface CommandTemplate {
  id?: number;
  title: string;
  description: string;
  command: string;
  category: string;
  tags: string;
  created_at: string;
}
```

**Categories:**
- Git commands
- Docker commands
- Database commands
- System commands
- Custom scripts

**Methods:**
- `loadTemplates()` - Load all command templates
- `saveTemplate()` - Save current template
- `deleteTemplate(id: number)` - Delete template
- `copyToClipboard(command: string)` - Copy command to clipboard
- `executeCommand(command: string)` - Execute command (planned)

---

## Layout Components

### Main Layout

**File:** `src/routes/+layout.svelte`

The main layout wrapper that initializes the application theme and provides the base structure.

**Features:**
- Theme initialization
- Global CSS import
- Child component rendering

**Methods:**
- `initializeTheme()` - Initialize theme from localStorage

**Usage:**
```svelte
<script>
  import { onMount } from 'svelte';
  import { initializeTheme } from '../lib/stores/theme';
  import "../app.css";
  
  let { children } = $props();
  
  onMount(() => {
    initializeTheme();
  });
</script>

{@render children()}
```

---

## Store Management

### Theme Store

**File:** `src/lib/stores/theme.ts`

A Svelte writable store for managing application theme state with persistence.

**Exports:**
- `theme: WritableStore<string>` - The theme store
- `initializeTheme(): void` - Function to initialize theme on app startup

**Features:**
- Persists theme preference in localStorage
- Automatically applies theme to document root
- Browser-safe with environment checks
- Console logging for debugging

**API:**
- `theme.set(value: string)` - Set theme to 'light' or 'dark'
- `theme.subscribe(callback)` - Subscribe to theme changes
- `initializeTheme()` - Initialize theme from localStorage

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

---

## Styling and Theming

### Tailwind CSS Integration

The application uses Tailwind CSS 4.1.11 for styling with custom configuration.

**Features:**
- Dark mode support with `dark:` prefix
- Responsive design with breakpoint prefixes
- Custom color schemes
- Smooth transitions and animations

**Dark Mode Classes:**
- `dark:bg-gray-900` - Dark background
- `dark:text-white` - Dark text
- `dark:border-gray-700` - Dark borders

**Common Patterns:**
```css
/* Card styling */
.card {
  @apply bg-white dark:bg-gray-800 rounded-lg shadow-md p-6;
}

/* Button styling */
.btn-primary {
  @apply bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded;
}

/* Input styling */
.input {
  @apply border border-gray-300 dark:border-gray-600 rounded px-3 py-2;
}
```

### Theme System

The application supports light and dark themes with automatic persistence.

**Theme Variables:**
- `--bg-primary` - Primary background color
- `--text-primary` - Primary text color
- `--border-primary` - Primary border color

**Theme Switching:**
```typescript
// Toggle theme
function toggleTheme() {
  theme.update(current => current === 'light' ? 'dark' : 'light');
}
```

---

## Component Architecture

### State Management

The application uses Svelte 5's new state management with `$state` and `$props`.

**State Patterns:**
```typescript
// Local state
let isLoading = $state(true);
let data = $state<DataType[]>([]);

// Props
let { title, description } = $props<{
  title: string;
  description: string;
}>();
```

### Event Handling

Components use Svelte's event system for user interactions.

**Event Patterns:**
```svelte
<!-- Click events -->
<button on:click={handleClick}>Click me</button>

<!-- Form events -->
<form on:submit={handleSubmit}>
  <input on:input={handleInput} />
</form>

<!-- Custom events -->
<CustomComponent on:customEvent={handleCustomEvent} />
```

### Lifecycle Management

Components use Svelte's lifecycle functions for initialization and cleanup.

**Lifecycle Patterns:**
```typescript
import { onMount, onDestroy } from 'svelte';

onMount(() => {
  // Component initialization
  loadData();
  
  // Cleanup function
  return () => {
    // Cleanup code
  };
});
```

### Error Handling

Components implement comprehensive error handling for API calls and user interactions.

**Error Patterns:**
```typescript
async function loadData() {
  try {
    isLoading = true;
    const result = await invoke('get_data');
    data = result;
  } catch (error) {
    console.error('Error loading data:', error);
    // Show user-friendly error message
  } finally {
    isLoading = false;
  }
}
```

---

## Responsive Design

### Breakpoint System

The application uses Tailwind's responsive breakpoints:

- `sm:` - 640px and up
- `md:` - 768px and up
- `lg:` - 1024px and up
- `xl:` - 1280px and up
- `2xl:` - 1536px and up

### Mobile-First Approach

Components are designed mobile-first with progressive enhancement.

**Responsive Patterns:**
```svelte
<!-- Mobile-first grid -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
  <!-- Grid items -->
</div>

<!-- Responsive text -->
<h1 class="text-xl md:text-2xl lg:text-3xl">Title</h1>

<!-- Responsive spacing -->
<div class="p-4 md:p-6 lg:p-8">Content</div>
```

---

## Accessibility

### ARIA Support

Components include proper ARIA attributes for screen readers.

**ARIA Patterns:**
```svelte
<!-- Button with aria-label -->
<button aria-label="Toggle theme" title="Toggle dark/light mode">
  <!-- Icon -->
</button>

<!-- Form with labels -->
<label for="title">Title</label>
<input id="title" type="text" aria-describedby="title-help" />

<!-- Modal with proper focus management -->
<div role="dialog" aria-labelledby="modal-title" aria-modal="true">
  <h2 id="modal-title">Modal Title</h2>
</div>
```

### Keyboard Navigation

Components support keyboard navigation for accessibility.

**Keyboard Patterns:**
```svelte
<!-- Keyboard event handling -->
<input 
  on:keydown={(e) => {
    if (e.key === 'Enter') handleSubmit();
    if (e.key === 'Escape') closeModal();
  }}
/>

<!-- Focus management -->
<button bind:this={focusButton}>Focus me</button>
```

---

## Performance Optimization

### Lazy Loading

Components implement lazy loading for better performance.

**Lazy Loading Patterns:**
```typescript
// Dynamic imports
const Component = await import('./Component.svelte');

// Conditional rendering
{#if showComponent}
  <Component />
{/if}
```

### Memoization

Expensive calculations are memoized using Svelte's reactive statements.

**Memoization Patterns:**
```typescript
// Reactive statement for filtered data
$: filteredData = data.filter(item => 
  item.name.toLowerCase().includes(searchTerm.toLowerCase())
);

// Reactive statement for computed values
$: totalValue = portfolio.reduce((sum, item) => sum + item.total_value, 0);
```

---

This documentation provides a comprehensive overview of all frontend components, their features, and usage patterns in the Storage Manager application. For implementation details, refer to the source code and inline comments.