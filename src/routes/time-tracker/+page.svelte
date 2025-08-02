<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface TimeEntry {
    id?: number;
    project: string;
    task: string;
    description: string;
    start_time: string;
    end_time: string;
    duration: number;
    billable: boolean;
    rate: number;
    total_amount: number;
    created_at: string;
  }

  let timeEntries = $state<TimeEntry[]>([]);
  let isLoading = $state(true);
  let showCreateModal = $state(false);
  let selectedEntry = $state<TimeEntry | null>(null);
  let searchTerm = $state("");
  let selectedProject = $state("all");
  let isTracking = $state(false);
  let currentEntry = $state<TimeEntry | null>(null);

  let newEntry = $state<TimeEntry>({
    project: "",
    task: "",
    description: "",
    start_time: "",
    end_time: "",
    duration: 0,
    billable: true,
    rate: 50,
    total_amount: 0,
    created_at: ""
  });

  const projects = [
    { id: "all", name: "All Projects", color: "gray" },
    { id: "web-development", name: "Web Development", color: "blue" },
    { id: "mobile-app", name: "Mobile App", color: "green" },
    { id: "design", name: "Design", color: "purple" },
    { id: "consulting", name: "Consulting", color: "orange" }
  ];

  onMount(async () => {
    await loadTimeEntries();
  });

  async function loadTimeEntries() {
    try {
      isLoading = true;
      timeEntries = [
        {
          id: 1,
          project: "web-development",
          task: "Frontend Development",
          description: "Working on React components",
          start_time: "2024-01-15T09:00:00",
          end_time: "2024-01-15T12:00:00",
          duration: 180,
          billable: true,
          rate: 50,
          total_amount: 150,
          created_at: "2024-01-15T09:00:00"
        }
      ];
    } catch (error) {
      console.error('Error loading time entries:', error);
    } finally {
      isLoading = false;
    }
  }

  async function saveTimeEntry() {
    try {
      await invoke('save_note', { 
        note: { 
          title: `${newEntry.project} - ${newEntry.task}`,
          content: newEntry.description,
          category: 'time-entry',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString()
        } 
      });
      
      await loadTimeEntries();
      closeModal();
    } catch (error) {
      console.error('Error saving time entry:', error);
    }
  }

  function startTracking() {
    isTracking = true;
    currentEntry = {
      project: "",
      task: "",
      description: "",
      start_time: new Date().toISOString(),
      end_time: "",
      duration: 0,
      billable: true,
      rate: 50,
      total_amount: 0,
      created_at: new Date().toISOString()
    };
  }

  function stopTracking() {
    isTracking = false;
    if (currentEntry) {
      newEntry = { ...currentEntry };
      showCreateModal = true;
    }
    currentEntry = null;
  }

  function openCreateModal() {
    selectedEntry = null;
    newEntry = {
      project: "",
      task: "",
      description: "",
      start_time: "",
      end_time: "",
      duration: 0,
      billable: true,
      rate: 50,
      total_amount: 0,
      created_at: ""
    };
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    selectedEntry = null;
  }

  function getFilteredEntries() {
    let filtered = timeEntries;
    
    if (searchTerm) {
      filtered = filtered.filter(entry => 
        entry.project.toLowerCase().includes(searchTerm.toLowerCase()) ||
        entry.task.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    if (selectedProject !== "all") {
      filtered = filtered.filter(entry => entry.project === selectedProject);
    }
    
    return filtered;
  }

  function formatDuration(minutes: number): string {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    return `${hours}h ${mins}m`;
  }

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD'
    }).format(amount);
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  function getProjectColor(project: string) {
    const projectObj = projects.find(p => p.id === project);
    switch (projectObj?.color) {
      case 'blue': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400';
      case 'green': return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400';
      case 'purple': return 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-400';
      case 'orange': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-400';
      default: return 'bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400';
    }
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
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Time Tracker</h1>
          <button
            onclick={openCreateModal}
            class="flex items-center space-x-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>Add Entry</span>
          </button>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if isLoading}
      <div class="flex items-center justify-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    {:else}
      <!-- Timer Section -->
      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6 mb-8">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Timer</h2>
          <div class="flex items-center space-x-2">
            {#if isTracking}
              <div class="text-2xl font-mono text-green-600 dark:text-green-400">
                {currentEntry ? formatDuration(currentEntry.duration) : '00h 00m'}
              </div>
              <button
                onclick={stopTracking}
                class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors duration-200"
              >
                Stop
              </button>
            {:else}
              <button
                onclick={startTracking}
                class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors duration-200"
              >
                Start Timer
              </button>
            {/if}
          </div>
        </div>
        
        {#if currentEntry}
          <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Project</label>
                <input
                  type="text"
                  bind:value={currentEntry.project}
                  placeholder="Enter project name..."
                  class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white text-sm"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Task</label>
                <input
                  type="text"
                  bind:value={currentEntry.task}
                  placeholder="Enter task description..."
                  class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white text-sm"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Rate ($/hr)</label>
                <input
                  type="number"
                  bind:value={currentEntry.rate}
                  min="0"
                  step="0.01"
                  class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white text-sm"
                />
              </div>
            </div>
          </div>
        {/if}
      </div>

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
                placeholder="Search time entries..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
          <div class="flex space-x-2">
            {#each projects as project}
              <button
                onclick={() => selectedProject = project.id}
                class="px-4 py-2 rounded-lg transition-all duration-200 {selectedProject === project.id 
                  ? 'bg-blue-600 text-white shadow-md' 
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              >
                {project.name}
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Time Entries List -->
      <div class="space-y-4">
        {#each getFilteredEntries() as entry, index}
          <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group">
            <div class="p-6">
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-2">
                    <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{entry.task}</h3>
                    <span class="px-2 py-1 text-xs font-medium rounded-full {getProjectColor(entry.project)}">
                      {projects.find(p => p.id === entry.project)?.name || entry.project}
                    </span>
                    {#if entry.billable}
                      <span class="px-2 py-1 text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400 rounded-full">
                        Billable
                      </span>
                    {/if}
                  </div>
                  
                  <p class="text-gray-600 dark:text-gray-400 text-sm mb-3">
                    {entry.description}
                  </p>
                  
                  <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                    <div>
                      <span class="text-gray-500 dark:text-gray-400">Duration:</span>
                      <span class="ml-2 font-medium">{formatDuration(entry.duration)}</span>
                    </div>
                    <div>
                      <span class="text-gray-500 dark:text-gray-400">Rate:</span>
                      <span class="ml-2 font-medium">{formatCurrency(entry.rate)}/hr</span>
                    </div>
                    <div>
                      <span class="text-gray-500 dark:text-gray-400">Amount:</span>
                      <span class="ml-2 font-medium">{formatCurrency(entry.total_amount)}</span>
                    </div>
                    <div>
                      <span class="text-gray-500 dark:text-gray-400">Date:</span>
                      <span class="ml-2 font-medium">{formatDate(entry.created_at)}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if getFilteredEntries().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">⏱️</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm || selectedProject !== "all" ? "No time entries found" : "No time entries yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm || selectedProject !== "all" 
              ? "Try adjusting your search or filters" 
              : "Start tracking your time to get started"}
          </p>
          {#if !searchTerm && selectedProject === "all"}
            <button
              onclick={startTracking}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Start Tracking</span>
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
          {selectedEntry ? 'Edit Time Entry' : 'Add Time Entry'}
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
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label for="project" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Project</label>
            <select
              id="project"
              bind:value={newEntry.project}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            >
              <option value="">Select Project</option>
              {#each projects.slice(1) as project}
                <option value={project.id}>{project.name}</option>
              {/each}
            </select>
          </div>
          <div>
            <label for="task" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Task</label>
            <input
              id="task"
              type="text"
              bind:value={newEntry.task}
              placeholder="Enter task description..."
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
        </div>
        
        <div>
          <label for="description" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Description</label>
          <textarea
            id="description"
            bind:value={newEntry.description}
            placeholder="Enter detailed description..."
            rows="3"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white resize-none"
          ></textarea>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label for="start_time" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Start Time</label>
            <input
              id="start_time"
              type="datetime-local"
              bind:value={newEntry.start_time}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label for="end_time" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">End Time</label>
            <input
              id="end_time"
              type="datetime-local"
              bind:value={newEntry.end_time}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label for="rate" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Rate ($/hr)</label>
            <input
              id="rate"
              type="number"
              bind:value={newEntry.rate}
              min="0"
              step="0.01"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
        </div>
        
        <div class="flex items-center">
          <input
            id="billable"
            type="checkbox"
            bind:checked={newEntry.billable}
            class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
          />
          <label for="billable" class="ml-2 text-sm text-gray-700 dark:text-gray-300">
            Billable
          </label>
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
          onclick={saveTimeEntry}
          disabled={!newEntry.project.trim() || !newEntry.task.trim()}
          class="px-6 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
        >
          {selectedEntry ? 'Update' : 'Save'} Entry
        </button>
      </div>
    </div>
  </div>
{/if} 