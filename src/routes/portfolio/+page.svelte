<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface PortfolioItem {
    id?: number;
    symbol: string;
    shares: number;
    avg_price: number;
    current_price: number;
    total_value: number;
    gain_loss: number;
    gain_loss_percent: number;
    notes: string;
  }

  let portfolio = $state<PortfolioItem[]>([]);
  let isLoading = $state(true);
  let showCreateModal = $state(false);
  let selectedItem = $state<PortfolioItem | null>(null);
  let searchTerm = $state("");

  let newItem = $state<PortfolioItem>({
    symbol: "",
    shares: 0,
    avg_price: 0,
    current_price: 0,
    total_value: 0,
    gain_loss: 0,
    gain_loss_percent: 0,
    notes: ""
  });

  let totalPortfolioValue = $state(0);
  let totalGainLoss = $state(0);
  let totalGainLossPercent = $state(0);

  onMount(async () => {
    await loadPortfolio();
  });

  async function loadPortfolio() {
    try {
      isLoading = true;
      const result = await invoke('get_portfolio') as PortfolioItem[];
      portfolio = result;
      calculatePortfolioTotals();
    } catch (error) {
      console.error('Error loading portfolio:', error);
    } finally {
      isLoading = false;
    }
  }

  async function savePortfolioItem() {
    try {
      calculateItemTotals();
      
      if (selectedItem?.id) {
        await invoke('save_portfolio_item', { item: { ...newItem, id: selectedItem.id } });
      } else {
        await invoke('save_portfolio_item', { item: newItem });
      }
      
      await loadPortfolio();
      closeModal();
    } catch (error) {
      console.error('Error saving portfolio item:', error);
    }
  }

  function calculateItemTotals() {
    newItem.total_value = newItem.shares * newItem.current_price;
    newItem.gain_loss = (newItem.current_price - newItem.avg_price) * newItem.shares;
    newItem.gain_loss_percent = newItem.avg_price > 0 ? ((newItem.current_price - newItem.avg_price) / newItem.avg_price) * 100 : 0;
  }

  function calculatePortfolioTotals() {
    totalPortfolioValue = portfolio.reduce((sum, item) => sum + item.total_value, 0);
    totalGainLoss = portfolio.reduce((sum, item) => sum + item.gain_loss, 0);
    totalGainLossPercent = totalPortfolioValue > 0 ? (totalGainLoss / (totalPortfolioValue - totalGainLoss)) * 100 : 0;
  }

  function openCreateModal() {
    selectedItem = null;
    newItem = {
      symbol: "",
      shares: 0,
      avg_price: 0,
      current_price: 0,
      total_value: 0,
      gain_loss: 0,
      gain_loss_percent: 0,
      notes: ""
    };
    showCreateModal = true;
  }

  function openEditModal(item: PortfolioItem) {
    selectedItem = item;
    newItem = { ...item };
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    selectedItem = null;
  }

  function getFilteredPortfolio() {
    if (!searchTerm) return portfolio;
    
    return portfolio.filter(item => 
      item.symbol.toLowerCase().includes(searchTerm.toLowerCase()) ||
      item.notes.toLowerCase().includes(searchTerm.toLowerCase())
    );
  }

  function formatCurrency(amount: number) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD'
    }).format(amount);
  }

  function formatNumber(num: number) {
    return new Intl.NumberFormat('en-US').format(num);
  }

  function getGainLossColor(gainLoss: number) {
    return gainLoss >= 0 ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400';
  }

  function getGainLossBgColor(gainLoss: number) {
    return gainLoss >= 0 ? 'bg-green-100 dark:bg-green-900/30' : 'bg-red-100 dark:bg-red-900/30';
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
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Portfolio</h1>
          <button
            onclick={openCreateModal}
            class="flex items-center space-x-2 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            <span>Add Position</span>
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
      <!-- Portfolio Summary -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Total Value</p>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{formatCurrency(totalPortfolioValue)}</p>
            </div>
            <div class="w-12 h-12 bg-blue-100 dark:bg-blue-900/30 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Total Gain/Loss</p>
              <p class="text-2xl font-bold {getGainLossColor(totalGainLoss)}">{formatCurrency(totalGainLoss)}</p>
            </div>
            <div class="w-12 h-12 {getGainLossBgColor(totalGainLoss)} rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 {getGainLossColor(totalGainLoss)}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path>
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Gain/Loss %</p>
              <p class="text-2xl font-bold {getGainLossColor(totalGainLossPercent)}">{totalGainLossPercent.toFixed(2)}%</p>
            </div>
            <div class="w-12 h-12 {getGainLossBgColor(totalGainLossPercent)} rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 {getGainLossColor(totalGainLossPercent)}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
              </svg>
            </div>
          </div>
        </div>
      </div>

      <!-- Search -->
      <div class="mb-8">
        <div class="relative">
          <svg class="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
          </svg>
          <input
            type="text"
            bind:value={searchTerm}
            placeholder="Search positions..."
            class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
      </div>

      <!-- Portfolio Table -->
      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead class="bg-gray-50 dark:bg-gray-700">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Symbol</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Shares</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Avg Price</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Current Price</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Total Value</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Gain/Loss</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Gain/Loss %</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Actions</th>
              </tr>
            </thead>
            <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
              {#each getFilteredPortfolio() as item, index}
                <tr class="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex items-center">
                      <div class="text-sm font-medium text-gray-900 dark:text-white">{item.symbol}</div>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {formatNumber(item.shares)}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {formatCurrency(item.avg_price)}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                    {formatCurrency(item.current_price)}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                    {formatCurrency(item.total_value)}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="text-sm font-medium {getGainLossColor(item.gain_loss)}">
                      {formatCurrency(item.gain_loss)}
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="text-sm font-medium {getGainLossColor(item.gain_loss_percent)}">
                      {item.gain_loss_percent.toFixed(2)}%
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <div class="flex items-center space-x-2">
                      <button
                        onclick={() => openEditModal(item)}
                        class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300 transition-colors"
                      >
                        Edit
                      </button>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>

      {#if getFilteredPortfolio().length === 0}
        <div class="text-center py-16">
          <div class="text-6xl mb-4">📊</div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            {searchTerm ? "No positions found" : "No positions yet"}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {searchTerm 
              ? "Try adjusting your search" 
              : "Add your first position to start tracking your portfolio"}
          </p>
          {#if !searchTerm}
            <button
              onclick={openCreateModal}
              class="inline-flex items-center space-x-2 px-6 py-3 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors duration-200"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
              </svg>
              <span>Add First Position</span>
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
          {selectedItem ? 'Edit Position' : 'Add New Position'}
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
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Symbol</label>
          <input
            type="text"
            bind:value={newItem.symbol}
            placeholder="e.g., AAPL, GOOGL, TSLA"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Number of Shares</label>
            <input
              type="number"
              bind:value={newItem.shares}
              step="0.01"
              min="0"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Average Price</label>
            <input
              type="number"
              bind:value={newItem.avg_price}
              step="0.01"
              min="0"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Current Price</label>
          <input
            type="number"
            bind:value={newItem.current_price}
            step="0.01"
            min="0"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Notes</label>
          <textarea
            bind:value={newItem.notes}
            placeholder="Add any notes about this position..."
            rows="3"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white resize-none"
          ></textarea>
        </div>

        <!-- Calculated Values -->
        <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-4 space-y-2">
          <div class="flex justify-between text-sm">
            <span class="text-gray-600 dark:text-gray-400">Total Value:</span>
            <span class="font-medium">{formatCurrency(newItem.shares * newItem.current_price)}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-600 dark:text-gray-400">Gain/Loss:</span>
            <span class="font-medium {getGainLossColor((newItem.current_price - newItem.avg_price) * newItem.shares)}">
              {formatCurrency((newItem.current_price - newItem.avg_price) * newItem.shares)}
            </span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-600 dark:text-gray-400">Gain/Loss %:</span>
            <span class="font-medium {getGainLossColor(newItem.avg_price > 0 ? ((newItem.current_price - newItem.avg_price) / newItem.avg_price) * 100 : 0)}">
              {newItem.avg_price > 0 ? (((newItem.current_price - newItem.avg_price) / newItem.avg_price) * 100).toFixed(2) : '0.00'}%
            </span>
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
          onclick={savePortfolioItem}
          disabled={!newItem.symbol.trim() || newItem.shares <= 0 || newItem.avg_price <= 0}
          class="px-6 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-400 text-white rounded-lg transition-colors duration-200"
        >
          {selectedItem ? 'Update' : 'Add'} Position
        </button>
      </div>
    </div>
  </div>
{/if} 