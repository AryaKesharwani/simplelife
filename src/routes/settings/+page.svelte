<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import ThemeSwitcher from '../../lib/components/ThemeSwitcher.svelte';
  import { theme } from '../../lib/stores/theme';

  interface Settings {
    s3_config: {
      bucket: string;
      region: string;
      access_key_id: string;
      secret_access_key: string;
    };
    theme: string;
    auto_upload: boolean;
    user_type: string;
  }

  let settings = $state<Settings>({
    s3_config: {
      bucket: "",
      region: "us-east-1",
      access_key_id: "",
      secret_access_key: ""
    },
    theme: "light",
    auto_upload: false,
    user_type: "normal"
  });

  let isLoading = $state(true);
  let isSaving = $state(false);
  let saveMessage = $state("");

  const userTypes = [
    { id: "normal", name: "Normal User", description: "Notes, reminders, and personal tools" },
    { id: "freelancer", name: "Freelancer", description: "Invoices, time tracking, and client management" },
    { id: "trader", name: "Trader", description: "Portfolio tracking, market analysis, and trading tools" },
    { id: "coder", name: "Coder", description: "Code snippets, command templates, and development tools" }
  ];

  onMount(async () => {
    try {
      const savedSettings = await invoke('get_settings') as Settings;
      settings = savedSettings;
    } catch (error) {
      console.error('Error loading settings:', error);
    } finally {
      isLoading = false;
    }
  });

  async function saveSettings() {
    isSaving = true;
    saveMessage = "";

    try {
      await invoke('save_settings', { settings });
      saveMessage = "✅ Settings saved successfully!";
    } catch (error) {
      saveMessage = `❌ Error saving settings: ${error}`;
    } finally {
      isSaving = false;
    }
  }

  function resetSettings() {
    settings = {
      s3_config: {
        bucket: "",
        region: "us-east-1",
        access_key_id: "",
        secret_access_key: ""
      },
      theme: "light",
      auto_upload: false,
      user_type: "normal"
    };
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-indigo-100 via-white to-blue-100 dark:from-gray-950 dark:via-gray-900 dark:to-gray-950 transition-colors duration-300 ease-in-out">
  <!-- Sticky Header -->
  <header class="sticky top-0 z-50 bg-white/80 dark:bg-gray-800/80 backdrop-blur border-b border-gray-200 dark:border-gray-700 transition-all">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center py-6">
        <a href="/" class="flex items-center gap-2 text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
          </svg>
          <span>Back to Multi-Tool App</span>
        </a>
        <div class="flex items-center gap-4">
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Settings</h1>
          <ThemeSwitcher />
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if isLoading}
      <div class="flex items-center justify-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    {:else}
      <div class="space-y-10">
        <!-- User Type -->
        <section class="bg-white dark:bg-gray-800 rounded-2xl shadow-md border border-gray-200 dark:border-gray-700 p-6 transition-shadow hover:shadow-lg">
          <div class="flex items-center gap-3 mb-6">
            <div class="w-8 h-8 bg-purple-100 dark:bg-purple-900/30 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
              </svg>
            </div>
            <h2 class="text-xl font-semibold text-gray-900 dark:text-white">User Type</h2>
          </div>

          <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
            Choose your primary user type to customize available tools.
          </p>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each userTypes as userType}
              <label class="relative flex cursor-pointer rounded-lg border bg-white dark:bg-gray-800 p-4 shadow-sm transition-all duration-300 ease-in-out
              {settings.user_type === userType.id ? 'border-purple-600 ring-2 ring-purple-400/50 animate-pulse' : 'border-gray-200 dark:border-gray-700'}">
                <input 
                  type="radio" 
                  name="user-type" 
                  value={userType.id}
                  bind:group={settings.user_type}
                  class="sr-only"
                />
                <span class="flex-1 flex flex-col">
                  <span class="text-sm font-medium text-gray-900 dark:text-white">{userType.name}</span>
                  <span class="mt-1 text-sm text-gray-500 dark:text-gray-400">{userType.description}</span>
                </span>
              </label>
            {/each}
          </div>
        </section>

        <!-- Application Settings -->
        <section class="bg-white dark:bg-gray-800 rounded-2xl shadow-md border border-gray-200 dark:border-gray-700 p-6">
          <div class="flex items-center gap-3 mb-6">
            <div class="w-8 h-8 bg-blue-100 dark:bg-blue-900/30 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
              </svg>
            </div>
            <h2 class="text-xl font-semibold text-gray-900 dark:text-white">Application Settings</h2>
          </div>

          <div class="space-y-6">
            <div>
              <label for="theme" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Theme</label>
              <select 
                id="theme" 
                bind:value={settings.theme}
                onchange={() => theme.set(settings.theme)}
                class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="auto">Auto (System)</option>
              </select>
            </div>

            <div class="flex items-center">
              <input 
                id="autoUpload" 
                type="checkbox" 
                bind:checked={settings.auto_upload}
                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
              />
              <label for="autoUpload" class="ml-2 text-sm text-gray-700 dark:text-gray-300">
                Auto-upload files when selected
              </label>
            </div>
          </div>
        </section>

        <!-- Action Buttons -->
        <div class="flex flex-col sm:flex-row gap-4">
          <button 
            onclick={saveSettings} 
            disabled={isSaving}
            class="flex-1 flex items-center justify-center px-6 py-3 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 disabled:from-gray-400 disabled:to-gray-400 text-white font-semibold rounded-lg transition-all duration-200 shadow-md hover:shadow-lg disabled:shadow-none"
          >
            {#if isSaving}
              <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
              </svg>
              Saving...
            {:else}
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
              </svg>
              Save Settings
            {/if}
          </button>

          <button 
            onclick={resetSettings} 
            class="flex-1 px-6 py-3 bg-gray-100 hover:bg-gray-200 text-gray-700 dark:bg-gray-700 dark:hover:bg-gray-600 dark:text-gray-300 font-semibold rounded-lg transition"
          >
            Reset to Defaults
          </button>
        </div>

        {#if saveMessage}
          <div class="mt-4 p-4 rounded-lg {saveMessage.includes('Error') ? 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400' : 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400'}">
            {saveMessage}
          </div>
        {/if}
      </div>
    {/if}
  </main>
</div>
