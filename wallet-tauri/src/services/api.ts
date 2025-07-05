import { commands as tauriCommands } from '@/bindings';
import type { Account, Result } from '@/bindings';
import { mockAccounts } from './mock/accounts';
import { delay } from './mock/utils';

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
        error: "Erreur de connexion à la base de données" 
      };
    }
    
    return { 
      status: "ok", 
      data: mockAccounts 
    };
  }
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