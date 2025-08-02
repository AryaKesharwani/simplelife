<script lang="ts">
  import { theme } from '../../lib/stores/theme';
  import { browser } from '$app/environment';
  import { onMount } from 'svelte';
  
  let isDark = $state(false);
  
  onMount(() => {
    if (browser) {
      // Subscribe to theme changes
      const unsubscribe = theme.subscribe((value: string) => {
        console.log('ThemeSwitcher received theme:', value);
        isDark = value === 'dark';
      });
      
      // Cleanup subscription on component destroy
      return unsubscribe;
    }
  });
  
  function toggleTheme() {
    if (browser) {
      console.log('Toggle theme clicked, current isDark:', isDark);
      const newTheme = isDark ? 'light' : 'dark';
      console.log('Setting theme to:', newTheme);
      theme.set(newTheme);
    }
  }
</script>

<button
  onclick={toggleTheme}
  class="relative p-2 text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors duration-200 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 group"
  aria-label="Toggle theme"
  title="Toggle dark/light mode"
>
  <!-- Sun icon for light mode -->
  <svg
    class="w-5 h-5 transition-all duration-300 {isDark ? 'opacity-0 scale-0 rotate-90' : 'opacity-100 scale-100 rotate-0'}"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
    />
  </svg>
  
  <!-- Moon icon for dark mode -->
  <svg
    class="absolute inset-0 w-5 h-5 transition-all duration-300 {isDark ? 'opacity-100 scale-100 rotate-0' : 'opacity-0 scale-0 -rotate-90'}"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
    />
  </svg>
  
  <!-- Ripple effect -->
  <div class="absolute inset-0 rounded-lg bg-gradient-to-r from-yellow-400/20 to-orange-500/20 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
</button> 