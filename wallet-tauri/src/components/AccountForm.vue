<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { commands, unwrapResult } from "@/services/api";
import type { Account, AccountType } from "@/bindings";

// Props & Emits
const emit = defineEmits<{
  created: [account: Account];
  cancel: [];
}>();

// Form state
const loading = ref(false);
const error = ref<string | null>(null);
const formData = ref({
  account_type: "" as AccountType | "",
  parent_id: null as bigint | null,
  name: "",
  description: "",
  currency: "EUR",
});

// Accounts for the parent dropdown
const accounts = ref<Account[]>([]);

// Fetch accounts for dropdown on component mount
const fetchAccountsForDropdown = async () => {
  try {
    const result = await commands.getAccounts();
    accounts.value = unwrapResult(result);
  } catch (e) {
    console.error('Failed to fetch accounts for dropdown:', e);
  }
};

// Computed property for valid parent accounts - filter by same type
const parentAccounts = computed(() => {
  if (!formData.value.account_type) return [];
  
  return accounts.value.filter(account => {
    // Same type can be parent of same type
    return account.account_type === formData.value.account_type;
  });
});

// Submit handler - this will error and guide us to implement the command
const handleSubmit = async () => {
  if (!formData.value.account_type || !formData.value.name.trim()) {
    error.value = "Account type and name are required";
    return;
  }

  loading.value = true;
  error.value = null;

  try {
    // Call the Tauri command with the correct parameter names
    const result = await commands.createAccount(
      formData.value.name.trim(),
      formData.value.account_type,
      formData.value.parent_id,
      formData.value.description || null,
      formData.value.currency
    );

    const newAccount = unwrapResult(result);

    // Reset form
    formData.value = {
      account_type: "",
      parent_id: null,
      name: "",
      description: "",
      currency: "EUR",
    };

    // Emit success
    emit("created", newAccount);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    loading.value = false;
  }
};

// Fetch accounts when component mounts
onMounted(() => {
  fetchAccountsForDropdown();
});
</script>

<template>
  <form @submit.prevent="handleSubmit" class="space-y-4">
    <!-- Account Type (First) -->
    <div>
      <label class="block text-sm font-medium mb-1">Account Type *</label>
      <select
        v-model="formData.account_type"
        required
        class="w-full p-2 border rounded"
      >
        <option value="">Select type...</option>
        <option value="Asset">
          💰 Asset (Actif) - Bank accounts, cash, investments
        </option>
        <option value="Liability">
          💳 Liability (Passif) - Loans, credit cards
        </option>
        <option value="Equity">
          🏦 Equity (Capitaux propres) - Owner's equity
        </option>
        <option value="Income">📈 Income (Revenus) - Salary, bonuses</option>
        <option value="Expense">
          📉 Expense (Dépenses) - Food, transport, bills
        </option>
      </select>
    </div>

    <!-- Parent Account -->
    <div>
      <label class="block text-sm font-medium mb-1">Parent Account</label>
      <select v-model="formData.parent_id" class="w-full p-2 border rounded">
        <option value="">None (Root account)</option>
        <option
          v-for="account in parentAccounts"
          :key="account.id?.toString()"
          :value="account.id"
        >
          {{ account.name }}
        </option>
      </select>
    </div>

    <!-- Account Name -->
    <div>
      <label class="block text-sm font-medium mb-1">Account Name *</label>
      <input
        v-model="formData.name"
        type="text"
        required
        class="w-full p-2 border rounded"
        placeholder="e.g., Compte Courant"
      />
    </div>

    <!-- Description -->
    <div>
      <label class="block text-sm font-medium mb-1">Description</label>
      <textarea
        v-model="formData.description"
        class="w-full p-2 border rounded"
        rows="2"
        placeholder="Optional description..."
      />
    </div>

    <!-- Currency (Hidden for now) -->
    <input type="hidden" v-model="formData.currency" value="EUR" />

    <!-- Buttons -->
    <div class="flex space-x-2">
      <button
        type="submit"
        :disabled="loading"
        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50"
      >
        {{ loading ? "Creating..." : "Create Account" }}
      </button>
      <button
        type="button"
        @click="emit('cancel')"
        class="px-4 py-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400"
      >
        Cancel
      </button>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="text-red-500 text-sm">
      {{ error }}
    </div>
  </form>
</template>
