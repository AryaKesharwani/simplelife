import { writable } from 'svelte/store';
import { browser } from '$app/environment';

// Theme store
export const theme = writable(browser ? localStorage.getItem('theme') || 'light' : 'light');

// Function to apply theme to DOM
function applyTheme(themeValue: string) {
  if (browser && typeof document !== 'undefined') {
    console.log('Applying theme:', themeValue);
    document.documentElement.classList.toggle('dark', themeValue === 'dark');
    console.log('Dark class applied:', document.documentElement.classList.contains('dark'));
    console.log('All classes on html:', document.documentElement.className);
  }
}

// Subscribe to theme changes and update localStorage
if (browser) {
  theme.subscribe((value: string) => {
    console.log('Theme changed to:', value);
    localStorage.setItem('theme', value);
    applyTheme(value);
  });
}

// Initialize theme function
export function initializeTheme() {
  if (browser) {
    const savedTheme = localStorage.getItem('theme') || 'light';
    console.log('Initializing theme with:', savedTheme);
    theme.set(savedTheme);
    applyTheme(savedTheme);
  }
} 