# Mock Data Generation with Tauri-Specta Types

This document describes strategies for generating type-safe mock data using the TypeScript types generated by tauri-specta. This complements the existing [Frontend Mock Data Guide](./FRONTEND_MOCK_DATA_GUIDE.md) with specific implementation patterns.

## Overview

While tauri-specta doesn't directly generate mock data, we can leverage the generated TypeScript types (`src/bindings.ts`) to create type-safe mock factories that ensure our mock data always matches the Rust API structure.

## Approach 1: Type-Safe Mock Factories

Create dedicated mock factories that use the generated types from `bindings.ts`. This ensures compile-time type safety.

### Implementation

```typescript
// src/mocks/factories.ts
import type { Account, AccountType, Currency, Transaction, TransactionEntry } from '@/bindings';
import { faker } from '@faker-js/faker';

// Constants for French banking context
const FRENCH_BANK_ACCOUNTS = [
  'Compte Courant',
  'Livret A',
  'PEA',
  'Assurance Vie',
  'LDD',
  'Compte Joint',
  'Compte Professionnel'
];

const FRENCH_BANKS = [
  'BoursoBank',
  'Société Générale',
  'Crédit Agricole',
  'BNP Paribas',
  'Crédit Mutuel',
  'La Banque Postale'
];

// Currency factory
export const mockEUR: Currency = {
  code: 'EUR',
  minor_unit_scale: 2,
  symbol: '€'
};

// Account factory
export const createMockAccount = (overrides?: Partial<Account>): Account => {
  const accountType = overrides?.account_type || faker.helpers.arrayElement<AccountType>([
    'Asset', 'Liability', 'Equity', 'Income', 'Expense'
  ]);
  
  const bank = faker.helpers.arrayElement(FRENCH_BANKS);
  const accountName = faker.helpers.arrayElement(FRENCH_BANK_ACCOUNTS);
  
  return {
    id: BigInt(faker.number.int({ min: 1, max: 1000 })),
    name: `${bank} - ${accountName}`,
    account_type: accountType,
    parent_id: faker.datatype.boolean() ? BigInt(faker.number.int({ min: 1, max: 100 })) : null,
    currency: mockEUR,
    description: faker.lorem.sentence(),
    is_active: true,
    created_at: faker.date.past().toISOString(),
    updated_at: faker.date.recent().toISOString(),
    ...overrides
  };
};

// Create hierarchical accounts
export const createMockAccountHierarchy = (): Account[] => {
  const bankAccount = createMockAccount({
    id: BigInt(1),
    name: 'BoursoBank',
    account_type: 'Asset',
    parent_id: null
  });

  const checkingAccount = createMockAccount({
    id: BigInt(2),
    name: 'Compte Courant',
    account_type: 'Asset',
    parent_id: bankAccount.id
  });

  const savingsAccount = createMockAccount({
    id: BigInt(3),
    name: 'Livret A',
    account_type: 'Asset',
    parent_id: bankAccount.id
  });

  return [bankAccount, checkingAccount, savingsAccount];
};

// Transaction factory
export const createMockTransaction = (overrides?: Partial<Transaction>): Transaction => ({
  id: BigInt(faker.number.int({ min: 1, max: 10000 })),
  date: faker.date.recent({ days: 30 }).toISOString().split('T')[0],
  description: faker.finance.transactionDescription(),
  created_at: faker.date.recent().toISOString(),
  updated_at: faker.date.recent().toISOString(),
  ...overrides
});

// Transaction entry factory (for double-entry bookkeeping)
export const createMockTransactionEntry = (
  transactionId: bigint,
  accountId: bigint,
  isDebit: boolean,
  amount: number
): TransactionEntry => ({
  id: BigInt(faker.number.int({ min: 1, max: 10000 })),
  transaction_id: transactionId,
  account_id: accountId,
  amount_minor: amount,
  is_debit: isDebit,
  created_at: faker.date.recent().toISOString()
});

// Create a complete double-entry transaction
export const createMockDoubleEntryTransaction = (
  fromAccountId: bigint,
  toAccountId: bigint,
  amountEuros: number,
  description?: string
): { transaction: Transaction; entries: TransactionEntry[] } => {
  const transaction = createMockTransaction({ description });
  const amountMinor = Math.round(amountEuros * 100); // Convert to cents
  
  const entries = [
    createMockTransactionEntry(transaction.id!, fromAccountId, true, amountMinor),
    createMockTransactionEntry(transaction.id!, toAccountId, false, amountMinor)
  ];
  
  return { transaction, entries };
};
```

