import { commands as tauriCommands } from "@/bindings";
import type { Account, AccountNode, Transaction, TransactionFilters, Result } from "@/bindings";
import { mockAccounts } from "./mock/accounts";
import { mockTransactions, getMockTransactionById } from "./mock/transactions";
import { delay, EUR } from "./mock/utils";

// Check if we're running in Tauri
const isTauri = () => window.__TAURI_INTERNALS__ !== undefined;

// Mock implementation of commands
const mockCommands = {
  async getAccounts(): Promise<Result<Account[], string>> {
    await delay(300); // Simulate network delay

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
    description: string | null,
    currency: string
  ): Promise<Result<Account, string>> {
    await delay(500); // Simulate API delay

    // Create new account with generated ID
    const newAccount: Account = {
      id: BigInt(Date.now()), // Use timestamp as ID
      name: name,
      account_type: account_type,
      parent_id: parent_id,
      currency: EUR, // Use the EUR constant from utils
      description: description,
      is_active: true,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    // Add to mock accounts array
    mockAccounts.push(newAccount);

    return { status: "ok", data: newAccount };
  },
  async getAccountTree(): Promise<Result<AccountNode[], string>> {
    await delay(300);
    
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
};

// Export either real or mock commands based on environment
export const commands = isTauri() ? tauriCommands : mockCommands;

// Helper to extract data from Result type
export function unwrapResult<T>(result: Result<T, string>): T {
  if (result.status === "error") {
    throw new Error(result.error);
  }
  return result.data;
}
