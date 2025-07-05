import type { Currency } from '@/bindings';

// Shared currency constants
export const EUR: Currency = {
  code: 'EUR',
  minor_unit_scale: 2,
  symbol: '€'
};

// Helper function to simulate network delay
export const delay = (ms: number = 300) => new Promise(resolve => setTimeout(resolve, ms));

// Helper to generate consistent dates
export const mockDate = (daysAgo: number = 0) => {
  const date = new Date();
  date.setDate(date.getDate() - daysAgo);
  return date.toISOString();
};