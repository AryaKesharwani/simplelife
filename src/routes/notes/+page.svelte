<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Note {
    id?: number;
    title: string;
    content: string;
    category: string;
    created_at: string;
    updated_at: string;
  }

  let notes = $state<Note[]>([]);
  let isLoading = $state(true);
  let showCreateModal = $state(false);
  let selectedNote = $state<Note | null>(null);
  let searchTerm = $state("");
  let selectedCategory = $state("all");

  let newNote = $state<Note>({
    title: "",
    content: "",
    category: "general",
    created_at: "",
    updated_at: ""
  });

  const categories = [
    { id: "all", name: "All Notes", icon: "📝" },
    { id: "general", name: "General", icon: "📄" },
    { id: "work", name: "Work", icon: "💼" },
    { id: "personal", name: "Personal", icon: "👤" },
    { id: "ideas", name: "Ideas", icon: "💡" },
    { id: "todo", name: "Todo", icon: "✅" }
  ];

  onMount(async () => {
    await loadNotes();
  });

  async function loadNotes() {
    try {
      isLoading = true;
      const result = await invoke('get_notes') as Note[];
      notes = result;
    } catch (error) {
      console.error('Error loading notes:', error);
    } finally {
      isLoading = false;
    }
  }

  async function saveNote() {
    try {
      if (selectedNote?.id) {
        // Update existing note
        await invoke('save_note', { note: { ...newNote, id: selectedNote.id } });
      } else {
        // Create new note
        await invoke('save_note', { note: newNote });
      }
      
      await loadNotes();
      closeModal();
    } catch (error) {
      console.error('Error saving note:', error);
    }
  }

  async function deleteNote(id: number) {
    if (confirm('Are you sure you want to delete this note?')) {
      try {
        await invoke('delete_note', { id });
        await loadNotes();
      } catch (error) {
        console.error('Error deleting note:', error);
      }
    }
  }

  function openCreateModal() {
    selectedNote = null;
    newNote = {
      title: "",
      content: "",
      category: "general",
      created_at: "",
      updated_at: ""
    };
    showCreateModal = true;
  }

  function openEditModal(note: Note) {
    selectedNote = note;
    newNote = { ...note };
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    selectedNote = null;
  }

  function getFilteredNotes() {
    let filtered = notes;
    
    if (searchTerm) {
      filtered = filtered.filter(note => 
        note.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
        note.content.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    if (selectedCategory !== "all") {
      filtered = filtered.filter(note => note.category === selectedCategory);
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

  function getCategoryIcon(category: string) {
    const cat = categories.find(c => c.id === category);
    return cat?.icon || "📄";
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
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Notes</h1>
          <button
            onclick={openCreateModal}
            class="flex items-center space-x-2 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>New Note</span>
          </button>
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
                placeholder="Search notes..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
          <div class="flex space-x-2">
            {#each categories as category}
              <button
                onclick={() => selectedCategory = category.id}
                class="px-4 py-2 rounded-lg transition-all duration-200 {selectedCategory === category.id 
                  ? 'bg-indigo-600 text-white shadow-md' 
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              >
                <span class="mr-2">{category.icon}</span>
                {category.name}
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Notes Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each getFilteredNotes() as note, index}
          <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group">
            <div class="p-6">
              <div class="flex items-start justify-between mb-4">
                <div class="flex items-center space-x-2">
                  <span class="text-2xl">{getCategoryIcon(note.category)}</span>
                  <span class="px-2 py-1 text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 rounded-full">
                    {note.category}
                  </span>
                </div>
                <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                  <button
                    onclick={() => openEditModal(note)}
                    class="p-1 text-gray-400 hover:text-indigo-600 transition-colors"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                  </button>
                  <button
                    onclick={() => deleteNote(note.id!)}
                    class="p-1 text-gray-400 hover:text-red-600 transition-colors"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                    </svg>
                  </button>
                </div>
              </div>
              
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2 line-clamp-2">
                {note.title}
              </h3>
              
              <p class="text-gray-600 dark:text-gray-400 text-sm mb-4 line-clamp-3">
                {note.content}
              </p>
              
              <div class="text-xs text-gray-500 dark:text-gray-400">
                Updated {formatDate(note.updated_at)}
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if getFilteredNotes().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">📝</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm || selectedCategory !== "all" ? "No notes found" : "No notes yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm || selectedCategory !== "all" 
              ? "Try adjusting your search or filters" 
              : "Create your first note to get started"}
          </p>
          {#if !searchTerm && selectedCategory === "all"}
            <button
              onclick={openCreateModal}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Create First Note</span>
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
          {selectedNote ? 'Edit Note' : 'Create New Note'}
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
            bind:value={newNote.title}
            placeholder="Enter note title..."
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Category</label>
          <select
            bind:value={newNote.category}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white"
          >
            {#each categories.slice(1) as category}
              <option value={category.id}>{category.icon} {category.name}</option>
            {/each}
          </select>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Content</label>
          <textarea
            bind:value={newNote.content}
            placeholder="Write your note here..."
            rows="8"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white resize-none"
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
          onclick={saveNote}
          disabled={!newNote.title.trim()}
          class="px-6 py-2 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
        >
          {selectedNote ? 'Update' : 'Create'} Note
        </button>
      </div>
    </div>
  </div>
{/if} 