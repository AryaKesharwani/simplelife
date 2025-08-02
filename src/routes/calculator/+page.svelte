<script lang="ts">
  import { onMount } from "svelte";

  let display = $state("0");
  let previousValue = $state(0);
  let operation = $state("");
  let waitingForOperand = $state(false);
  let showScientific = $state(false);
  let memory = $state(0);

  function inputDigit(digit: string) {
    if (waitingForOperand) {
      display = digit;
      waitingForOperand = false;
    } else {
      display = display === "0" ? digit : display + digit;
    }
  }

  function inputDecimal() {
    if (waitingForOperand) {
      display = "0.";
      waitingForOperand = false;
    } else if (display.indexOf(".") === -1) {
      display = display + ".";
    }
  }

  function clear() {
    display = "0";
    previousValue = 0;
    operation = "";
    waitingForOperand = false;
  }

  function performOperation(nextOperation: string) {
    const inputValue = parseFloat(display);

    if (previousValue === 0) {
      previousValue = inputValue;
    } else if (operation) {
      const currentValue = previousValue || 0;
      const newValue = calculate(currentValue, inputValue, operation);

      display = String(newValue);
      previousValue = newValue;
    }

    waitingForOperand = true;
    operation = nextOperation;
  }

  function calculate(firstValue: number, secondValue: number, operation: string): number {
    switch (operation) {
      case "+":
        return firstValue + secondValue;
      case "-":
        return firstValue - secondValue;
      case "×":
        return firstValue * secondValue;
      case "÷":
        return firstValue / secondValue;
      case "%":
        return firstValue % secondValue;
      case "^":
        return Math.pow(firstValue, secondValue);
      default:
        return secondValue;
    }
  }

  function scientificFunction(func: string) {
    const value = parseFloat(display);
    let result = 0;

    switch (func) {
      case "sin":
        result = Math.sin(value * Math.PI / 180);
        break;
      case "cos":
        result = Math.cos(value * Math.PI / 180);
        break;
      case "tan":
        result = Math.tan(value * Math.PI / 180);
        break;
      case "log":
        result = Math.log10(value);
        break;
      case "ln":
        result = Math.log(value);
        break;
      case "sqrt":
        result = Math.sqrt(value);
        break;
      case "square":
        result = value * value;
        break;
      case "cube":
        result = value * value * value;
        break;
      case "factorial":
        result = factorial(value);
        break;
      case "inverse":
        result = 1 / value;
        break;
      case "abs":
        result = Math.abs(value);
        break;
      case "exp":
        result = Math.exp(value);
        break;
    }

    display = String(result);
    waitingForOperand = true;
  }

  function factorial(n: number): number {
    if (n < 0) return NaN;
    if (n === 0 || n === 1) return 1;
    let result = 1;
    for (let i = 2; i <= n; i++) {
      result *= i;
    }
    return result;
  }

  function memoryStore() {
    memory = parseFloat(display);
  }

  function memoryRecall() {
    display = String(memory);
    waitingForOperand = true;
  }

  function memoryClear() {
    memory = 0;
  }

  function memoryAdd() {
    memory += parseFloat(display);
  }

  function memorySubtract() {
    memory -= parseFloat(display);
  }

  function toggleSign() {
    display = String(-parseFloat(display));
  }

  function percentage() {
    const value = parseFloat(display);
    display = String(value / 100);
    waitingForOperand = true;
  }

  function clearEntry() {
    display = "0";
    waitingForOperand = false;
  }

  function backspace() {
    if (display.length > 1) {
      display = display.slice(0, -1);
    } else {
      display = "0";
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
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Calculator</h1>
          <button
            onclick={() => showScientific = !showScientific}
            class="flex items-center space-x-2 px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg transition-colors duration-200"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"></path>
            </svg>
            <span>{showScientific ? 'Basic' : 'Scientific'}</span>
          </button>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden">
      <!-- Display -->
      <div class="bg-gray-900 dark:bg-gray-900 p-6">
        <div class="text-right">
          <div class="text-sm text-gray-400 mb-1">
            {operation ? `${previousValue} ${operation}` : ''}
          </div>
          <div class="text-4xl font-mono text-white break-all">
            {display}
          </div>
        </div>
      </div>

      <!-- Calculator Buttons -->
      <div class="p-6">
        <div class="grid grid-cols-4 gap-3">
          <!-- Memory Functions -->
          <button onclick={memoryStore} class="calc-btn bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600">
            MS
          </button>
          <button onclick={memoryRecall} class="calc-btn bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600">
            MR
          </button>
          <button onclick={memoryClear} class="calc-btn bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600">
            MC
          </button>
          <button onclick={memoryAdd} class="calc-btn bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600">
            M+
          </button>

          <!-- Scientific Functions (when enabled) -->
          {#if showScientific}
            <button onclick={() => scientificFunction('sin')} class="calc-btn bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-900/50">
              sin
            </button>
            <button onclick={() => scientificFunction('cos')} class="calc-btn bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-900/50">
              cos
            </button>
            <button onclick={() => scientificFunction('tan')} class="calc-btn bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-900/50">
              tan
            </button>
            <button onclick={() => scientificFunction('log')} class="calc-btn bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-900/50">
              log
            </button>

            <button onclick={() => scientificFunction('sqrt')} class="calc-btn bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-900/50">
              √
            </button>
            <button onclick={() => scientificFunction('square')} class="calc-btn bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-900/50">
              x²
            </button>
            <button onclick={() => scientificFunction('cube')} class="calc-btn bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-900/50">
              x³
            </button>
            <button onclick={() => scientificFunction('factorial')} class="calc-btn bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-900/50">
              x!
            </button>
          {/if}

          <!-- Clear and Backspace -->
          <button onclick={clear} class="calc-btn bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 hover:bg-red-200 dark:hover:bg-red-900/50">
            C
          </button>
          <button onclick={clearEntry} class="calc-btn bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 hover:bg-red-200 dark:hover:bg-red-900/50">
            CE
          </button>
          <button onclick={backspace} class="calc-btn bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 hover:bg-red-200 dark:hover:bg-red-900/50">
            ⌫
          </button>
          <button onclick={toggleSign} class="calc-btn bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600">
            ±
          </button>

          <!-- Numbers and Basic Operations -->
          <button onclick={() => inputDigit('7')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            7
          </button>
          <button onclick={() => inputDigit('8')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            8
          </button>
          <button onclick={() => inputDigit('9')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            9
          </button>
          <button onclick={() => performOperation('÷')} class="calc-btn bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300 hover:bg-orange-200 dark:hover:bg-orange-900/50">
            ÷
          </button>

          <button onclick={() => inputDigit('4')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            4
          </button>
          <button onclick={() => inputDigit('5')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            5
          </button>
          <button onclick={() => inputDigit('6')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            6
          </button>
          <button onclick={() => performOperation('×')} class="calc-btn bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300 hover:bg-orange-200 dark:hover:bg-orange-900/50">
            ×
          </button>

          <button onclick={() => inputDigit('1')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            1
          </button>
          <button onclick={() => inputDigit('2')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            2
          </button>
          <button onclick={() => inputDigit('3')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            3
          </button>
          <button onclick={() => performOperation('-')} class="calc-btn bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300 hover:bg-orange-200 dark:hover:bg-orange-900/50">
            -
          </button>

          <button onclick={() => inputDigit('0')} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700 col-span-2">
            0
          </button>
          <button onclick={inputDecimal} class="calc-btn bg-white dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-700">
            .
          </button>
          <button onclick={() => performOperation('+')} class="calc-btn bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300 hover:bg-orange-200 dark:hover:bg-orange-900/50">
            +
          </button>

          <!-- Additional Operations -->
          <button onclick={percentage} class="calc-btn bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600">
            %
          </button>
          <button onclick={() => performOperation('^')} class="calc-btn bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600">
            ^
          </button>
          <button onclick={() => performOperation('%')} class="calc-btn bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600">
            mod
          </button>
          <button onclick={() => performOperation('=')} class="calc-btn bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 hover:bg-green-200 dark:hover:bg-green-900/50">
            =
          </button>
        </div>
      </div>
    </div>

    <!-- Calculator History (placeholder) -->
    <div class="mt-8 bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Calculator History</h3>
      <div class="text-center text-gray-500 dark:text-gray-400 py-8">
        <div class="text-4xl mb-4">📊</div>
        <p>Your calculation history will appear here</p>
      </div>
    </div>
  </main>
</div>

<style>
  .calc-btn {
    padding: 1rem;
    padding-top: 0.75rem;
    padding-bottom: 0.75rem;
    font-size: 1.125rem;
    font-weight: 500;
    border-radius: 0.5rem;
    transition: all 0.2s;
    outline: none;
  }
  .calc-btn:focus {
    box-shadow: 0 0 0 2px #f97316, 0 0 0 4px #1f2937;
  }
  .calc-btn:focus-visible {
    box-shadow: 0 0 0 2px #f97316, 0 0 0 4px #1f2937;
  }
</style> 