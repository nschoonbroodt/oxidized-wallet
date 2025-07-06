<script setup lang="ts">
import { ref, onMounted } from "vue";
import { commands, unwrapResult } from "@/services/api";
import type { Account, AccountNode, Money } from "@/bindings";
import AccountForm from "@/components/AccountForm.vue";

const accountNodes = ref<AccountNode[]>([]);
const balances = ref<Map<bigint, Money>>(new Map());
const hierarchicalBalances = ref<Map<bigint, Money>>(new Map());
const loading = ref<boolean>(false);
const loadingBalances = ref<boolean>(false);
const error = ref<string | null>(null);
const showForm = ref(false);

const fetchAccounts = async () => {
  loading.value = true;
  error.value = null;

  try {
    const result = await commands.getAccountTree();
    const nodes = unwrapResult(result);
    accountNodes.value = nodes;
    
    // Fetch balances for all accounts
    await fetchBalances(nodes);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    console.error("Failed to fetch accounts:", e);
  } finally {
    loading.value = false;
  }
};

const fetchBalances = async (nodes: AccountNode[]) => {
  loadingBalances.value = true;
  const newBalances = new Map<bigint, Money>();
  const newHierarchicalBalances = new Map<bigint, Money>();
  
  for (const node of nodes) {
    if (node.account.id) {
      try {
        // Fetch direct balance
        const balanceResult = await commands.getAccountBalance(node.account.id);
        const balance = unwrapResult(balanceResult);
        newBalances.set(node.account.id, balance);
        
        // Fetch hierarchical balance
        const hierarchicalResult = await commands.getAccountBalanceWithChildren(node.account.id);
        const hierarchicalBalance = unwrapResult(hierarchicalResult);
        newHierarchicalBalances.set(node.account.id, hierarchicalBalance);
      } catch (e) {
        console.error(`Failed to fetch balance for account ${node.account.id}:`, e);
      }
    }
  }
  
  balances.value = newBalances;
  hierarchicalBalances.value = newHierarchicalBalances;
  loadingBalances.value = false;
};

const formatMoney = (money: Money): string => {
  // Convert bigint to number and handle the decimal scaling
  const amountMinor = Number(money.amount_minor);
  const scale = money.currency.minor_unit_scale;
  const value = amountMinor / Math.pow(10, scale);
  
  return new Intl.NumberFormat('fr-FR', {
    style: 'currency',
    currency: money.currency.code,
  }).format(value);
};

const getBalanceClass = (money: Money): string => {
  const amount = Number(money.amount_minor);
  if (amount > 0) return 'text-green-600';
  if (amount < 0) return 'text-red-600';
  return 'text-gray-500';
};

const onAccountCreated = (_newAccount: Account) => {
  showForm.value = false; // Hide form
  fetchAccounts(); // Refresh tree from server
};

onMounted(() => {
  fetchAccounts();
});
</script>

<template>
  <div class="p-6">
    <div class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-bold">Accounts</h1>
      <button
        @click="showForm = !showForm"
        class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors"
      >
        {{ showForm ? "Cancel" : "New Account" }}
      </button>
    </div>

    <!-- New Account Form -->
    <div v-if="showForm" class="mb-6 p-4 border rounded bg-gray-50">
      <h2 class="text-lg font-semibold mb-4">Create New Account</h2>
      <AccountForm @created="onAccountCreated" @cancel="showForm = false" />
    </div>

    <div v-if="loading">Loading accounts...</div>
    <div v-else-if="error" class="text-red-500">Error: {{ error }}</div>
    <div v-else-if="accountNodes.length === 0">No accounts found</div>
    <div v-else>
      <div class="space-y-1">
        <div
          v-for="node in accountNodes"
          :key="node.account.id?.toString()"
          :style="{ paddingLeft: `${node.level * 1.5}rem` }"
          class="py-3 px-4 border rounded bg-white hover:bg-gray-50 transition-colors"
        >
          <div class="flex items-center justify-between">
            <div class="flex-1">
              <div class="flex items-center gap-2">
                <span class="font-medium">{{ node.account.name }}</span>
                <span class="text-sm text-gray-500"
                  >({{ node.account.account_type }})</span
                >
              </div>
              <div
                v-if="node.account.description"
                class="text-sm text-gray-600 mt-1"
              >
                {{ node.account.description }}
              </div>
            </div>
            
            <!-- Balance Information -->
            <div class="text-right ml-4" v-if="node.account.id">
              <div class="font-medium">
                <span 
                  v-if="loadingBalances" 
                  class="text-gray-400 animate-pulse"
                >
                  Loading...
                </span>
                <span 
                  v-else-if="balances.get(node.account.id)" 
                  class="text-lg"
                  :class="getBalanceClass(balances.get(node.account.id)!)"
                >
                  {{ formatMoney(balances.get(node.account.id)!) }}
                </span>
                <span v-else class="text-gray-400">€0.00</span>
              </div>
              
              <!-- Show hierarchical balance if different from direct balance -->
              <div 
                v-if="!loadingBalances && 
                      hierarchicalBalances.get(node.account.id) && 
                      balances.get(node.account.id) && 
                      hierarchicalBalances.get(node.account.id)!.amount_minor !== balances.get(node.account.id)!.amount_minor"
                class="text-sm"
                :class="getBalanceClass(hierarchicalBalances.get(node.account.id)!)"
                :title="'Including children: ' + formatMoney(hierarchicalBalances.get(node.account.id)!)"
              >
                ↳ {{ formatMoney(hierarchicalBalances.get(node.account.id)!) }}
              </div>
              
              <div class="text-xs text-gray-400 mt-1">Level {{ node.level }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <button
      @click="fetchAccounts"
      class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
    >
      Refresh Accounts
    </button>
  </div>
</template>
