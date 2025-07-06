<script setup lang="ts">
import OverviewCards from "@/components/dashboard/OverviewCards.vue";
import RecentTransactions from "@/components/dashboard/RecentTransactions.vue";
</script>

<template>
  <div class="h-full flex flex-col">
    <OverviewCards />
    <div class="flex-1">
      <RecentTransactions />
    </div>
  </div>
</template>
