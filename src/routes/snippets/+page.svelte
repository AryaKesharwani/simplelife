<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

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

  let snippets = $state<CodeSnippet[]>([]);
  let isLoading = $state(true);
  let showCreateModal = $state(false);
  let selectedSnippet = $state<CodeSnippet | null>(null);
  let searchTerm = $state("");
  let selectedLanguage = $state("all");

  let newSnippet = $state<CodeSnippet>({
    title: "",
    description: "",
    code: "",
    language: "javascript",
    tags: "",
    created_at: "",
    updated_at: ""
  });

  const languages = [
    { id: "all", name: "All Languages", icon: "💻" },
    { id: "javascript", name: "JavaScript", icon: "🟨" },
    { id: "typescript", name: "TypeScript", icon: "🔵" },
    { id: "python", name: "Python", icon: "🐍" },
    { id: "java", name: "Java", icon: "☕" },
    { id: "cpp", name: "C++", icon: "🔷" },
    { id: "csharp", name: "C#", icon: "🟪" },
    { id: "php", name: "PHP", icon: "🟦" },
    { id: "ruby", name: "Ruby", icon: "🔴" },
    { id: "go", name: "Go", icon: "🔵" },
    { id: "rust", name: "Rust", icon: "🟠" },
    { id: "sql", name: "SQL", icon: "🗄️" },
    { id: "html", name: "HTML", icon: "🌐" },
    { id: "css", name: "CSS", icon: "🎨" },
    { id: "bash", name: "Bash", icon: "💻" },
    { id: "json", name: "JSON", icon: "📄" },
    { id: "yaml", name: "YAML", icon: "📋" },
    { id: "markdown", name: "Markdown", icon: "📝" },
    { id: "text", name: "Text", icon: "📄" }
  ];

  onMount(async () => {
    await loadSnippets();
  });

  async function loadSnippets() {
    try {
      isLoading = true;
      const result = await invoke('get_code_snippets') as CodeSnippet[];
      snippets = result;
    } catch (error) {
      console.error('Error loading snippets:', error);
    } finally {
      isLoading = false;
    }
  }

  async function saveSnippet() {
    try {
      if (selectedSnippet?.id) {
        await invoke('save_code_snippet', { snippet: { ...newSnippet, id: selectedSnippet.id } });
      } else {
        await invoke('save_code_snippet', { snippet: newSnippet });
      }
      
      await loadSnippets();
      closeModal();
    } catch (error) {
      console.error('Error saving snippet:', error);
    }
  }

  function openCreateModal() {
    selectedSnippet = null;
    newSnippet = {
      title: "",
      description: "",
      code: "",
      language: "javascript",
      tags: "",
      created_at: "",
      updated_at: ""
    };
    showCreateModal = true;
  }

  function openEditModal(snippet: CodeSnippet) {
    selectedSnippet = snippet;
    newSnippet = { ...snippet };
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    selectedSnippet = null;
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).then(() => {
      // You could add a toast notification here
      console.log('Copied to clipboard');
    });
  }

  function getFilteredSnippets() {
    let filtered = snippets;
    
    if (searchTerm) {
      filtered = filtered.filter(snippet => 
        snippet.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
        snippet.description.toLowerCase().includes(searchTerm.toLowerCase()) ||
        snippet.code.toLowerCase().includes(searchTerm.toLowerCase()) ||
        snippet.tags.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    if (selectedLanguage !== "all") {
      filtered = filtered.filter(snippet => snippet.language === selectedLanguage);
    }
    
    return filtered;
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  function getLanguageIcon(language: string) {
    const lang = languages.find(l => l.id === language);
    return lang?.icon || "💻";
  }

  function getLanguageName(language: string) {
    const lang = languages.find(l => l.id === language);
    return lang?.name || "Unknown";
  }

  function getLanguageColor(language: string) {
    const colors: Record<string, string> = {
      javascript: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400",
      typescript: "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400",
      python: "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400",
      java: "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400",
      cpp: "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400",
      csharp: "bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400",
      php: "bg-indigo-100 text-indigo-800 dark:bg-indigo-900/30 dark:text-indigo-400",
      ruby: "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400",
      go: "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400",
      rust: "bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-400",
      sql: "bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400",
      html: "bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-400",
      css: "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400",
      bash: "bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400",
      json: "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400",
      yaml: "bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400",
      markdown: "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400",
      text: "bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400"
    };
    return colors[language] || "bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400";
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
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Code Snippets</h1>
          <button
            onclick={openCreateModal}
            class="flex items-center space-x-2 px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>New Snippet</span>
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
                placeholder="Search snippets..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
          <div class="flex flex-wrap gap-2">
            {#each languages.slice(0, 8) as language}
              <button
                onclick={() => selectedLanguage = language.id}
                class="px-3 py-2 rounded-lg transition-all duration-200 text-sm {selectedLanguage === language.id 
                  ? 'bg-orange-600 text-white shadow-md' 
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              >
                <span class="mr-1">{language.icon}</span>
                {language.name}
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Snippets Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each getFilteredSnippets() as snippet, index}
          <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group">
            <div class="p-6">
              <div class="flex items-start justify-between mb-4">
                <div class="flex items-center space-x-2">
                  <span class="text-2xl">{getLanguageIcon(snippet.language)}</span>
                  <span class="px-2 py-1 text-xs font-medium rounded-full {getLanguageColor(snippet.language)}">
                    {getLanguageName(snippet.language)}
                  </span>
                </div>
                <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                  <button
                    onclick={() => copyToClipboard(snippet.code)}
                    class="p-1 text-gray-400 hover:text-orange-600 transition-colors"
                    title="Copy to clipboard"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                    </svg>
                  </button>
                  <button
                    onclick={() => openEditModal(snippet)}
                    class="p-1 text-gray-400 hover:text-orange-600 transition-colors"
                    title="Edit snippet"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                  </button>
                </div>
              </div>
              
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2 line-clamp-2">
                {snippet.title}
              </h3>
              
              <p class="text-gray-600 dark:text-gray-400 text-sm mb-4 line-clamp-2">
                {snippet.description}
              </p>
              
              <!-- Code Preview -->
              <div class="bg-gray-900 dark:bg-gray-900 rounded-lg p-3 mb-4">
                <pre class="text-xs text-gray-300 overflow-hidden line-clamp-3"><code>{snippet.code}</code></pre>
              </div>
              
              {#if snippet.tags}
                <div class="flex flex-wrap gap-1 mb-4">
                  {#each snippet.tags.split(',').map(tag => tag.trim()) as tag}
                    <span class="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 rounded-full">
                      #{tag}
                    </span>
                  {/each}
                </div>
              {/if}
              
              <div class="text-xs text-gray-500 dark:text-gray-400">
                Updated {formatDate(snippet.updated_at)}
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if getFilteredSnippets().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">💻</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm || selectedLanguage !== "all" ? "No snippets found" : "No snippets yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm || selectedLanguage !== "all" 
              ? "Try adjusting your search or filters" 
              : "Create your first code snippet to get started"}
          </p>
          {#if !searchTerm && selectedLanguage === "all"}
            <button
              onclick={openCreateModal}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-orange-600 hover:bg-orange-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Create First Snippet</span>
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
    <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
      <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {selectedSnippet ? 'Edit Snippet' : 'Create New Snippet'}
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
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Title</label>
          <input
            type="text"
            bind:value={newSnippet.title}
            placeholder="Enter snippet title..."
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Description</label>
          <input
            type="text"
            bind:value={newSnippet.description}
            placeholder="Brief description of the snippet..."
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Language</label>
            <select
              bind:value={newSnippet.language}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
            >
              {#each languages.slice(1) as language}
                <option value={language.id}>{language.icon} {language.name}</option>
              {/each}
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Tags</label>
            <input
              type="text"
              bind:value={newSnippet.tags}
              placeholder="tag1, tag2, tag3..."
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Code</label>
          <textarea
            bind:value={newSnippet.code}
            placeholder="Paste your code here..."
            rows="12"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 dark:bg-gray-700 dark:text-white font-mono text-sm resize-none"
          ></textarea>
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
          onclick={saveSnippet}
          disabled={!newSnippet.title.trim() || !newSnippet.code.trim()}
          class="px-6 py-2 bg-orange-600 hover:bg-orange-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
        >
          {selectedSnippet ? 'Update' : 'Create'} Snippet
        </button>
      </div>
    </div>
  </div>
{/if} 