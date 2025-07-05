import { commands as tauriCommands } from "@/bindings";
import type { Account, Result } from "@/bindings";
import { mockAccounts } from "./mock/accounts";
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
