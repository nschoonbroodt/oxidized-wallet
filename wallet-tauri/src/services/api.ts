import { commands as tauriCommands } from "@/bindings";
import type { Account, AccountNode, Transaction, TransactionFilters, Result, Money } from "@/bindings";
import { mockAccounts } from "./mock/accounts";
import { mockTransactions, getMockTransactionById } from "./mock/transactions";
import { delay, EUR } from "./mock/utils";

// Check if we're running in Tauri
const isTauri = () => window.__TAURI_INTERNALS__ !== undefined;

// Mock implementation of commands
const mockCommands = {
  async getAccounts(): Promise<Result<Account[], string>> {
    await delay(10); // Simulate network delay

    // Simulate occasional errors for testing
    if (Math.random() > 0.95) {
      return {
        status: "error",
        error: "Erreur de connexion à la base de données",
      };
    }

    return {
      status: "ok",
      data: mockAccounts,
    };
  },
  async createAccount(
    name: string,
    account_type: Account["account_type"],
    parent_id: bigint | null,
    _description: string | null,
    _currency: string
  ): Promise<Result<Account, string>> {
    await delay(20); // Simulate API delay

    // Create new account with generated ID
    const newAccount: Account = {
      id: BigInt(Date.now()), // Use timestamp as ID
      name: name,
      account_type: account_type,
      parent_id: parent_id,
      currency: EUR, // Use the EUR constant from utils
      description: _description,
      is_active: true,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    // Add to mock accounts array
    mockAccounts.push(newAccount);

    return { status: "ok", data: newAccount };
  },
  async getAccountTree(): Promise<Result<AccountNode[], string>> {
    await delay(10);
    
    // Build tree structure from mock data (depth-first traversal)
    const buildTree = (parentId: bigint | null = null, level: number = 0, path: string = ""): AccountNode[] => {
      return mockAccounts
        .filter(acc => acc.parent_id === parentId)
        .sort((a, b) => a.name.localeCompare(b.name))
        .flatMap(account => {
          const nodePath = path ? `${path} > ${account.name}` : account.name;
          const node: AccountNode = {
            account,
            level,
            path: nodePath
          };
          
          // Get children recursively
          const children = buildTree(account.id!, level + 1, nodePath);
          
          // Return this node followed by its children
          return [node, ...children];
        });
    };

    return { status: "ok", data: buildTree() };
  },
  async getTransactions(filters: TransactionFilters): Promise<Result<Transaction[], string>> {
    await delay(400);
    
    // Apply date filtering
    let filteredTransactions = mockTransactions;
    if (filters.from_date) {
      filteredTransactions = filteredTransactions.filter(t => t.transaction_date >= filters.from_date!);
    }
    if (filters.to_date) {
      filteredTransactions = filteredTransactions.filter(t => t.transaction_date <= filters.to_date!);
    }

    return { status: "ok", data: filteredTransactions };
  },
  async getTransaction(id: bigint): Promise<Result<Transaction, string>> {
    await delay(200);
    
    const transaction = getMockTransactionById(id);
    if (!transaction) {
      return { status: "error", error: `Transaction ${id} not found` };
    }
    
    return { status: "ok", data: transaction };
  },
  async createSimpleTransaction(
    description: string,
    date: string,
    amountCents: bigint,
    _currencyCode: string,
    fromAccountId: bigint,
    toAccountId: bigint
  ): Promise<Result<Transaction, string>> {
    await delay(600); // Simulate database write delay
    
    // Validate accounts exist
    const fromAccount = mockAccounts.find(acc => acc.id === fromAccountId);
    const toAccount = mockAccounts.find(acc => acc.id === toAccountId);
    
    if (!fromAccount || !toAccount) {
      return { status: "error", error: "Compte source ou destination introuvable" };
    }
    
    // Create new transaction with mock data
    const newTransaction: Transaction = {
      id: BigInt(Date.now()), // Use timestamp as ID
      description,
      reference: null,
      transaction_date: date,
      created_at: new Date().toISOString(),
      tags: null,
      notes: null,
      entries: [
        {
          id: BigInt(Date.now() + 1),
          transaction_id: BigInt(Date.now()),
          account_id: fromAccountId,
          amount: {
            amount_minor: amountCents,
            currency: EUR,
          },
          entry_type: "Credit", // Money comes FROM this account
          description: null,
          created_at: new Date().toISOString(),
        },
        {
          id: BigInt(Date.now() + 2),
          transaction_id: BigInt(Date.now()),
          account_id: toAccountId,
          amount: {
            amount_minor: amountCents,
            currency: EUR,
          },
          entry_type: "Debit", // Money goes TO this account
          description: null,
          created_at: new Date().toISOString(),
        },
      ],
    };
    
    // Add to mock transactions array (at beginning for recent display)
    mockTransactions.unshift(newTransaction);
    
    return { status: "ok", data: newTransaction };
  },
  
  // Dashboard/reporting commands
  async getNetWorth(): Promise<Result<Money, string>> {
    await delay(300);
    
    // Calculate mock net worth (assets - liabilities)
    const totalAssets = BigInt(50000 * 100); // €50,000 in cents
    const totalLiabilities = BigInt(15000 * 100); // €15,000 in cents
    const netWorth = totalAssets - totalLiabilities;
    
    return {
      status: "ok",
      data: {
        amount_minor: netWorth,
        currency: EUR,
      },
    };
  },
  
  async getTotalAssets(): Promise<Result<Money, string>> {
    await delay(300);
    
    return {
      status: "ok",
      data: {
        amount_minor: BigInt(50000 * 100), // €50,000 in cents
        currency: EUR,
      },
    };
  },
  
  async getCurrentMonthIncome(): Promise<Result<Money, string>> {
    await delay(300);
    
    return {
      status: "ok",
      data: {
        amount_minor: BigInt(3500 * 100), // €3,500 in cents
        currency: EUR,
      },
    };
  },
  
  async getCurrentMonthExpenses(): Promise<Result<Money, string>> {
    await delay(300);
    
    return {
      status: "ok",
      data: {
        amount_minor: BigInt(2800 * 100), // €2,800 in cents
        currency: EUR,
      },
    };
  },
  
  async getAccountBalance(_accountId: bigint): Promise<Result<Money, string>> {
    await delay(5);
    
    // Mock balance calculation - in reality this would sum transactions
    const balanceAmount = BigInt(Math.floor(Math.random() * 10000) * 100); // Random balance
    
    return {
      status: "ok",
      data: {
        amount_minor: balanceAmount,
        currency: EUR,
      },
    };
  },
  
  async getAccountBalanceWithChildren(_accountId: bigint): Promise<Result<Money, string>> {
    await delay(10);
    
    // Mock hierarchical balance - slightly higher than individual balance
    const balanceAmount = BigInt(Math.floor(Math.random() * 15000) * 100);
    
    return {
      status: "ok",
      data: {
        amount_minor: balanceAmount,
        currency: EUR,
      },
    };
  },
  
  async getRecentTransactions(limit?: number): Promise<Result<Transaction[], string>> {
    await delay(400);
    
    const maxTransactions = limit || 10;
    const recentTransactions = mockTransactions.slice(0, maxTransactions);
    
    return {
      status: "ok",
      data: recentTransactions,
    };
  },

  async updateAccount(
    accountId: bigint,
    name: string,
    description: string | null
  ): Promise<Result<Account, string>> {
    await delay(15);
    
    // Find the account to update
    const accountIndex = mockAccounts.findIndex(acc => acc.id === accountId);
    if (accountIndex === -1) {
      return { status: "error", error: "Account not found" };
    }
    
    // Validate name is not empty
    if (!name.trim()) {
      return { status: "error", error: "Account name cannot be empty" };
    }
    
    // Update the account
    const updatedAccount = {
      ...mockAccounts[accountIndex],
      name: name.trim(),
      description: description,
      updated_at: new Date().toISOString(),
    };
    
    mockAccounts[accountIndex] = updatedAccount;
    
    return { status: "ok", data: updatedAccount };
  },

  async deactivateAccount(accountId: bigint): Promise<Result<void, string>> {
    await delay(10);
    
    // Find the account to deactivate
    const accountIndex = mockAccounts.findIndex(acc => acc.id === accountId);
    if (accountIndex === -1) {
      return { status: "error", error: "Account not found" };
    }
    
    // Check if account has children
    const hasChildren = mockAccounts.some(acc => acc.parent_id === accountId);
    if (hasChildren) {
      return { 
        status: "error", 
        error: "Cannot deactivate account - it has child accounts" 
      };
    }
    
    // Deactivate the account
    mockAccounts[accountIndex] = {
      ...mockAccounts[accountIndex],
      is_active: false,
      updated_at: new Date().toISOString(),
    };
    
    return { status: "ok", data: undefined };
  },
};

// Extend Tauri commands with new functions (until bindings are regenerated)
const extendedTauriCommands = isTauri() ? {
  ...tauriCommands,
  async updateAccount(accountId: bigint, name: string, description: string | null): Promise<Result<Account, string>> {
    try {
      const data = await (window as any).__TAURI_INTERNALS__.invoke("update_account", { accountId, name, description });
      return { status: "ok", data };
    } catch (error) {
      return { status: "error", error: String(error) };
    }
  },
  async deactivateAccount(accountId: bigint): Promise<Result<void, string>> {
    try {
      await (window as any).__TAURI_INTERNALS__.invoke("deactivate_account", { accountId });
      return { status: "ok", data: undefined };
    } catch (error) {
      return { status: "error", error: String(error) };
    }
  }
} : mockCommands;

// Export either real or mock commands based on environment
export const commands = extendedTauriCommands;

// Helper to extract data from Result type
export function unwrapResult<T>(result: Result<T, string>): T {
  if (result.status === "error") {
    throw new Error(result.error);
  }
  return result.data;
}
