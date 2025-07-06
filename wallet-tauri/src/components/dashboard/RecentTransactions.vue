<script setup lang="ts">
import { ref, onMounted } from "vue";
import { commands, unwrapResult } from "@/services/api";
import type { Transaction, Money } from "@/bindings";

const transactions = ref<Transaction[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);

const formatMoney = (money: Money): string => {
  const amountMinor = Number(money.amount_minor);
  const scale = money.currency.minor_unit_scale;
  const value = amountMinor / Math.pow(10, scale);
  
  return new Intl.NumberFormat('fr-FR', {
    style: 'currency',
    currency: money.currency.code,
  }).format(value);
};

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  }).format(date);
};


const fetchRecentTransactions = async () => {
  loading.value = true;
  error.value = null;
  
  try {
    const result = await commands.getRecentTransactions(10);
    transactions.value = unwrapResult(result);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    console.error("Failed to fetch recent transactions:", e);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  fetchRecentTransactions();
});
</script>

<template>
  <div class="bg-white rounded-lg border p-6">
    <div class="flex items-center justify-between mb-4">
      <h2 class="text-lg font-semibold text-gray-900">Transactions récentes</h2>
      <button
        @click="fetchRecentTransactions"
        class="text-blue-600 hover:text-blue-800 text-sm font-medium"
      >
        Actualiser
      </button>
    </div>

    <div v-if="loading" class="space-y-3">
      <div v-for="i in 5" :key="i" class="animate-pulse">
        <div class="flex items-center justify-between p-3 bg-gray-50 rounded">
          <div class="flex-1">
            <div class="h-4 bg-gray-300 rounded w-3/4 mb-2"></div>
            <div class="h-3 bg-gray-300 rounded w-1/2"></div>
          </div>
          <div class="h-4 bg-gray-300 rounded w-20"></div>
        </div>
      </div>
    </div>

    <div v-else-if="error" class="text-red-500 text-center py-4">
      Erreur: {{ error }}
    </div>

    <div v-else-if="transactions.length === 0" class="text-gray-500 text-center py-8">
      Aucune transaction trouvée
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="transaction in transactions"
        :key="transaction.id?.toString()"
        class="flex items-center justify-between p-3 border rounded hover:bg-gray-50 transition-colors"
      >
        <div class="flex-1">
          <div class="font-medium text-gray-900">{{ transaction.description }}</div>
          <div class="text-sm text-gray-500">
            {{ formatDate(transaction.transaction_date) }}
            <span v-if="transaction.entries.length > 2" class="ml-2 text-xs bg-gray-200 px-2 py-0.5 rounded">
              {{ transaction.entries.length }} entrées
            </span>
          </div>
        </div>
        
        <div class="text-right">
          <div 
            v-for="entry in transaction.entries.slice(0, 2)" 
            :key="entry.id?.toString()"
            :class="entry.entry_type === 'Debit' ? 'text-green-600' : 'text-red-600'"
            class="font-medium"
          >
            {{ entry.entry_type === 'Debit' ? '+' : '-' }}{{ formatMoney(entry.amount) }}
          </div>
          <div v-if="transaction.entries.length > 2" class="text-xs text-gray-500">
            +{{ transaction.entries.length - 2 }} autres
          </div>
        </div>
      </div>
    </div>

    <div v-if="transactions.length > 0" class="mt-4 text-center">
      <router-link
        to="/transactions"
        class="inline-flex items-center text-blue-600 hover:text-blue-800 text-sm font-medium"
      >
        Voir toutes les transactions
        <svg class="ml-1 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
        </svg>
      </router-link>
    </div>
  </div>
</template>
