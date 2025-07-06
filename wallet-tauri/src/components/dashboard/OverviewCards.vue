<script setup lang="ts">
import { ref, onMounted } from "vue";
import MetricCard from "./MetricCard.vue";
import { commands, unwrapResult } from "@/services/api";
import type { Money } from "@/bindings";

type Metric = {
  title: string;
  value: string;
  icon: "trending-up" | "wallet" | "arrow-down" | "arrow-up";
  loading?: boolean;
};

const metrics = ref<Metric[]>([
  { title: "Patrimoine net", value: "€0.00", icon: "trending-up", loading: true },
  { title: "Patrimoine brut", value: "€0.00", icon: "wallet", loading: true },
  { title: "Revenu mensuel", value: "€0.00", icon: "arrow-down", loading: true },
  { title: "Dépenses mensuelles", value: "€0.00", icon: "arrow-up", loading: true },
]);

const formatMoney = (money: Money): string => {
  const amountMinor = Number(money.amount_minor);
  const scale = money.currency.minor_unit_scale;
  const value = amountMinor / Math.pow(10, scale);
  
  return new Intl.NumberFormat('fr-FR', {
    style: 'currency',
    currency: money.currency.code,
  }).format(value);
};

const fetchDashboardData = async () => {
  try {
    // Fetch net worth
    const netWorthResult = await commands.getNetWorth();
    const netWorth = unwrapResult(netWorthResult);
    metrics.value[0].value = formatMoney(netWorth);
    metrics.value[0].loading = false;
    
    // Fetch total assets
    const totalAssetsResult = await commands.getTotalAssets();
    const totalAssets = unwrapResult(totalAssetsResult);
    metrics.value[1].value = formatMoney(totalAssets);
    metrics.value[1].loading = false;
    
    // Fetch monthly income
    const monthlyIncomeResult = await commands.getCurrentMonthIncome();
    const monthlyIncome = unwrapResult(monthlyIncomeResult);
    metrics.value[2].value = formatMoney(monthlyIncome);
    metrics.value[2].loading = false;
    
    // Fetch monthly expenses
    const monthlyExpensesResult = await commands.getCurrentMonthExpenses();
    const monthlyExpenses = unwrapResult(monthlyExpensesResult);
    metrics.value[3].value = formatMoney(monthlyExpenses);
    metrics.value[3].loading = false;
    
  } catch (error) {
    console.error('Failed to fetch dashboard data:', error);
    // Set error state but keep loading false
    metrics.value.forEach(metric => metric.loading = false);
  }
};

onMounted(() => {
  fetchDashboardData();
});
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
    <MetricCard v-for="metric in metrics" :key="metric.title" v-bind="metric" />
  </div>
</template>
