<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { commands, unwrapResult } from "@/services/api";
import type { Account, AccountNode, Money } from "@/bindings";
import AccountForm from "@/components/AccountForm.vue";
import AccountEditDialog from "@/components/AccountEditDialog.vue";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";

const accountNodes = ref<AccountNode[]>([]);
const balances = ref<Map<bigint, Money>>(new Map());
const hierarchicalBalances = ref<Map<bigint, Money>>(new Map());
const loading = ref<boolean>(false);
const loadingBalances = ref<boolean>(false);
const error = ref<string | null>(null);
const showForm = ref(false);
const showInactive = ref(false);
const editingAccount = ref<Account | null>(null);
const showEditDialog = ref(false);
const accountToDeactivate = ref<Account | null>(null);
const showDeactivateDialog = ref(false);
const deactivateError = ref<string | null>(null);

// Computed property to get all accounts for parent lookup
const allAccounts = computed(() => {
  return accountNodes.value.map(node => node.account);
});

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

const editAccount = (account: Account) => {
  editingAccount.value = account;
  showEditDialog.value = true;
};

const onAccountEdited = (_updatedAccount: Account) => {
  showEditDialog.value = false;
  editingAccount.value = null;
  fetchAccounts(); // Refresh tree from server
};

const closeEditDialog = () => {
  showEditDialog.value = false;
  editingAccount.value = null;
};

const deactivateAccount = (account: Account) => {
  accountToDeactivate.value = account;
  deactivateError.value = null;
  showDeactivateDialog.value = true;
};

const confirmDeactivateAccount = async () => {
  const account = accountToDeactivate.value;
  if (!account?.id) return;

  try {
    const result = await commands.deactivateAccount(account.id);
    unwrapResult(result);
    
    // Close dialog and refresh accounts
    showDeactivateDialog.value = false;
    accountToDeactivate.value = null;
    deactivateError.value = null;
    
    // Refresh accounts to reflect the change
    await fetchAccounts();
  } catch (e) {
    const errorMessage = e instanceof Error ? e.message : String(e);
    deactivateError.value = `Failed to deactivate account: ${errorMessage}`;
    console.error('Failed to deactivate account:', e);
  }
};

const cancelDeactivateAccount = () => {
  showDeactivateDialog.value = false;
  accountToDeactivate.value = null;
  deactivateError.value = null;
};

onMounted(() => {
  fetchAccounts();
});
</script>

<template>
  <div class="h-full p-6">
    <div class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-bold">Accounts</h1>
      <div class="flex items-center gap-4">
        <label class="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            v-model="showInactive"
            class="rounded"
          />
          Show inactive accounts
        </label>
        <button
          @click="showForm = !showForm"
          class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors"
        >
          {{ showForm ? "Cancel" : "New Account" }}
        </button>
      </div>
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
          v-show="showInactive || node.account.is_active"
          :key="node.account.id?.toString()"
          :style="{ paddingLeft: `${node.level * 1.5}rem` }"
          :class="[
            'py-3 px-4 border rounded transition-colors',
            node.account.is_active 
              ? 'bg-white hover:bg-gray-50' 
              : 'bg-gray-100 border-gray-300'
          ]"
        >
          <div class="flex items-center justify-between">
            <div class="flex-1">
              <div class="flex items-center gap-2">
                <span 
                  :class="[
                    'font-medium',
                    !node.account.is_active && 'italic text-gray-500'
                  ]"
                >
                  {{ node.account.name }}
                  <span v-if="!node.account.is_active" class="text-xs">(Inactive)</span>
                </span>
                <span class="text-sm text-gray-500"
                  >({{ node.account.account_type }})</span
                >
              </div>
              <div
                v-if="node.account.description"
                :class="[
                  'text-sm mt-1',
                  node.account.is_active ? 'text-gray-600' : 'text-gray-400'
                ]"
              >
                {{ node.account.description }}
              </div>
            </div>
            
            <!-- Action Buttons -->
            <div class="flex items-center gap-1 ml-4" v-if="node.account.id && node.account.is_active">
              <button
                @click="editAccount(node.account)"
                class="p-1.5 text-blue-600 hover:bg-blue-100 rounded transition-colors"
                title="Edit account"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                        d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
              </button>
              <button
                @click="deactivateAccount(node.account)"
                class="p-1.5 text-red-600 hover:bg-red-100 rounded transition-colors"
                title="Deactivate account"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                        d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z" />
                </svg>
              </button>
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

    <!-- Account Edit Dialog -->
    <AccountEditDialog
      :account="editingAccount"
      :all-accounts="allAccounts"
      :is-open="showEditDialog"
      @close="closeEditDialog"
      @save="onAccountEdited"
    />

    <!-- Account Deactivate Confirmation Dialog -->
    <AlertDialog :open="showDeactivateDialog" @update:open="showDeactivateDialog = $event">
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Deactivate Account</AlertDialogTitle>
          <AlertDialogDescription>
            Are you sure you want to deactivate account "{{ accountToDeactivate?.name }}"?
            
            <div class="mt-3 space-y-2">
              <div class="text-sm">This will:</div>
              <ul class="text-sm list-disc list-inside space-y-1 ml-2">
                <li>Hide the account from normal views</li>
                <li>Preserve all transaction history</li>
                <li>Prevent new transactions</li>
              </ul>
              <div class="text-sm text-orange-600 mt-2">
                <strong>Note:</strong> You cannot deactivate accounts with child accounts.
              </div>
            </div>
          </AlertDialogDescription>
        </AlertDialogHeader>
        
        <div v-if="deactivateError" class="text-red-600 text-sm bg-red-50 p-3 rounded-md">
          {{ deactivateError }}
        </div>

        <AlertDialogFooter>
          <AlertDialogCancel @click="cancelDeactivateAccount">Cancel</AlertDialogCancel>
          <AlertDialogAction 
            @click="confirmDeactivateAccount"
            class="bg-red-600 hover:bg-red-700"
          >
            Deactivate
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  </div>
</template>