### Usage in Components

```typescript
// src/components/AccountList.vue
import { createMockAccountHierarchy } from '@/mocks/factories';

const accounts = ref<Account[]>([]);

onMounted(async () => {
  if (import.meta.env.DEV && !window.__TAURI__) {
    // Use mock data in development without Tauri
    accounts.value = createMockAccountHierarchy();
  } else {
    // Use real Tauri commands
    accounts.value = await commands.getAccounts();
  }
});
```

## Approach 2: Service Layer with Integrated Mocks

Extend the service abstraction pattern with built-in mock data generation.

### Implementation

```typescript
// src/services/accountService.ts
import type { Account } from '@/bindings';
import { createMockAccount, createMockAccountHierarchy } from '@/mocks/factories';
import { commands } from '@/bindings';

export interface AccountService {
  getAccounts(): Promise<Account[]>;
  getAccount(id: bigint): Promise<Account | null>;
  createAccount(data: Omit<Account, 'id' | 'created_at' | 'updated_at'>): Promise<Account>;
  updateAccount(id: bigint, data: Partial<Account>): Promise<Account>;
  deleteAccount(id: bigint): Promise<void>;
  getAccountBalance(id: bigint): Promise<number>;
}

class MockAccountService implements AccountService {
  private accounts: Map<bigint, Account>;
  
  constructor() {
    // Initialize with hierarchical mock data
    const mockAccounts = createMockAccountHierarchy();
    this.accounts = new Map(mockAccounts.map(acc => [acc.id!, acc]));
    
    // Add more accounts for variety
    for (let i = 0; i < 10; i++) {
      const account = createMockAccount();
      this.accounts.set(account.id!, account);
    }
  }

  async getAccounts(): Promise<Account[]> {
    // Simulate network delay
    await new Promise(resolve => setTimeout(resolve, 200));
    return Array.from(this.accounts.values());
  }

  async getAccount(id: bigint): Promise<Account | null> {
    await new Promise(resolve => setTimeout(resolve, 100));
    return this.accounts.get(id) || null;
  }

  async createAccount(data: Omit<Account, 'id' | 'created_at' | 'updated_at'>): Promise<Account> {
    await new Promise(resolve => setTimeout(resolve, 300));
    const newAccount: Account = {
      ...data,
      id: BigInt(Date.now()),
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    };
    this.accounts.set(newAccount.id, newAccount);
    return newAccount;
  }

  async updateAccount(id: bigint, data: Partial<Account>): Promise<Account> {
    const account = this.accounts.get(id);
    if (!account) throw new Error('Account not found');
    
    const updated = {
      ...account,
      ...data,
      id, // Ensure ID doesn't change
      updated_at: new Date().toISOString()
    };
    this.accounts.set(id, updated);
    return updated;
  }

  async deleteAccount(id: bigint): Promise<void> {
    this.accounts.delete(id);
  }

  async getAccountBalance(id: bigint): Promise<number> {
    // Return random balance for mock
    return faker.number.float({ min: -1000, max: 50000, fractionDigits: 2 });
  }
}

class TauriAccountService implements AccountService {
  async getAccounts(): Promise<Account[]> {
    return await commands.getAccounts();
  }

  async getAccount(id: bigint): Promise<Account | null> {
    return await commands.getAccount({ id });
  }

  async createAccount(data: Omit<Account, 'id' | 'created_at' | 'updated_at'>): Promise<Account> {
    return await commands.createAccount(data);
  }

  async updateAccount(id: bigint, data: Partial<Account>): Promise<Account> {
    return await commands.updateAccount({ id, data });
  }

  async deleteAccount(id: bigint): Promise<void> {
    return await commands.deleteAccount({ id });
  }

  async getAccountBalance(id: bigint): Promise<number> {
    return await commands.getAccountBalance({ id });
  }
}

// Service factory
let accountServiceInstance: AccountService | null = null;

export const getAccountService = (): AccountService => {
  if (!accountServiceInstance) {
    accountServiceInstance = window.__TAURI__ 
      ? new TauriAccountService() 
      : new MockAccountService();
  }
  return accountServiceInstance;
};
```

## Approach 3: Mock State Management

For more complex scenarios, implement a mock state store that simulates backend behavior.

