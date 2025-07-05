import type { Account } from '@/bindings';
import { EUR, mockDate } from './utils';

// Mock accounts data for French banking context
export const mockAccounts: Account[] = [
  // === ASSET ACCOUNTS ===
  // BoursoBank hierarchy
  {
    id: BigInt(1),
    name: "BoursoBank",
    account_type: "Asset",
    parent_id: null,
    currency: EUR,
    description: "Banque principale",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(1)
  },
  {
    id: BigInt(2),
    name: "Compte Courant",
    account_type: "Asset",
    parent_id: BigInt(1),
    currency: EUR,
    description: "Compte courant principal",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(0)
  },
  {
    id: BigInt(3),
    name: "Livret A",
    account_type: "Asset",
    parent_id: BigInt(1),
    currency: EUR,
    description: "Épargne réglementée - Taux 3%",
    is_active: true,
    created_at: mockDate(300),
    updated_at: mockDate(30)
  },
  
  // Crédit Agricole hierarchy
  {
    id: BigInt(4),
    name: "Crédit Agricole",
    account_type: "Asset",
    parent_id: null,
    currency: EUR,
    description: "Banque secondaire",
    is_active: true,
    created_at: mockDate(500),
    updated_at: mockDate(10)
  },
  {
    id: BigInt(5),
    name: "PEA",
    account_type: "Asset",
    parent_id: BigInt(4),
    currency: EUR,
    description: "Plan d'Épargne en Actions",
    is_active: true,
    created_at: mockDate(200),
    updated_at: mockDate(5)
  },
  
  // === INCOME ACCOUNTS ===
  {
    id: BigInt(10),
    name: "Revenus",
    account_type: "Income",
    parent_id: null,
    currency: EUR,
    description: "Catégorie des revenus",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(365)
  },
  {
    id: BigInt(11),
    name: "Salaire",
    account_type: "Income",
    parent_id: BigInt(10),
    currency: EUR,
    description: "Salaire mensuel net",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(1)
  },
  {
    id: BigInt(12),
    name: "Primes",
    account_type: "Income",
    parent_id: BigInt(10),
    currency: EUR,
    description: "Primes et bonus",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(90)
  },
  
  // === EXPENSE ACCOUNTS ===
  {
    id: BigInt(20),
    name: "Dépenses",
    account_type: "Expense",
    parent_id: null,
    currency: EUR,
    description: "Catégorie des dépenses",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(365)
  },
  {
    id: BigInt(21),
    name: "Alimentation",
    account_type: "Expense",
    parent_id: BigInt(20),
    currency: EUR,
    description: "Courses et restaurants",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(0)
  },
  {
    id: BigInt(22),
    name: "Transport",
    account_type: "Expense",
    parent_id: BigInt(20),
    currency: EUR,
    description: "Essence, transports en commun",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(2)
  },
  {
    id: BigInt(23),
    name: "Logement",
    account_type: "Expense", 
    parent_id: BigInt(20),
    currency: EUR,
    description: "Loyer, charges, électricité",
    is_active: true,
    created_at: mockDate(365),
    updated_at: mockDate(1)
  }
];

// Get a single account by ID
export const getMockAccountById = (id: bigint): Account | undefined => {
  return mockAccounts.find(acc => acc.id === id);
};

// Get accounts by parent ID
export const getMockAccountsByParent = (parentId: bigint | null): Account[] => {
  return mockAccounts.filter(acc => acc.parent_id === parentId);
};