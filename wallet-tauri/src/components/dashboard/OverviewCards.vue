<script setup lang="ts">
import MetricCard from "./MetricCard.vue";

type Metric = {
  title: string;
  value: string;
  icon: "trending-up" | "wallet" | "arrow-down" | "arrow-up";
};

const metrics: Metric[] = [
  { title: "Patrimoine net", value: "23 684,56 €", icon: "trending-up" },
  { title: "Patrimoine brut", value: "24 684,56 €", icon: "wallet" },
  { title: "Revenu mensuel", value: "3 200,00 €", icon: "arrow-down" },
  { title: "Dépenses mensuelles", value: "2 450,78 €", icon: "arrow-up" },
];
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
    <MetricCard v-for="metric in metrics" :key="metric.title" v-bind="metric" />
  </div>
</template>
