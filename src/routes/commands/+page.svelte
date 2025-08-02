<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface CommandTemplate {
    id?: number;
    name: string;
    command: string;
    description: string;
    category: string;
    tags: string[];
    created_at: string;
    updated_at: string;
  }

  let commands = $state<CommandTemplate[]>([]);
  let isLoading = $state(true);
  let showCreateModal = $state(false);
  let selectedCommand = $state<CommandTemplate | null>(null);
  let searchTerm = $state("");
  let selectedCategory = $state("all");

  let newCommand = $state<CommandTemplate>({
    name: "",
    command: "",
    description: "",
    category: "general",
    tags: [],
    created_at: "",
    updated_at: ""
  });

  const categories = [
    { id: "all", name: "All Categories", color: "gray" },
    { id: "git", name: "Git", color: "green" },
    { id: "docker", name: "Docker", color: "blue" },
    { id: "npm", name: "NPM", color: "red" },
    { id: "system", name: "System", color: "purple" },
    { id: "database", name: "Database", color: "orange" },
    { id: "general", name: "General", color: "gray" }
  ];

  onMount(async () => {
    await loadCommands();
  });

  async function loadCommands() {
    try {
      isLoading = true;
      commands = [
        {
          id: 1,
          name: "Git Status",
          command: "git status",
          description: "Check the status of your git repository",
          category: "git",
          tags: ["git", "status", "repository"],
          created_at: "2024-01-15T09:00:00",
          updated_at: "2024-01-15T09:00:00"
        },
        {
          id: 2,
          name: "Docker Cleanup",
          command: "docker system prune -a",
          description: "Remove all unused containers, networks, and images",
          category: "docker",
          tags: ["docker", "cleanup", "prune"],
          created_at: "2024-01-10T14:30:00",
          updated_at: "2024-01-10T14:30:00"
        },
        {
          id: 3,
          name: "Install Dependencies",
          command: "npm install",
          description: "Install project dependencies",
          category: "npm",
          tags: ["npm", "install", "dependencies"],
          created_at: "2024-01-05T11:20:00",
          updated_at: "2024-01-05T11:20:00"
        }
      ];
    } catch (error) {
      console.error('Error loading commands:', error);
    } finally {
      isLoading = false;
    }
  }

  async function saveCommand() {
    try {
      await invoke('save_command_template', { 
        template: { 
          name: newCommand.name,
          command: newCommand.command,
          description: newCommand.description,
          category: newCommand.category,
          tags: newCommand.tags.join(','),
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString()
        } 
      });
      
      await loadCommands();
      closeModal();
    } catch (error) {
      console.error('Error saving command:', error);
    }
  }

  async function deleteCommand(id: number) {
    if (confirm('Are you sure you want to delete this command?')) {
      try {
        await invoke('delete_note', { id });
        await loadCommands();
      } catch (error) {
        console.error('Error deleting command:', error);
      }
    }
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).then(() => {
      // Show a brief success message
      console.log('Command copied to clipboard');
    });
  }

  function openCreateModal() {
    selectedCommand = null;
    newCommand = {
      name: "",
      command: "",
      description: "",
      category: "general",
      tags: [],
      created_at: "",
      updated_at: ""
    };
    showCreateModal = true;
  }

  function openEditModal(command: CommandTemplate) {
    selectedCommand = command;
    newCommand = { ...command };
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    selectedCommand = null;
  }

  function getFilteredCommands() {
    let filtered = commands;
    
    if (searchTerm) {
      filtered = filtered.filter(cmd => 
        cmd.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        cmd.command.toLowerCase().includes(searchTerm.toLowerCase()) ||
        cmd.description.toLowerCase().includes(searchTerm.toLowerCase()) ||
        cmd.tags.some(tag => tag.toLowerCase().includes(searchTerm.toLowerCase()))
      );
    }
    
    if (selectedCategory !== "all") {
      filtered = filtered.filter(cmd => cmd.category === selectedCategory);
    }
    
    return filtered;
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  }

  function getCategoryColor(category: string) {
    const categoryObj = categories.find(c => c.id === category);
    switch (categoryObj?.color) {
      case 'green': return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400';
      case 'blue': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400';
      case 'red': return 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400';
      case 'purple': return 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400';
      case 'orange': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-400';
      default: return 'bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400';
    }
  }

  function addTag(tag: string) {
    if (tag.trim() && !newCommand.tags.includes(tag.trim())) {
      newCommand.tags = [...newCommand.tags, tag.trim()];
    }
  }

  function removeTag(tag: string) {
    newCommand.tags = newCommand.tags.filter(t => t !== tag);
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-50 dark:from-gray-950 dark:via-gray-900 dark:to-gray-950">
  <!-- Header -->
  <header class="bg-white/80 dark:bg-gray-800/80 backdrop-blur-md border-b border-gray-200 dark:border-gray-700 sticky top-0 z-40">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between py-4">
        <div class="flex items-center space-x-4">
          <a href="/" class="flex items-center space-x-2 text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
            </svg>
            <span>Back to Hub</span>
          </a>
        </div>
        <div class="flex items-center space-x-4">
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Command Templates</h1>
          <button
            onclick={openCreateModal}
            class="flex items-center space-x-2 px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>Add Command</span>
          </button>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if isLoading}
      <div class="flex items-center justify-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-orange-600"></div>
      </div>
    {:else}
      <!-- Search and Filters -->
      <div class="mb-8 space-y-4">
        <div class="flex flex-col sm:flex-row gap-4">
          <div class="flex-1">
            <div class="relative">
              <svg class="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
              </svg>
              <input
                type="text"
                bind:value={searchTerm}
                placeholder="Search commands..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
          <div class="flex space-x-2">
            {#each categories as category}
              <button
                onclick={() => selectedCategory = category.id}
                class="px-4 py-2 rounded-lg transition-all duration-200 {selectedCategory === category.id 
                  ? 'bg-orange-600 text-white shadow-md' 
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              >
                {category.name}
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Commands Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each getFilteredCommands() as command, index}
          <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group">
            <div class="p-6">
              <div class="flex items-start justify-between mb-4">
                <div class="flex-1">
                  <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-1">
                    {command.name}
                  </h3>
                  <span class="px-2 py-1 text-xs font-medium rounded-full {getCategoryColor(command.category)}">
                    {categories.find(c => c.id === command.category)?.name || command.category}
                  </span>
                </div>
                
                <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                  <button
                    onclick={() => copyToClipboard(command.command)}
                    class="p-1 text-gray-400 hover:text-orange-600 transition-colors"
                    title="Copy command"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                    </svg>
                  </button>
                  <button
                    onclick={() => openEditModal(command)}
                    class="p-1 text-gray-400 hover:text-orange-600 transition-colors"
                    title="Edit command"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                  </button>
                  <button
                    onclick={() => deleteCommand(command.id!)}
                    class="p-1 text-gray-400 hover:text-red-600 transition-colors"
                    title="Delete command"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                    </svg>
                  </button>
                </div>
              </div>
              
              <div class="space-y-3">
                <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-3">
                  <code class="text-sm text-gray-900 dark:text-white font-mono break-all">
                    {command.command}
                  </code>
                </div>
                
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  {command.description}
                </p>
                
                {#if command.tags.length > 0}
                  <div class="flex flex-wrap gap-1">
                    {#each command.tags as tag}
                      <span class="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded-full">
                        {tag}
                      </span>
                    {/each}
                  </div>
                {/if}
                
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  Created: {formatDate(command.created_at)}
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if getFilteredCommands().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">⌨️</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm || selectedCategory !== "all" ? "No commands found" : "No command templates yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm || selectedCategory !== "all" 
              ? "Try adjusting your search or filters" 
              : "Add your first command template to get started"}
          </p>
          {#if !searchTerm && selectedCategory === "all"}
            <button
              onclick={openCreateModal}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-orange-600 hover:bg-orange-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Add First Command</span>
            </button>
          {/if}
        </div>
      {/if}
    {/if}
  </main>
</div>

<!-- Create/Edit Modal -->
{#if showCreateModal}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-xl max-w-2xl w-full max-h-[90vh] overflow-hidden">
      <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {selectedCommand ? 'Edit Command' : 'Add New Command'}
        </h2>
        <button
          onclick={closeModal}
          class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
      
      <div class="p-6 space-y-4">
        <div>
          <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Name</label>
          <input
            id="name"
            type="text"
            bind:value={newCommand.name}
            placeholder="Enter command name..."
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div>
          <label for="command" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Command</label>
          <textarea
            id="command"
            bind:value={newCommand.command}
            placeholder="Enter the command..."
            rows="3"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white resize-none font-mono"
          ></textarea>
        </div>
        
        <div>
          <label for="description" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Description</label>
          <textarea
            id="description"
            bind:value={newCommand.description}
            placeholder="Enter command description..."
            rows="2"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white resize-none"
          ></textarea>
        </div>
        
        <div>
          <label for="category" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Category</label>
          <select
            id="category"
            bind:value={newCommand.category}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
          >
            {#each categories.slice(1) as category}
              <option value={category.id}>{category.name}</option>
            {/each}
          </select>
        </div>
        
        <div>
          <label for="tags" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Tags</label>
          <div class="space-y-2">
            <div class="flex flex-wrap gap-2">
              {#each newCommand.tags as tag}
                <span class="px-2 py-1 bg-orange-100 dark:bg-orange-900/30 text-orange-800 dark:text-orange-400 rounded-full text-sm flex items-center space-x-1">
                  <span>{tag}</span>
                  <button
                    onclick={() => removeTag(tag)}
                    class="text-orange-600 dark:text-orange-400 hover:text-orange-800 dark:hover:text-orange-200"
                  >
                    ×
                  </button>
                </span>
              {/each}
            </div>
            <div class="flex space-x-2">
              <input
                type="text"
                placeholder="Add tag..."
                class="flex-1 px-3 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
                onkeydown={(e) => {
                  if (e.key === 'Enter') {
                    e.preventDefault();
                    addTag(e.currentTarget.value);
                    e.currentTarget.value = '';
                  }
                }}
              />
              <button
                onclick={() => {
                  const input = document.querySelector('input[placeholder="Add tag..."]') as HTMLInputElement;
                  if (input) {
                    addTag(input.value);
                    input.value = '';
                  }
                }}
                class="px-3 py-1 text-sm bg-orange-600 hover:bg-orange-700 text-white rounded-lg transition-colors"
              >
                Add
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <div class="flex items-center justify-end space-x-3 p-6 border-t border-gray-200 dark:border-gray-700">
        <button
          onclick={closeModal}
          class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors duration-200"
        >
          Cancel
        </button>
        <button
          onclick={saveCommand}
          disabled={!newCommand.name.trim() || !newCommand.command.trim()}
          class="px-6 py-2 bg-orange-600 hover:bg-orange-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
        >
          {selectedCommand ? 'Update' : 'Save'} Command
        </button>
      </div>
    </div>
  </div>
{/if} 