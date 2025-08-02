<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Reminder {
    id?: number;
    title: string;
    description: string;
    due_date: string;
    priority: string;
    completed: boolean;
    created_at: string;
    updated_at: string;
  }

  let reminders = $state<Reminder[]>([]);
  let isLoading = $state(true);
  let showCreateModal = $state(false);
  let selectedReminder = $state<Reminder | null>(null);
  let searchTerm = $state("");
  let selectedPriority = $state("all");
  let showCompleted = $state(true);

  let newReminder = $state<Reminder>({
    title: "",
    description: "",
    due_date: "",
    priority: "medium",
    completed: false,
    created_at: "",
    updated_at: ""
  });

  const priorities = [
    { id: "all", name: "All Priorities", color: "gray" },
    { id: "high", name: "High", color: "red" },
    { id: "medium", name: "Medium", color: "yellow" },
    { id: "low", name: "Low", color: "green" }
  ];

  onMount(async () => {
    await loadReminders();
  });

  async function loadReminders() {
    try {
      isLoading = true;
      // For now, we'll use the notes table as reminders
      const result = await invoke('get_notes') as any[];
      reminders = result.map(note => ({
        id: note.id,
        title: note.title,
        description: note.content,
        due_date: note.category === 'reminder' ? note.updated_at : '',
        priority: note.category === 'reminder' ? 'medium' : 'low',
        completed: false,
        created_at: note.created_at,
        updated_at: note.updated_at
      }));
    } catch (error) {
      console.error('Error loading reminders:', error);
    } finally {
      isLoading = false;
    }
  }

  async function saveReminder() {
    try {
      if (selectedReminder?.id) {
        // Update existing reminder
        await invoke('save_note', { 
          note: { 
            id: selectedReminder.id,
            title: newReminder.title,
            content: newReminder.description,
            category: 'reminder',
            created_at: selectedReminder.created_at,
            updated_at: new Date().toISOString()
          } 
        });
      } else {
        // Create new reminder
        await invoke('save_note', { 
          note: { 
            title: newReminder.title,
            content: newReminder.description,
            category: 'reminder',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString()
          } 
        });
      }
      
      await loadReminders();
      closeModal();
    } catch (error) {
      console.error('Error saving reminder:', error);
    }
  }

  async function toggleComplete(reminder: Reminder) {
    try {
      reminder.completed = !reminder.completed;
      await invoke('save_note', { 
        note: { 
          id: reminder.id,
          title: reminder.title,
          content: reminder.description,
          category: 'reminder',
          created_at: reminder.created_at,
          updated_at: new Date().toISOString()
        } 
      });
      await loadReminders();
    } catch (error) {
      console.error('Error updating reminder:', error);
    }
  }

  async function deleteReminder(id: number) {
    if (confirm('Are you sure you want to delete this reminder?')) {
      try {
        await invoke('delete_note', { id });
        await loadReminders();
      } catch (error) {
        console.error('Error deleting reminder:', error);
      }
    }
  }

  function openCreateModal() {
    selectedReminder = null;
    newReminder = {
      title: "",
      description: "",
      due_date: "",
      priority: "medium",
      completed: false,
      created_at: "",
      updated_at: ""
    };
    showCreateModal = true;
  }

  function openEditModal(reminder: Reminder) {
    selectedReminder = reminder;
    newReminder = { ...reminder };
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    selectedReminder = null;
  }

  function getFilteredReminders() {
    let filtered = reminders;
    
    if (searchTerm) {
      filtered = filtered.filter(reminder => 
        reminder.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
        reminder.description.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    if (selectedPriority !== "all") {
      filtered = filtered.filter(reminder => reminder.priority === selectedPriority);
    }
    
    if (!showCompleted) {
      filtered = filtered.filter(reminder => !reminder.completed);
    }
    
    return filtered.sort((a, b) => {
      // Sort by priority first, then by due date
      const priorityOrder = { high: 3, medium: 2, low: 1 };
      const aPriority = priorityOrder[a.priority as keyof typeof priorityOrder] || 1;
      const bPriority = priorityOrder[b.priority as keyof typeof priorityOrder] || 1;
      
      if (aPriority !== bPriority) {
        return bPriority - aPriority;
      }
      
      return new Date(a.due_date).getTime() - new Date(b.due_date).getTime();
    });
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

  function getPriorityColor(priority: string) {
    const priorityObj = priorities.find(p => p.id === priority);
    switch (priorityObj?.color) {
      case 'red': return 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400';
      case 'yellow': return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400';
      case 'green': return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400';
      default: return 'bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400';
    }
  }

  function isOverdue(dueDate: string) {
    return new Date(dueDate) < new Date();
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
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Reminders</h1>
          <button
            onclick={openCreateModal}
            class="flex items-center space-x-2 px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>New Reminder</span>
          </button>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if isLoading}
      <div class="flex items-center justify-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600"></div>
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
                placeholder="Search reminders..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
          <div class="flex space-x-2">
            {#each priorities as priority}
              <button
                onclick={() => selectedPriority = priority.id}
                class="px-4 py-2 rounded-lg transition-all duration-200 {selectedPriority === priority.id 
                  ? 'bg-purple-600 text-white shadow-md' 
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              >
                {priority.name}
              </button>
            {/each}
          </div>
        </div>
        <div class="flex items-center space-x-4">
          <label class="flex items-center">
            <input
              type="checkbox"
              bind:checked={showCompleted}
              class="h-4 w-4 text-purple-600 focus:ring-purple-500 border-gray-300 rounded"
            />
            <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Show completed</span>
          </label>
        </div>
      </div>

      <!-- Reminders List -->
      <div class="space-y-4">
        {#each getFilteredReminders() as reminder, index}
          <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group {reminder.completed ? 'opacity-60' : ''}">
            <div class="p-6">
              <div class="flex items-start justify-between">
                <div class="flex items-start space-x-4 flex-1">
                  <input
                    type="checkbox"
                    checked={reminder.completed}
                    onclick={() => toggleComplete(reminder)}
                    class="mt-1 h-4 w-4 text-purple-600 focus:ring-purple-500 border-gray-300 rounded"
                  />
                  <div class="flex-1">
                    <div class="flex items-center space-x-2 mb-2">
                      <h3 class="text-lg font-semibold text-gray-900 dark:text-white {reminder.completed ? 'line-through' : ''}">
                        {reminder.title}
                      </h3>
                      <span class="px-2 py-1 text-xs font-medium rounded-full {getPriorityColor(reminder.priority)}">
                        {reminder.priority.charAt(0).toUpperCase() + reminder.priority.slice(1)}
                      </span>
                      {#if isOverdue(reminder.due_date) && !reminder.completed}
                        <span class="px-2 py-1 text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400 rounded-full">
                          Overdue
                        </span>
                      {/if}
                    </div>
                    
                    <p class="text-gray-600 dark:text-gray-400 text-sm mb-3 {reminder.completed ? 'line-through' : ''}">
                      {reminder.description}
                    </p>
                    
                    <div class="flex items-center space-x-4 text-xs text-gray-500 dark:text-gray-400">
                      <span>Due: {formatDate(reminder.due_date)}</span>
                      <span>Created: {formatDate(reminder.created_at)}</span>
                    </div>
                  </div>
                </div>
                
                <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                  <button
                    onclick={() => openEditModal(reminder)}
                    class="p-1 text-gray-400 hover:text-purple-600 transition-colors"
                    title="Edit reminder"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                  </button>
                  <button
                    onclick={() => deleteReminder(reminder.id!)}
                    class="p-1 text-gray-400 hover:text-red-600 transition-colors"
                    title="Delete reminder"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                    </svg>
                  </button>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if getFilteredReminders().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">⏰</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm || selectedPriority !== "all" ? "No reminders found" : "No reminders yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm || selectedPriority !== "all" 
              ? "Try adjusting your search or filters" 
              : "Create your first reminder to get started"}
          </p>
          {#if !searchTerm && selectedPriority === "all"}
            <button
              onclick={openCreateModal}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Create First Reminder</span>
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
          {selectedReminder ? 'Edit Reminder' : 'Create New Reminder'}
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
          <label for="title" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Title</label>
          <input
            id="title"
            type="text"
            bind:value={newReminder.title}
            placeholder="Enter reminder title..."
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div>
          <label for="description" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Description</label>
          <textarea
            id="description"
            bind:value={newReminder.description}
            placeholder="Enter reminder description..."
            rows="3"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white resize-none"
          ></textarea>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label for="due_date" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Due Date</label>
            <input
              id="due_date"
              type="datetime-local"
              bind:value={newReminder.due_date}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label for="priority" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Priority</label>
            <select
              id="priority"
              bind:value={newReminder.priority}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
            >
              <option value="low">Low</option>
              <option value="medium">Medium</option>
              <option value="high">High</option>
            </select>
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
          onclick={saveReminder}
          disabled={!newReminder.title.trim()}
          class="px-6 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
        >
          {selectedReminder ? 'Update' : 'Create'} Reminder
        </button>
      </div>
    </div>
  </div>
{/if} 