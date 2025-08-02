<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface InvoiceItem {
    description: string;
    quantity: number;
    unit_price: number;
    total: number;
  }

  interface Invoice {
    id?: number;
    invoice_number: string;
    client_name: string;
    client_email: string;
    items: InvoiceItem[];
    subtotal: number;
    tax_rate: number;
    tax_amount: number;
    total: number;
    status: string;
    due_date: string;
    created_at: string;
  }

  let invoices = $state<Invoice[]>([]);
  let isLoading = $state(true);
  let showCreateModal = $state(false);
  let selectedInvoice = $state<Invoice | null>(null);
  let searchTerm = $state("");
  let selectedStatus = $state("all");

  let newInvoice = $state<Invoice>({
    invoice_number: "",
    client_name: "",
    client_email: "",
    items: [{ description: "", quantity: 1, unit_price: 0, total: 0 }],
    subtotal: 0,
    tax_rate: 0,
    tax_amount: 0,
    total: 0,
    status: "draft",
    due_date: "",
    created_at: ""
  });

  const statuses = [
    { id: "all", name: "All Invoices", color: "gray" },
    { id: "draft", name: "Draft", color: "yellow" },
    { id: "sent", name: "Sent", color: "blue" },
    { id: "paid", name: "Paid", color: "green" },
    { id: "overdue", name: "Overdue", color: "red" }
  ];

  onMount(async () => {
    await loadInvoices();
  });

  async function loadInvoices() {
    try {
      isLoading = true;
      const result = await invoke('get_invoices') as Invoice[];
      invoices = result;
    } catch (error) {
      console.error('Error loading invoices:', error);
    } finally {
      isLoading = false;
    }
  }

  async function saveInvoice() {
    try {
      calculateTotals();
      
      if (selectedInvoice?.id) {
        await invoke('save_invoice', { invoice: { ...newInvoice, id: selectedInvoice.id } });
      } else {
        await invoke('save_invoice', { invoice: newInvoice });
      }
      
      await loadInvoices();
      closeModal();
    } catch (error) {
      console.error('Error saving invoice:', error);
    }
  }

  function calculateTotals() {
    newInvoice.subtotal = newInvoice.items.reduce((sum, item) => sum + (item.quantity * item.unit_price), 0);
    newInvoice.tax_amount = (newInvoice.subtotal * newInvoice.tax_rate) / 100;
    newInvoice.total = newInvoice.subtotal + newInvoice.tax_amount;
  }

  function addItem() {
    newInvoice.items = [...newInvoice.items, { description: "", quantity: 1, unit_price: 0, total: 0 }];
  }

  function removeItem(index: number) {
    newInvoice.items = newInvoice.items.filter((_, i) => i !== index);
    if (newInvoice.items.length === 0) {
      addItem();
    }
  }

  function updateItem(index: number, field: keyof InvoiceItem, value: any) {
    newInvoice.items[index] = { ...newInvoice.items[index], [field]: value };
    if (field === 'quantity' || field === 'unit_price') {
      newInvoice.items[index].total = newInvoice.items[index].quantity * newInvoice.items[index].unit_price;
    }
  }

  function openCreateModal() {
    selectedInvoice = null;
    newInvoice = {
      invoice_number: generateInvoiceNumber(),
      client_name: "",
      client_email: "",
      items: [{ description: "", quantity: 1, unit_price: 0, total: 0 }],
      subtotal: 0,
      tax_rate: 0,
      tax_amount: 0,
      total: 0,
      status: "draft",
      due_date: "",
      created_at: ""
    };
    showCreateModal = true;
  }

  function openEditModal(invoice: Invoice) {
    selectedInvoice = invoice;
    newInvoice = { ...invoice };
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    selectedInvoice = null;
  }

  function generateInvoiceNumber() {
    const date = new Date();
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const random = Math.floor(Math.random() * 1000).toString().padStart(3, '0');
    return `INV-${year}${month}${day}-${random}`;
  }

  function getFilteredInvoices() {
    let filtered = invoices;
    
    if (searchTerm) {
      filtered = filtered.filter(invoice => 
        invoice.invoice_number.toLowerCase().includes(searchTerm.toLowerCase()) ||
        invoice.client_name.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    if (selectedStatus !== "all") {
      filtered = filtered.filter(invoice => invoice.status === selectedStatus);
    }
    
    return filtered;
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  }

  function formatCurrency(amount: number) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD'
    }).format(amount);
  }

  function getStatusColor(status: string) {
    const statusObj = statuses.find(s => s.id === status);
    switch (statusObj?.color) {
      case 'green': return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400';
      case 'yellow': return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400';
      case 'blue': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400';
      case 'red': return 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400';
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
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Invoices</h1>
          <button
            onclick={openCreateModal}
            class="flex items-center space-x-2 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>New Invoice</span>
          </button>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if isLoading}
      <div class="flex items-center justify-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-green-600"></div>
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
                placeholder="Search invoices..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
              />
            </div>
          </div>
          <div class="flex space-x-2">
            {#each statuses as status}
              <button
                onclick={() => selectedStatus = status.id}
                class="px-4 py-2 rounded-lg transition-all duration-200 {selectedStatus === status.id 
                  ? 'bg-green-600 text-white shadow-md' 
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              >
                {status.name}
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Invoices Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each getFilteredInvoices() as invoice, index}
          <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group">
            <div class="p-6">
              <div class="flex items-start justify-between mb-4">
                <div>
                  <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-1">
                    {invoice.invoice_number}
                  </h3>
                  <p class="text-sm text-gray-600 dark:text-gray-400">
                    {invoice.client_name}
                  </p>
                </div>
                <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                  <button
                    onclick={() => openEditModal(invoice)}
                    class="p-1 text-gray-400 hover:text-green-600 transition-colors"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                  </button>
                </div>
              </div>
              
              <div class="space-y-2 mb-4">
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600 dark:text-gray-400">Subtotal:</span>
                  <span class="font-medium">{formatCurrency(invoice.subtotal)}</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600 dark:text-gray-400">Tax ({invoice.tax_rate}%):</span>
                  <span class="font-medium">{formatCurrency(invoice.tax_amount)}</span>
                </div>
                <div class="flex justify-between text-lg font-semibold border-t pt-2">
                  <span>Total:</span>
                  <span class="text-green-600 dark:text-green-400">{formatCurrency(invoice.total)}</span>
                </div>
              </div>
              
              <div class="flex items-center justify-between">
                <span class="px-3 py-1 text-xs font-medium rounded-full {getStatusColor(invoice.status)}">
                  {invoice.status.charAt(0).toUpperCase() + invoice.status.slice(1)}
                </span>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  {formatDate(invoice.created_at)}
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if getFilteredInvoices().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">💰</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm || selectedStatus !== "all" ? "No invoices found" : "No invoices yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm || selectedStatus !== "all" 
              ? "Try adjusting your search or filters" 
              : "Create your first invoice to get started"}
          </p>
          {#if !searchTerm && selectedStatus === "all"}
            <button
              onclick={openCreateModal}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Create First Invoice</span>
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
          {selectedInvoice ? 'Edit Invoice' : 'Create New Invoice'}
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
      
      <div class="p-6 space-y-6">
        <!-- Invoice Details -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Invoice Number</label>
            <input
              type="text"
              bind:value={newInvoice.invoice_number}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Status</label>
            <select
              bind:value={newInvoice.status}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
            >
              <option value="draft">Draft</option>
              <option value="sent">Sent</option>
              <option value="paid">Paid</option>
              <option value="overdue">Overdue</option>
            </select>
          </div>
        </div>

        <!-- Client Information -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Client Name</label>
            <input
              type="text"
              bind:value={newInvoice.client_name}
              placeholder="Enter client name..."
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Client Email</label>
            <input
              type="email"
              bind:value={newInvoice.client_email}
              placeholder="Enter client email..."
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
        </div>

        <!-- Due Date -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Due Date</label>
          <input
            type="date"
            bind:value={newInvoice.due_date}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
          />
        </div>

        <!-- Items -->
        <div>
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-medium text-gray-900 dark:text-white">Invoice Items</h3>
            <button
              onclick={addItem}
              class="flex items-center space-x-1 px-3 py-1 text-sm bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Add Item</span>
            </button>
          </div>
          
          <div class="space-y-3">
            {#each newInvoice.items as item, index}
              <div class="grid grid-cols-12 gap-2 items-center">
                <div class="col-span-6">
                                     <input
                     type="text"
                     bind:value={item.description}
                     oninput={() => updateItem(index, 'description', item.description)}
                     placeholder="Item description..."
                     class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white text-sm"
                   />
                 </div>
                 <div class="col-span-2">
                   <input
                     type="number"
                     bind:value={item.quantity}
                     oninput={() => updateItem(index, 'quantity', Number(item.quantity))}
                     min="1"
                     class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white text-sm"
                   />
                 </div>
                 <div class="col-span-2">
                   <input
                     type="number"
                     bind:value={item.unit_price}
                     oninput={() => updateItem(index, 'unit_price', Number(item.unit_price))}
                    step="0.01"
                    min="0"
                    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white text-sm"
                  />
                </div>
                <div class="col-span-1 text-sm font-medium text-gray-900 dark:text-white">
                  {formatCurrency(item.total)}
                </div>
                <div class="col-span-1">
                  <button
                    onclick={() => removeItem(index)}
                    class="p-1 text-red-600 hover:text-red-800 transition-colors"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                    </svg>
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Tax Rate -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Tax Rate (%)</label>
          <input
            type="number"
            bind:value={newInvoice.tax_rate}
            step="0.01"
            min="0"
            max="100"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
          />
        </div>

        <!-- Totals -->
        <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-4 space-y-2">
          <div class="flex justify-between text-sm">
            <span class="text-gray-600 dark:text-gray-400">Subtotal:</span>
            <span class="font-medium">{formatCurrency(newInvoice.subtotal)}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-600 dark:text-gray-400">Tax ({newInvoice.tax_rate}%):</span>
            <span class="font-medium">{formatCurrency(newInvoice.tax_amount)}</span>
          </div>
          <div class="flex justify-between text-lg font-semibold border-t pt-2">
            <span>Total:</span>
            <span class="text-green-600 dark:text-green-400">{formatCurrency(newInvoice.total)}</span>
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
          onclick={saveInvoice}
          disabled={!newInvoice.invoice_number.trim() || !newInvoice.client_name.trim()}
          class="px-6 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
        >
          {selectedInvoice ? 'Update' : 'Create'} Invoice
        </button>
      </div>
    </div>
  </div>
{/if} 