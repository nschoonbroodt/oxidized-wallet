    <script setup lang="ts">
    import { ref, onMounted } from 'vue';
    import { commands, unwrapResult } from '@/services/api';
    import type { Account } from '@/bindings';
    
    const accounts = ref<Account[]>([]);
    const loading = ref<boolean>(false);
    const error = ref<string | null>(null);
    
    const fetchAccounts = async () => {
      loading.value = true;
      error.value = null;
    
      try {
        const result = await commands.getAccounts();
        accounts.value = unwrapResult(result);
      } catch (e) {
        error.value = e instanceof Error ? e.message : String(e);
        console.error('Failed to fetch accounts:', e);
      } finally {
        loading.value = false;
      }
    };
    
    onMounted(() => {
      fetchAccounts();
    });
    </script>

<template>
  <div class="p-6">
    <div class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-bold">Accounts</h1>
    </div>

    <div v-if="loading">Loading accounts...</div>
    <div v-else-if="error" class="text-red-500">Error: {{ error }}</div>
    <div v-else-if="accounts.length === 0">No accounts found</div>
    <div v-else>
      <div class="space-y-4">
        <div v-for="account in accounts" :key="account.id?.toString()" class="p-4 border rounded">
          <h3 class="font-semibold">{{ account.name }}</h3>
          <p class="text-sm text-gray-600">Type: {{ account.account_type }}</p>
          <p class="text-sm text-gray-600">Currency: {{ account.currency.symbol }}</p>
          <p v-if="account.description" class="text-sm">{{ account.description }}</p>
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
