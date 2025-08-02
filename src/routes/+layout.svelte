<script>
  import { browser } from '$app/environment';
  import { onMount } from 'svelte';
  import { initializeTheme } from '../lib/stores/theme';
  import "../app.css";
  
  let { children } = $props();
  
  onMount(() => {
    if (browser) {
      initializeTheme();
    }
  });
</script>

{@render children()} 