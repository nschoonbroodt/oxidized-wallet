<script setup lang="ts">
import { ref, onMounted } from "vue";
import { commands, unwrapResult } from "@/services/api";
import type { Account, AccountNode } from "@/bindings";
import AccountForm from "@/components/AccountForm.vue";

const accountNodes = ref<AccountNode[]>([]);
const loading = ref<boolean>(false);
const error = ref<string | null>(null);
const showForm = ref(false);

const fetchAccounts = async () => {
  loading.value = true;
  error.value = null;

  try {
    const result = await commands.getAccountTree();
    accountNodes.value = unwrapResult(result);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    console.error("Failed to fetch accounts:", e);
  } finally {
    loading.value = false;
  }
};

const onAccountCreated = (newAccount: Account) => {
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
          class="py-2 px-3 border rounded bg-white hover:bg-gray-50"
        >
          <div class="flex items-center justify-between">
            <div>
              <span class="font-medium">{{ node.account.name }}</span>
              <span class="text-sm text-gray-500 ml-2"
                >({{ node.account.account_type }})</span
              >
              <div
                v-if="node.account.description"
                class="text-sm text-gray-600"
              >
                {{ node.account.description }}
              </div>
            </div>
            <div class="text-xs text-gray-400">Level {{ node.level }}</div>
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
