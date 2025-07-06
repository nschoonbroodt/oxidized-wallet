<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { commands, unwrapResult } from "@/services/api";
import type { Transaction, Account } from "@/bindings";
import { Button } from "@/components/ui/button";

// Date helpers for current month
const now = new Date();
const currentYear = now.getFullYear();
const currentMonth = now.getMonth();

// Date filters - default to current month
const fromDate = ref(new Date(currentYear, currentMonth, 1).toISOString().split('T')[0]);
const toDate = ref(new Date(currentYear, currentMonth + 1, 0).toISOString().split('T')[0]);

const transactions = ref<Transaction[]>([]);
const accounts = ref<Account[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const showForm = ref(false);

// Helper to get account name by ID
const getAccountName = (accountId: bigint): string => {
  const account = accounts.value.find(acc => acc.id === accountId);
  return account?.name || `Account ${accountId}`;
};

// Helper to format money amount
const formatAmount = (amountMinor: number): string => {
  return (amountMinor / 100).toFixed(2);
};

// Helper to check if transaction is simple (2 entries with equal amounts)
const isSimpleTransaction = (transaction: Transaction): boolean => {
  if (transaction.entries.length !== 2) return false;
  
  const [entry1, entry2] = transaction.entries;
  return entry1.amount.amount_minor === entry2.amount.amount_minor;
};

// Helper to get simple transaction flow (from → to)
const getSimpleTransactionFlow = (transaction: Transaction): { from: string; to: string; amount: number; label: string } | null => {
  if (!isSimpleTransaction(transaction)) return null;
  
  const debitEntry = transaction.entries.find(e => e.entry_type === 'Debit');
  const creditEntry = transaction.entries.find(e => e.entry_type === 'Credit');
  
  if (!debitEntry || !creditEntry) return null;
  
  const fromAccount = accounts.value.find(acc => acc.id === creditEntry.account_id);
  const toAccount = accounts.value.find(acc => acc.id === debitEntry.account_id);
  
  // Determine transaction type based on account types
  let label = "Transfer";
  if (fromAccount?.account_type === 'Income' || toAccount?.account_type === 'Income') {
    label = "Income";
  } else if (fromAccount?.account_type === 'Expense' || toAccount?.account_type === 'Expense') {
    label = "Expense";
  }
  
  return {
    from: getAccountName(creditEntry.account_id), // Money comes FROM the credited account
    to: getAccountName(debitEntry.account_id),    // Money goes TO the debited account
    amount: debitEntry.amount.amount_minor,
    label
  };
};

const fetchTransactions = async () => {
  loading.value = true;
  error.value = null;

  try {
    // Fetch accounts for name lookup if not already loaded
    if (accounts.value.length === 0) {
      const accountsResult = await commands.getAccounts();
      accounts.value = unwrapResult(accountsResult);
    }

    // Fetch transactions with date filters
    const result = await commands.getTransactions({
      account_id: null,
      from_date: fromDate.value,
      to_date: toDate.value,
      limit: null,
      offset: null,
    });
    transactions.value = unwrapResult(result);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    console.error("Failed to fetch transactions:", e);
  } finally {
    loading.value = false;
  }
};

// Form data
const formData = ref({
  description: '',
  date: new Date().toISOString().split('T')[0], // Today's date
  amount: '',
  fromAccountId: null as bigint | null,
  toAccountId: null as bigint | null,
});
const formLoading = ref(false);
const formError = ref<string | null>(null);

// Account tree for hierarchical display
const accountTree = ref<any[]>([]);

const fetchAccountTree = async () => {
  try {
    const result = await commands.getAccountTree();
    accountTree.value = unwrapResult(result);
  } catch (e) {
    console.error("Failed to fetch account tree:", e);
  }
};

// Flatten account tree for form selection
const flattenedAccounts = computed(() => {
  const flatten = (nodes: any[], depth = 0): any[] => {
    const result: any[] = [];
    for (const node of nodes) {
      result.push({
        ...node.account,
        depth,
        displayName: '  '.repeat(depth) + node.account.name
      });
      if (node.children?.length > 0) {
        result.push(...flatten(node.children, depth + 1));
      }
    }
    return result;
  };
  return flatten(accountTree.value);
});

const createTransaction = async () => {
  if (!formData.value.description || !formData.value.amount || 
      !formData.value.fromAccountId || !formData.value.toAccountId) {
    formError.value = "Tous les champs sont requis";
    return;
  }

  if (formData.value.fromAccountId === formData.value.toAccountId) {
    formError.value = "Les comptes source et destination doivent être différents";
    return;
  }

  formLoading.value = true;
  formError.value = null;

  try {
    const amountCents = Math.round(parseFloat(formData.value.amount) * 100);
    const result = await commands.createSimpleTransaction(
      formData.value.description,
      formData.value.date,
      amountCents,
      "EUR", // Fixed currency for now
      formData.value.fromAccountId,
      formData.value.toAccountId,
    );
    
    unwrapResult(result);
    
    // Reset form and close
    formData.value = {
      description: '',
      date: new Date().toISOString().split('T')[0],
      amount: '',
      fromAccountId: null,
      toAccountId: null,
    };
    onTransactionCreated();
  } catch (e) {
    formError.value = e instanceof Error ? e.message : String(e);
    console.error("Failed to create transaction:", e);
  } finally {
    formLoading.value = false;
  }
};

const onTransactionCreated = () => {
  showForm.value = false;
  fetchTransactions(); // Refresh list
};

onMounted(() => {
  fetchTransactions();
  fetchAccountTree();
});
</script>

<template>
  <div class="p-6">
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-2xl font-bold">Transactions</h1>
      <Button
        @click="showForm = !showForm"
        class="bg-green-500 hover:bg-green-600"
      >
        {{ showForm ? "Annuler" : "Nouvelle Transaction" }}
      </Button>
    </div>

    <!-- Date Filters -->
    <div class="mb-6 p-4 border rounded bg-gray-50">
      <h2 class="text-lg font-semibold mb-3">Filtres</h2>
      <div class="flex gap-4 items-center">
        <div>
          <label class="block text-sm font-medium mb-1">Du</label>
          <input 
            v-model="fromDate" 
            type="date" 
            class="p-2 border rounded"
            @change="fetchTransactions"
          />
        </div>
        <div>
          <label class="block text-sm font-medium mb-1">Au</label>
          <input 
            v-model="toDate" 
            type="date" 
            class="p-2 border rounded"
            @change="fetchTransactions"
          />
        </div>
        <Button 
          @click="fetchTransactions"
          variant="outline"
          class="mt-6"
        >
          Actualiser
        </Button>
      </div>
    </div>

    <!-- New Transaction Form -->
    <div v-if="showForm" class="mb-6 p-4 border rounded bg-gray-50">
      <h2 class="text-lg font-semibold mb-4">Nouvelle Transaction Simple</h2>
      
      <form @submit.prevent="createTransaction" class="space-y-4">
        <!-- Description -->
        <div>
          <label class="block text-sm font-medium mb-1">Description</label>
          <input 
            v-model="formData.description"
            type="text" 
            placeholder="Ex: Salaire, Courses, etc."
            class="w-full p-2 border rounded focus:ring-2 focus:ring-blue-500"
            required
          />
        </div>

        <!-- Date -->
        <div>
          <label class="block text-sm font-medium mb-1">Date</label>
          <input 
            v-model="formData.date"
            type="date" 
            class="p-2 border rounded focus:ring-2 focus:ring-blue-500"
            required
          />
        </div>

        <!-- Amount -->
        <div>
          <label class="block text-sm font-medium mb-1">Montant (€)</label>
          <input 
            v-model="formData.amount"
            type="number" 
            step="0.01"
            min="0"
            placeholder="0.00"
            class="p-2 border rounded focus:ring-2 focus:ring-blue-500"
            required
          />
        </div>

        <!-- From Account -->
        <div>
          <label class="block text-sm font-medium mb-1">De (compte source)</label>
          <select 
            v-model="formData.fromAccountId"
            class="w-full p-2 border rounded focus:ring-2 focus:ring-blue-500"
            required
          >
            <option :value="null">Choisir un compte</option>
            <option 
              v-for="account in flattenedAccounts" 
              :key="account.id?.toString()"
              :value="account.id"
            >
              {{ account.displayName }}
            </option>
          </select>
        </div>

        <!-- To Account -->
        <div>
          <label class="block text-sm font-medium mb-1">Vers (compte destination)</label>
          <select 
            v-model="formData.toAccountId"
            class="w-full p-2 border rounded focus:ring-2 focus:ring-blue-500"
            required
          >
            <option :value="null">Choisir un compte</option>
            <option 
              v-for="account in flattenedAccounts" 
              :key="account.id?.toString()"
              :value="account.id"
            >
              {{ account.displayName }}
            </option>
          </select>
        </div>

        <!-- Error Display -->
        <div v-if="formError" class="text-red-600 text-sm">
          {{ formError }}
        </div>

        <!-- Form Actions -->
        <div class="flex gap-3 pt-2">
          <Button 
            type="submit"
            :disabled="formLoading"
            class="bg-green-500 hover:bg-green-600"
          >
            {{ formLoading ? "Création..." : "Créer Transaction" }}
          </Button>
          <Button 
            type="button"
            @click="showForm = false"
            variant="outline"
          >
            Annuler
          </Button>
        </div>
      </form>
    </div>

    <!-- Transaction List -->
    <div v-if="loading">Chargement des transactions...</div>
    <div v-else-if="error" class="text-red-500">Erreur: {{ error }}</div>
    <div v-else-if="transactions.length === 0" class="text-center py-8 text-gray-500">
      Aucune transaction trouvée pour cette période
    </div>
    <div v-else>
      <div class="space-y-4">
        <div 
          v-for="transaction in transactions" 
          :key="transaction.id?.toString()"
          class="p-4 border rounded bg-white hover:bg-gray-50"
        >
          <!-- Transaction Header -->
          <div class="flex items-center justify-between mb-3">
            <div>
              <h3 class="font-semibold text-lg">{{ transaction.description }}</h3>
              <p class="text-sm text-gray-600">{{ transaction.transaction_date }}</p>
            </div>
            <div class="text-xs text-gray-400">
              ID: {{ transaction.id }}
            </div>
          </div>

          <!-- Simple Transaction View (2 equal entries) -->
          <div 
            v-if="isSimpleTransaction(transaction)" 
            class="py-2 px-3 rounded"
            :class="{
              'bg-green-50': getSimpleTransactionFlow(transaction)?.label === 'Income',
              'bg-red-50': getSimpleTransactionFlow(transaction)?.label === 'Expense', 
              'bg-blue-50': getSimpleTransactionFlow(transaction)?.label === 'Transfer'
            }"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <span 
                  class="text-sm font-medium"
                  :class="{
                    'text-green-700': getSimpleTransactionFlow(transaction)?.label === 'Income',
                    'text-red-700': getSimpleTransactionFlow(transaction)?.label === 'Expense',
                    'text-blue-700': getSimpleTransactionFlow(transaction)?.label === 'Transfer'
                  }"
                >
                  {{ getSimpleTransactionFlow(transaction)?.label }}:
                </span>
                <span class="font-medium">{{ getSimpleTransactionFlow(transaction)?.from }}</span>
                <span class="text-gray-400">→</span>
                <span class="font-medium">{{ getSimpleTransactionFlow(transaction)?.to }}</span>
              </div>
              <div 
                class="font-mono font-semibold"
                :class="{
                  'text-green-700': getSimpleTransactionFlow(transaction)?.label === 'Income',
                  'text-red-700': getSimpleTransactionFlow(transaction)?.label === 'Expense',
                  'text-blue-700': getSimpleTransactionFlow(transaction)?.label === 'Transfer'
                }"
              >
                €{{ formatAmount(getSimpleTransactionFlow(transaction)?.amount || 0) }}
              </div>
            </div>
          </div>

          <!-- Detailed Transaction View (complex transactions) -->
          <div v-else class="space-y-2">
            <div 
              v-for="entry in transaction.entries" 
              :key="entry.id?.toString()"
              class="flex items-center justify-between py-2 px-3 rounded"
              :class="entry.entry_type === 'Debit' ? 'bg-red-50' : 'bg-green-50'"
            >
              <div class="flex items-center gap-3">
                <!-- Entry Type Badge -->
                <span 
                  class="px-2 py-1 text-xs font-medium rounded"
                  :class="entry.entry_type === 'Debit' 
                    ? 'bg-red-100 text-red-800' 
                    : 'bg-green-100 text-green-800'"
                >
                  {{ entry.entry_type }}
                </span>
                
                <!-- Account Name -->
                <span class="font-medium">{{ getAccountName(entry.account_id) }}</span>
                
                <!-- Entry Description -->
                <span v-if="entry.description" class="text-sm text-gray-600">
                  - {{ entry.description }}
                </span>
              </div>

              <!-- Amount -->
              <div class="font-mono font-semibold">
                {{ entry.entry_type === 'Debit' ? '+' : '-' }}€{{ formatAmount(entry.amount.amount_minor) }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>