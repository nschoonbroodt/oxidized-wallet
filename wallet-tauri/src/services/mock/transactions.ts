import type { Transaction } from '@/bindings';
import { EUR, mockDate } from './utils';

// Mock transactions data matching our database test data
export const mockTransactions: Transaction[] = [
  {
    id: BigInt(1),
    description: "Salaire janvier 2025",
    reference: null,
    transaction_date: "2025-07-01",
    created_at: mockDate(5),
    tags: null,
    notes: null,
    entries: [
      {
        id: BigInt(1),
        transaction_id: BigInt(1),
        account_id: BigInt(4), // Income
        amount: { amount_minor: 300000, currency: EUR }, // €3000
        entry_type: "Credit",
        description: "Salaire net",
        created_at: mockDate(5),
      },
      {
        id: BigInt(2),
        transaction_id: BigInt(1),
        account_id: BigInt(9), // Compte Courant
        amount: { amount_minor: 300000, currency: EUR },
        entry_type: "Debit",
        description: "Salaire reçu",
        created_at: mockDate(5),
      },
    ],
  },
  {
    id: BigInt(2),
    description: "Courses Monoprix",
    reference: null,
    transaction_date: "2025-07-02",
    created_at: mockDate(4),
    tags: null,
    notes: null,
    entries: [
      {
        id: BigInt(3),
        transaction_id: BigInt(2),
        account_id: BigInt(9), // Compte Courant
        amount: { amount_minor: 7500, currency: EUR }, // €75
        entry_type: "Credit",
        description: "Paiement courses",
        created_at: mockDate(4),
      },
      {
        id: BigInt(4),
        transaction_id: BigInt(2),
        account_id: BigInt(5), // Expenses
        amount: { amount_minor: 7500, currency: EUR },
        entry_type: "Debit",
        description: "Courses alimentaires",
        created_at: mockDate(4),
      },
    ],
  },
  {
    id: BigInt(3),
    description: "Virement épargne",
    reference: null,
    transaction_date: "2025-07-03",
    created_at: mockDate(3),
    tags: null,
    notes: null,
    entries: [
      {
        id: BigInt(5),
        transaction_id: BigInt(3),
        account_id: BigInt(9), // Compte Courant
        amount: { amount_minor: 50000, currency: EUR }, // €500
        entry_type: "Credit",
        description: "Virement épargne",
        created_at: mockDate(3),
      },
      {
        id: BigInt(6),
        transaction_id: BigInt(3),
        account_id: BigInt(7), // Livret A
        amount: { amount_minor: 50000, currency: EUR },
        entry_type: "Debit",
        description: "Placement Livret A",
        created_at: mockDate(3),
      },
    ],
  },
  {
    id: BigInt(4),
    description: "Restaurant",
    reference: null,
    transaction_date: "2025-07-06",
    created_at: mockDate(0),
    tags: null,
    notes: null,
    entries: [
      {
        id: BigInt(7),
        transaction_id: BigInt(4),
        account_id: BigInt(9), // Compte Courant
        amount: { amount_minor: 4500, currency: EUR }, // €45
        entry_type: "Credit",
        description: "Paiement restaurant",
        created_at: mockDate(0),
      },
      {
        id: BigInt(8),
        transaction_id: BigInt(4),
        account_id: BigInt(5), // Expenses
        amount: { amount_minor: 4500, currency: EUR },
        entry_type: "Debit",
        description: "Repas restaurant",
        created_at: mockDate(0),
      },
    ],
  },
];

// Get a single transaction by ID
export const getMockTransactionById = (id: bigint): Transaction | undefined => {
  return mockTransactions.find(t => t.id === id);
};