```typescript
// src/mocks/mockStore.ts
import { reactive } from 'vue';
import type { Account, Transaction, TransactionEntry } from '@/bindings';
import { createMockAccountHierarchy, createMockDoubleEntryTransaction } from './factories';

class MockDataStore {
  private state = reactive({
    accounts: new Map<bigint, Account>(),
    transactions: new Map<bigint, Transaction>(),
    entries: new Map<bigint, TransactionEntry[]>()
  });

  constructor() {
    this.initializeData();
  }

  private initializeData() {
    // Add initial accounts
    const accounts = createMockAccountHierarchy();
    accounts.forEach(acc => this.state.accounts.set(acc.id!, acc));

    // Add some transactions
    const [bank, checking, savings] = accounts;
    
    // Salary transaction
    const salary = createMockDoubleEntryTransaction(
      BigInt(999), // External income account
      checking.id!,
      2500,
      "Salaire mensuel"
    );
    this.state.transactions.set(salary.transaction.id!, salary.transaction);
    this.state.entries.set(salary.transaction.id!, salary.entries);

    // Transfer to savings
    const transfer = createMockDoubleEntryTransaction(
      checking.id!,
      savings.id!,
      500,
      "Virement vers Livret A"
    );
    this.state.transactions.set(transfer.transaction.id!, transfer.transaction);
    this.state.entries.set(transfer.transaction.id!, transfer.entries);
  }

  getAccounts(): Account[] {
    return Array.from(this.state.accounts.values());
  }

  getAccountBalance(accountId: bigint): number {
    let balance = 0;
    
    // Calculate balance from all transaction entries
    this.state.entries.forEach(entries => {
      entries.forEach(entry => {
        if (entry.account_id === accountId) {
          balance += entry.is_debit ? -entry.amount_minor : entry.amount_minor;
        }
      });
    });
    
    return balance / 100; // Convert from cents to euros
  }

  addTransaction(fromAccountId: bigint, toAccountId: bigint, amountEuros: number, description: string) {
    const { transaction, entries } = createMockDoubleEntryTransaction(
      fromAccountId,
      toAccountId,
      amountEuros,
      description
    );
    
    this.state.transactions.set(transaction.id!, transaction);
    this.state.entries.set(transaction.id!, entries);
    
    return transaction;
  }
}

export const mockStore = new MockDataStore();
```

## Best Practices

1. **Type Safety First**: Always use the generated types from `bindings.ts` to ensure mock data matches the API structure.

2. **Realistic Data**: Use contextually appropriate data (French bank names, EUR currency, realistic amounts).

3. **Consistent IDs**: Use BigInt for IDs to match Rust's i64 type. Be careful with BigInt serialization.

4. **Simulate Delays**: Add artificial delays to mock async operations for more realistic development.

5. **Development Only**: Ensure mock data is only used in development mode:
   ```typescript
   if (import.meta.env.DEV && !window.__TAURI__) {
     // Use mocks
   }
   ```

6. **Mock Validation**: Implement the same validation rules in mock services as the real backend.

## Testing with Mock Data

```typescript
// src/tests/mockData.test.ts
import { describe, it, expect } from 'vitest';
import { createMockAccount, createMockDoubleEntryTransaction } from '@/mocks/factories';

describe('Mock Data Factories', () => {
  it('creates valid account with correct types', () => {
    const account = createMockAccount();
    
    expect(account.id).toBeInstanceOf(BigInt);
    expect(account.currency.code).toBe('EUR');
    expect(['Asset', 'Liability', 'Equity', 'Income', 'Expense']).toContain(account.account_type);
  });

  it('creates balanced double-entry transaction', () => {
    const { transaction, entries } = createMockDoubleEntryTransaction(
      BigInt(1),
      BigInt(2),
      100,
      'Test transaction'
    );
    
    expect(entries).toHaveLength(2);
    expect(entries[0].amount_minor).toBe(10000); // 100 EUR in cents
    expect(entries[0].is_debit).toBe(true);
    expect(entries[1].is_debit).toBe(false);
  });
});
```

## Integration with Vue DevTools

The mock services can be enhanced to work with Vue DevTools for better debugging:

```typescript
// Make mock store inspectable in Vue DevTools
if (import.meta.env.DEV) {
  window.__MOCK_STORE__ = mockStore;
}
```

This approach provides a robust foundation for frontend development without requiring the full Tauri backend to be running.