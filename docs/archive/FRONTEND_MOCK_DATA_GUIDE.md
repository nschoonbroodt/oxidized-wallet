# Frontend Development with Mock Data

This guide explains how to develop the Vue frontend independently from Tauri for faster development cycles.

## Overview

Developing the Vue frontend without starting Tauri is a **best practice** that allows for:
- ⚡ Faster development (no Rust compilation)
- 🎨 UI-focused work without backend dependencies
- 🧪 Easy testing of edge cases
- 👥 Better team collaboration
- 📱 Instant hot reload feedback

## Recommended Approach: Service Layer with Mock Data

### 1. Create a Service Abstraction

```typescript
// src/services/api.ts
interface AccountService {
  getAccounts(): Promise<Account[]>
  getBalance(accountId: number): Promise<Money>
}

// Detect if running in Tauri
const isTauri = () => {
  return window.__TAURI__ !== undefined
}

// Factory to get the right implementation
export function getAccountService(): AccountService {
  if (isTauri()) {
    return new TauriAccountService()
  } else {
    return new MockAccountService()
  }
}
```

### 2. Tauri Implementation

```typescript
// src/services/tauri-api.ts
import { invoke } from '@tauri-apps/api/tauri'

export class TauriAccountService implements AccountService {
  async getAccounts() {
    return await invoke('get_accounts')
  }
  
  async getBalance(accountId: number) {
    return await invoke('get_account_balance', { accountId })
  }
}
```

### 3. Mock Implementation

```typescript
// src/services/mock-api.ts
export class MockAccountService implements AccountService {
  async getAccounts() {
    // Simulate network delay
    await new Promise(resolve => setTimeout(resolve, 300))
    
    return [
      {
        id: 1,
        name: "BoursoBank",
        type: "bank",
        balance: { amount_minor: 1523456, currency: "EUR" },
        children: [
          { id: 2, name: "Compte Courant", type: "checking", balance: { amount_minor: 523456, currency: "EUR" } },
          { id: 3, name: "Livret A", type: "savings", balance: { amount_minor: 1000000, currency: "EUR" } }
        ]
      },
      {
        id: 4,
        name: "Crédit Agricole",
        type: "bank",
        balance: { amount_minor: 845000, currency: "EUR" }
      }
    ]
  }
  
  async getBalance(accountId: number) {
    await new Promise(resolve => setTimeout(resolve, 100))
    return { amount_minor: 1523456, currency: "EUR" }
  }
}
```

### 4. Use in Components

```vue
<!-- OverviewCards.vue -->
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getAccountService } from '@/services/api'
import MetricCard from './MetricCard.vue'

const accountService = getAccountService()

const netWorth = ref<string>('...')
const totalAssets = ref<string>('...')

onMounted(async () => {
  try {
    const accounts = await accountService.getAccounts()
    // Calculate totals...
    netWorth.value = formatMoney(calculateNetWorth(accounts))
    totalAssets.value = formatMoney(calculateAssets(accounts))
  } catch (error) {
    console.error('Failed to load accounts:', error)
  }
})

const metrics = computed(() => [
  { title: "Patrimoine net", value: netWorth.value, icon: "trending-up" },
  { title: "Actifs totaux", value: totalAssets.value, icon: "wallet" },
  // ...
])
</script>
```

## Alternative: Environment Variables

### Use Vite env variables:

```typescript
// .env.development
VITE_USE_MOCK_DATA=true

// .env.production  
VITE_USE_MOCK_DATA=false
```

```typescript
// src/services/api.ts
const useMockData = import.meta.env.VITE_USE_MOCK_DATA === 'true' || !window.__TAURI__

export const api = useMockData ? mockApi : tauriApi
```

## Running Frontend Only

### 1. Add npm scripts:

```json
// package.json
{
  "scripts": {
    "dev": "vite",              // Frontend only (fast!)
    "tauri": "tauri dev",       // Full Tauri app
    "preview": "vite preview"   // Production preview
  }
}
```

### 2. Run with:

```bash
# Fast frontend development (mock data)
npm run dev

# Full app with backend
npm run tauri dev
```

## Best Practices

### 1. Keep Mock Data Realistic

```typescript
// Match your actual data structure exactly
const mockTransaction = {
  id: 1,
  description: "Salaire Novembre",
  amount: { amount_minor: 320000, currency: "EUR" },
  date: "2024-11-28",
  entries: [
    { account_id: 1, type: "credit", amount: 320000 },
    { account_id: 2, type: "debit", amount: 320000 }
  ]
}
```

### 2. Add Loading States

```vue
<template>
  <div v-if="loading" class="grid grid-cols-4 gap-4">
    <Skeleton v-for="i in 4" :key="i" class="h-32" />
  </div>
  <div v-else class="grid grid-cols-4 gap-4">
    <MetricCard v-for="metric in metrics" :key="metric.title" v-bind="metric" />
  </div>
</template>
```

### 3. Mock Error Scenarios

```typescript
export class MockAccountService {
  async getAccounts() {
    // Randomly fail to test error handling
    if (Math.random() > 0.9) {
      throw new Error('Network error')
    }
    // ...
  }
}
```

### 4. Mock Delay for Realistic UX

```typescript
// Simulate network latency
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

export class MockAccountService {
  async getTransactions() {
    await delay(500) // Simulate API call
    return mockTransactions
  }
}
```

## Advantages

- **Fast Development**: No Rust compilation needed
- **UI-Focused**: Work on design without backend
- **Easy Testing**: Test edge cases with controlled data
- **Team Collaboration**: Designers/frontend devs can work independently
- **Hot Reload**: Instant feedback on changes

## Mock Data Examples for Oxidized Wallet

### Accounts
```typescript
const mockAccounts = [
  {
    id: 1,
    name: "BoursoBank",
    account_type: "bank",
    parent_id: null,
    currency: "EUR",
    balance: 1523456, // in cents
    children: [
      {
        id: 2,
        name: "Compte Courant",
        account_type: "asset",
        parent_id: 1,
        currency: "EUR",
        balance: 523456
      },
      {
        id: 3,
        name: "Livret A",
        account_type: "asset",
        parent_id: 1,
        currency: "EUR",
        balance: 1000000
      }
    ]
  }
]
```

### Transactions
```typescript
const mockTransactions = [
  {
    id: 1,
    description: "Salaire Novembre",
    transaction_date: "2024-11-28",
    entries: [
      {
        account_id: 10, // Income account
        entry_type: "credit",
        amount_minor: 320000
      },
      {
        account_id: 2, // Checking account
        entry_type: "debit",
        amount_minor: 320000
      }
    ]
  },
  {
    id: 2,
    description: "Courses Carrefour",
    transaction_date: "2024-11-27",
    entries: [
      {
        account_id: 2, // Checking account
        entry_type: "credit",
        amount_minor: 12456
      },
      {
        account_id: 15, // Grocery expense
        entry_type: "debit",
        amount_minor: 12456
      }
    ]
  }
]
```

## Directory Structure

```
src/
├── services/
│   ├── api.ts           # Service factory
│   ├── types.ts         # Shared interfaces
│   ├── tauri-api.ts     # Real Tauri implementation
│   └── mock-api.ts      # Mock implementation
├── composables/
│   └── useAccounts.ts   # Vue composables using services
└── components/
    └── dashboard/
        └── OverviewCards.vue
```

This approach keeps your frontend development fast while maintaining the same API interface for both mock and real data.