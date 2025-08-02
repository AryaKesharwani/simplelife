<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Client {
    id?: number;
    name: string;
    email: string;
    phone: string;
    company: string;
    status: string;
    total_projects: number;
    total_revenue: number;
    notes: string;
    created_at: string;
    updated_at: string;
  }

  let clients = $state<Client[]>([]);
  let isLoading = $state(true);
  let showCreateModal = $state(false);
  let selectedClient = $state<Client | null>(null);
  let searchTerm = $state("");
  let selectedStatus = $state("all");

  let newClient = $state<Client>({
    name: "",
    email: "",
    phone: "",
    company: "",
    status: "active",
    total_projects: 0,
    total_revenue: 0,
    notes: "",
    created_at: "",
    updated_at: ""
  });

  const statuses = [
    { id: "all", name: "All Status", color: "gray" },
    { id: "active", name: "Active", color: "green" },
    { id: "inactive", name: "Inactive", color: "red" },
    { id: "prospect", name: "Prospect", color: "yellow" },
    { id: "former", name: "Former", color: "gray" }
  ];

  onMount(async () => {
    await loadClients();
  });

  async function loadClients() {
    try {
      isLoading = true;
      clients = [
        {
          id: 1,
          name: "John Smith",
          email: "john@example.com",
          phone: "+1 (555) 123-4567",
          company: "Tech Solutions Inc",
          status: "active",
          total_projects: 5,
          total_revenue: 25000,
          notes: "Great client, always pays on time",
          created_at: "2024-01-15T09:00:00",
          updated_at: "2024-01-15T09:00:00"
        },
        {
          id: 2,
          name: "Sarah Johnson",
          email: "sarah@designstudio.com",
          phone: "+1 (555) 987-6543",
          company: "Design Studio",
          status: "active",
          total_projects: 3,
          total_revenue: 15000,
          notes: "Prefers weekly updates",
          created_at: "2024-01-10T14:30:00",
          updated_at: "2024-01-10T14:30:00"
        }
      ];
    } catch (error) {
      console.error('Error loading clients:', error);
    } finally {
      isLoading = false;
    }
  }

  async function saveClient() {
    try {
      await invoke('save_note', { 
        note: { 
          title: `${newClient.name} - ${newClient.company}`,
          content: newClient.notes,
          category: 'client',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString()
        } 
      });
      
      await loadClients();
      closeModal();
    } catch (error) {
      console.error('Error saving client:', error);
    }
  }

  async function deleteClient(id: number) {
    if (confirm('Are you sure you want to delete this client?')) {
      try {
        await invoke('delete_note', { id });
        await loadClients();
      } catch (error) {
        console.error('Error deleting client:', error);
      }
    }
  }

  function openCreateModal() {
    selectedClient = null;
    newClient = {
      name: "",
      email: "",
      phone: "",
      company: "",
      status: "active",
      total_projects: 0,
      total_revenue: 0,
      notes: "",
      created_at: "",
      updated_at: ""
    };
    showCreateModal = true;
  }

  function openEditModal(client: Client) {
    selectedClient = client;
    newClient = { ...client };
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    selectedClient = null;
  }

  function getFilteredClients() {
    let filtered = clients;
    
    if (searchTerm) {
      filtered = filtered.filter(client => 
        client.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        client.email.toLowerCase().includes(searchTerm.toLowerCase()) ||
        client.company.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    if (selectedStatus !== "all") {
      filtered = filtered.filter(client => client.status === selectedStatus);
    }
    
    return filtered;
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
      day: 'numeric'
    });
  }

  function getStatusColor(status: string) {
    const statusObj = statuses.find(s => s.id === status);
    switch (statusObj?.color) {
      case 'green': return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400';
      case 'red': return 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400';
      case 'yellow': return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400';
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
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Client Manager</h1>
          <button
            onclick={openCreateModal}
            class="flex items-center space-x-2 px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>Add Client</span>
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
                placeholder="Search clients..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
          <div class="flex space-x-2">
            {#each statuses as status}
              <button
                onclick={() => selectedStatus = status.id}
                class="px-4 py-2 rounded-lg transition-all duration-200 {selectedStatus === status.id 
                  ? 'bg-purple-600 text-white shadow-md' 
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              >
                {status.name}
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Clients Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each getFilteredClients() as client, index}
          <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group">
            <div class="p-6">
              <div class="flex items-start justify-between mb-4">
                <div class="flex-1">
                  <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-1">
                    {client.name}
                  </h3>
                  <p class="text-sm text-gray-600 dark:text-gray-400 mb-2">
                    {client.company}
                  </p>
                  <span class="px-2 py-1 text-xs font-medium rounded-full {getStatusColor(client.status)}">
                    {client.status.charAt(0).toUpperCase() + client.status.slice(1)}
                  </span>
                </div>
                
                <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                  <button
                    onclick={() => openEditModal(client)}
                    class="p-1 text-gray-400 hover:text-purple-600 transition-colors"
                    title="Edit client"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                  </button>
                  <button
                    onclick={() => deleteClient(client.id!)}
                    class="p-1 text-gray-400 hover:text-red-600 transition-colors"
                    title="Delete client"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                    </svg>
                  </button>
                </div>
              </div>
              
              <div class="space-y-3 text-sm">
                <div class="flex items-center space-x-2">
                  <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
                  </svg>
                  <span class="text-gray-600 dark:text-gray-400">{client.email}</span>
                </div>
                
                <div class="flex items-center space-x-2">
                  <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"></path>
                  </svg>
                  <span class="text-gray-600 dark:text-gray-400">{client.phone}</span>
                </div>
                
                <div class="flex items-center justify-between pt-3 border-t border-gray-200 dark:border-gray-700">
                  <div>
                    <span class="text-gray-500 dark:text-gray-400">Projects:</span>
                    <span class="ml-1 font-medium">{client.total_projects}</span>
                  </div>
                  <div>
                    <span class="text-gray-500 dark:text-gray-400">Revenue:</span>
                    <span class="ml-1 font-medium text-green-600 dark:text-green-400">{formatCurrency(client.total_revenue)}</span>
                  </div>
                </div>
                
                {#if client.notes}
                  <div class="pt-3 border-t border-gray-200 dark:border-gray-700">
                    <p class="text-xs text-gray-500 dark:text-gray-400 line-clamp-2">
                      {client.notes}
                    </p>
                  </div>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if getFilteredClients().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">👥</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm || selectedStatus !== "all" ? "No clients found" : "No clients yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm || selectedStatus !== "all" 
              ? "Try adjusting your search or filters" 
              : "Add your first client to get started"}
          </p>
          {#if !searchTerm && selectedStatus === "all"}
            <button
              onclick={openCreateModal}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Add First Client</span>
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
          {selectedClient ? 'Edit Client' : 'Add New Client'}
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
            <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Name</label>
            <input
              id="name"
              type="text"
              bind:value={newClient.name}
              placeholder="Enter client name..."
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label for="company" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Company</label>
            <input
              id="company"
              type="text"
              bind:value={newClient.company}
              placeholder="Enter company name..."
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Email</label>
            <input
              id="email"
              type="email"
              bind:value={newClient.email}
              placeholder="Enter email address..."
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label for="phone" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Phone</label>
            <input
              id="phone"
              type="tel"
              bind:value={newClient.phone}
              placeholder="Enter phone number..."
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
        </div>
        
        <div>
          <label for="status" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Status</label>
          <select
            id="status"
            bind:value={newClient.status}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="active">Active</option>
            <option value="inactive">Inactive</option>
            <option value="prospect">Prospect</option>
            <option value="former">Former</option>
          </select>
        </div>
        
        <div>
          <label for="notes" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Notes</label>
          <textarea
            id="notes"
            bind:value={newClient.notes}
            placeholder="Enter client notes..."
            rows="3"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white resize-none"
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
          onclick={saveClient}
          disabled={!newClient.name.trim() || !newClient.email.trim()}
          class="px-6 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
        >
          {selectedClient ? 'Update' : 'Save'} Client
        </button>
      </div>
    </div>
  </div>
{/if} 