# Simple Life - Multi-Tool Hub

A comprehensive desktop application built with SvelteKit and Tauri that provides multiple productivity tools in one place.

## Features

### 🎨 **Modern Design**
- **Google Fonts Integration**: Uses Inter and Poppins fonts for beautiful typography
- **Dark/Light Mode**: Seamless theme switching with persistent preferences
- **Responsive Design**: Works perfectly on all screen sizes
- **Smooth Animations**: Elegant transitions and hover effects

### 🛠️ **Multi-Tool Functionality**
- **Personal Tools**: Notes, reminders, file management, calculator
- **Freelancer Tools**: Invoices, time tracking, client management
- **Trader Tools**: Portfolio tracking, market analysis
- **Developer Tools**: Code snippets, command templates

### 🌙 **Theme System**
- **Automatic Theme Detection**: Respects system preferences
- **Manual Theme Switching**: Toggle between light and dark modes
- **Persistent Settings**: Theme preference is saved locally
- **Smooth Transitions**: Elegant color transitions when switching themes

## Getting Started

### Prerequisites
- Node.js (v18 or higher)
- Rust (for Tauri)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd Storage-Manager
```

2. Install dependencies:
```bash
npm install
```

3. Run the development server:
```bash
npm run dev
```

4. Build for production:
```bash
npm run build
```

## Theme Customization

The app includes a sophisticated theme system:

### Google Fonts
- **Inter**: Used for body text and general content
- **Poppins**: Used for headings and titles
- Both fonts are loaded with multiple weights (300, 400, 500, 600, 700)

### Dark/Light Mode
- Click the theme switcher button in the header to toggle themes
- Theme preference is automatically saved to localStorage
- Smooth transitions between themes
- All components are fully themed for both modes

### Theme Switcher Component
Located at `src/lib/components/ThemeSwitcher.svelte`, this component provides:
- Animated sun/moon icons
- Hover effects with gradient overlays
- Accessibility features (aria-labels, keyboard navigation)
- Integration with the global theme store

## Project Structure

```
src/
├── lib/
│   ├── components/
│   │   └── ThemeSwitcher.svelte    # Theme switching component
│   └── stores/
│       └── theme.ts                # Global theme store
├── routes/
│   ├── +layout.svelte              # Main layout with theme support
│   ├── +page.svelte                # Home page with theme switcher
│   └── settings/+page.svelte       # Settings page with theme controls
├── app.css                         # Global styles with Google Fonts
└── app.html                        # HTML template with font links
```

## Technology Stack

- **Frontend**: SvelteKit with TypeScript
- **Desktop**: Tauri (Rust + WebView)
- **Styling**: Tailwind CSS
- **Fonts**: Google Fonts (Inter + Poppins)
- **State Management**: Svelte stores
- **Database**: SQLite (via Tauri)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License.
