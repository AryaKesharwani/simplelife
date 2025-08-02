<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import ThemeSwitcher from '../lib/components/ThemeSwitcher.svelte';

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
  let selectedUserType = $state("normal");
  let showContextMenu = $state(false);

  const userTypes = [
    { 
      id: "normal", 
      name: "Personal", 
      icon: "👤", 
      description: "Notes, reminders, and personal tools",
      color: "from-blue-500 to-cyan-500",
      bgColor: "bg-blue-50 dark:bg-blue-900/20"
    },
    { 
      id: "freelancer", 
      name: "Freelancer", 
      icon: "💼", 
      description: "Invoices, time tracking, and client management",
      color: "from-purple-500 to-pink-500",
      bgColor: "bg-purple-50 dark:bg-purple-900/20"
    },
    { 
      id: "trader", 
      name: "Trader", 
      icon: "📈", 
      description: "Portfolio tracking, market analysis, and trading tools",
      color: "from-green-500 to-emerald-500",
      bgColor: "bg-green-50 dark:bg-green-900/20"
    },
    { 
      id: "coder", 
      name: "Developer", 
      icon: "💻", 
      description: "Code snippets, command templates, and development tools",
      color: "from-orange-500 to-red-500",
      bgColor: "bg-orange-50 dark:bg-orange-900/20"
    }
  ];

  const toolsByUserType: Record<string, Array<{name: string; icon: string; route: string; description: string; color: string}>> = {
    normal: [
      { name: "Notes", icon: "📝", route: "/notes", description: "Create and manage personal notes", color: "from-blue-500 to-blue-600" },
      { name: "Reminders", icon: "⏰", route: "/reminders", description: "Set and track reminders", color: "from-purple-500 to-purple-600" },
      { name: "File Manager", icon: "📁", route: "/files", description: "Upload files to cloud storage", color: "from-green-500 to-green-600" },
      { name: "Calculator", icon: "🧮", route: "/calculator", description: "Simple calculator tool", color: "from-orange-500 to-orange-600" }
    ],
    freelancer: [
      { name: "Invoices", icon: "💰", route: "/invoices", description: "Create and manage client invoices", color: "from-green-500 to-green-600" },
      { name: "Time Tracker", icon: "⏱️", route: "/time-tracker", description: "Track billable hours", color: "from-blue-500 to-blue-600" },
      { name: "Client Manager", icon: "👥", route: "/clients", description: "Manage client information", color: "from-purple-500 to-purple-600" },
      { name: "Expense Tracker", icon: "💳", route: "/expenses", description: "Track business expenses", color: "from-red-500 to-red-600" }
    ],
    trader: [
      { name: "Portfolio", icon: "📊", route: "/portfolio", description: "Track your investment portfolio", color: "from-green-500 to-green-600" },
      { name: "Watchlist", icon: "👀", route: "/watchlist", description: "Monitor stocks and assets", color: "from-blue-500 to-blue-600" },
      { name: "Trade Journal", icon: "📓", route: "/trade-journal", description: "Log your trading activities", color: "from-purple-500 to-purple-600" },
      { name: "Market News", icon: "📰", route: "/news", description: "Stay updated with market news", color: "from-orange-500 to-orange-600" }
    ],
    coder: [
      { name: "Code Snippets", icon: "💻", route: "/snippets", description: "Store and manage code snippets", color: "from-blue-500 to-blue-600" },
      { name: "Command Templates", icon: "⌨️", route: "/commands", description: "Save frequently used commands", color: "from-green-500 to-green-600" },
      { name: "API Tester", icon: "🔌", route: "/api-tester", description: "Test REST APIs", color: "from-purple-500 to-purple-600" },
      { name: "Database Browser", icon: "🗄️", route: "/database", description: "Browse and query databases", color: "from-orange-500 to-orange-600" }
    ]
  };

  onMount(async () => {
    try {
      const savedSettings = await invoke('get_settings') as Settings;
      settings = savedSettings;
      selectedUserType = savedSettings.user_type;
    } catch (error) {
      console.error('Error loading settings:', error);
    } finally {
      isLoading = false;
    }
  });

  async function updateUserType(userType: string) {
    selectedUserType = userType;
    settings.user_type = userType;
    
    try {
      await invoke('save_settings', { settings });
    } catch (error) {
      console.error('Error saving user type:', error);
    }
  }

  function getCurrentUserType() {
    return userTypes.find(u => u.id === selectedUserType) || userTypes[0];
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-50 dark:from-gray-950 dark:via-gray-900 dark:to-gray-950 transition-all duration-300">
  <!-- Sticky Header with Context Switcher -->
  <header class="sticky top-0 z-50 bg-white/80 dark:bg-gray-800/80 backdrop-blur-md border-b border-gray-200 dark:border-gray-700 shadow-sm">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between py-4">
        <!-- Logo and Title -->
        <div class="flex items-center space-x-4">
          <div class="w-10 h-10 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-xl flex items-center justify-center shadow-lg">
            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
            </svg>
          </div>
          <div>
            <h1 class="text-2xl font-bold bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent">
              Multi-Tool Hub
            </h1>
            <p class="text-sm text-gray-500 dark:text-gray-400">Your all-in-one productivity suite</p>
          </div>
        </div>

        <!-- Context Switcher -->
        <div class="flex items-center space-x-4">
          <div class="relative">
            <button
              onclick={() => showContextMenu = !showContextMenu}
              class="flex items-center space-x-3 px-4 py-2 bg-white dark:bg-gray-700 rounded-xl border border-gray-200 dark:border-gray-600 shadow-sm hover:shadow-md transition-all duration-200"
            >
              <div class="text-2xl">{getCurrentUserType().icon}</div>
              <div class="text-left">
                <div class="text-sm font-medium text-gray-900 dark:text-white">{getCurrentUserType().name}</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Role</div>
              </div>
              <svg class="w-4 h-4 text-gray-400 transition-transform duration-200 {showContextMenu ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
              </svg>
            </button>

            <!-- Context Menu -->
            {#if showContextMenu}
              <div class="absolute right-0 mt-2 w-64 bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 py-2 z-50">
                {#each userTypes as userType}
                  <button
                    onclick={() => { updateUserType(userType.id); showContextMenu = false; }}
                    class="w-full flex items-center space-x-3 px-4 py-3 text-left hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-150 {selectedUserType === userType.id ? 'bg-blue-50 dark:bg-blue-900/20' : ''}"
                  >
                    <div class="text-xl">{userType.icon}</div>
                    <div class="flex-1">
                      <div class="text-sm font-medium text-gray-900 dark:text-white">{userType.name}</div>
                      <div class="text-xs text-gray-500 dark:text-gray-400">{userType.description}</div>
                    </div>
                    {#if selectedUserType === userType.id}
                      <svg class="w-4 h-4 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                      </svg>
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Theme Switcher -->
        <div class="flex items-center space-x-2">
          <ThemeSwitcher />
          
          <!-- Test button -->
          <button
            onclick={() => {
              document.documentElement.classList.toggle('dark');
              console.log('Manual toggle - Dark class:', document.documentElement.classList.contains('dark'));
            }}
            class="px-2 py-1 bg-red-500 text-white text-xs rounded"
          >
            Test
          </button>
          
          <!-- Settings Button -->
          <a href="/settings" aria-label="Settings" class="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
            </svg>
          </a>
        </div>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if isLoading}
      <div class="flex items-center justify-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
      </div>
    {:else}
      <!-- Welcome Section -->
      <div class="mb-8">
        <div class="text-center">
          <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
            Welcome to your {getCurrentUserType().name} workspace
          </h2>
          <p class="text-lg text-gray-600 dark:text-gray-400">
            {getCurrentUserType().description}
          </p>
        </div>
      </div>

      <!-- Tools Grid -->
      {#if selectedUserType && toolsByUserType[selectedUserType]}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          {#each toolsByUserType[selectedUserType] as tool, index}
            <a 
              href={tool.route}
              class="group relative overflow-hidden bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-xl transition-all duration-300 hover:scale-105"
              style="animation-delay: {index * 100}ms"
            >
              <!-- Gradient Background -->
              <div class="absolute inset-0 bg-gradient-to-br {tool.color} opacity-0 group-hover:opacity-10 transition-opacity duration-300"></div>
              
              <div class="relative p-6">
                <div class="text-4xl mb-4 group-hover:scale-110 transition-transform duration-300">
                  {tool.icon}
                </div>
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">
                  {tool.name}
                </h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
                  {tool.description}
                </p>
                <div class="flex items-center text-indigo-600 dark:text-indigo-400 text-sm font-medium group-hover:text-indigo-700 dark:group-hover:text-indigo-300 transition-colors">
                  Open Tool
                  <svg class="w-4 h-4 ml-1 group-hover:translate-x-1 transition-transform duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                  </svg>
                </div>
              </div>
            </a>
          {/each}
        </div>
      {/if}

      <!-- Quick Actions -->
      <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Quick Actions</h3>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <button class="flex items-center p-4 rounded-xl bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 hover:bg-blue-100 dark:hover:bg-blue-900/30 transition-all duration-200 group">
            <svg class="w-5 h-5 mr-3 group-hover:scale-110 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            Create New Note
          </button>
          <button class="flex items-center p-4 rounded-xl bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-300 hover:bg-green-100 dark:hover:bg-green-900/30 transition-all duration-200 group">
            <svg class="w-5 h-5 mr-3 group-hover:scale-110 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
            </svg>
            Upload Files
          </button>
          <button class="flex items-center p-4 rounded-xl bg-purple-50 dark:bg-purple-900/20 text-purple-700 dark:text-purple-300 hover:bg-purple-100 dark:hover:bg-purple-900/30 transition-all duration-200 group">
            <svg class="w-5 h-5 mr-3 group-hover:scale-110 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
            </svg>
            Settings
          </button>
        </div>
      </div>
    {/if}
  </main>
</div>

<!-- Click outside to close context menu -->
{#if showContextMenu}
  <div class="fixed inset-0 z-40" onclick={() => showContextMenu = false} onkeydown={(e) => e.key === 'Escape' && (showContextMenu = false)} role="button" tabindex="0" aria-label="Close context menu"></div>
{/if}
