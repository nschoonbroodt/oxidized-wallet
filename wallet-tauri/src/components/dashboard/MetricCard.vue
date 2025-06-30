<script setup lang="ts">
import Card from "@/components/ui/card/Card.vue";
import CardContent from "@/components/ui/card/CardContent.vue";
import CardFooter from "@/components/ui/card/CardFooter.vue";
import CardHeader from "@/components/ui/card/CardHeader.vue";
import CardTitle from "@/components/ui/card/CardTitle.vue";

import { TrendingUp, Wallet, ArrowDown, ArrowUp } from "lucide-vue-next";

const iconMap = {
  "trending-up": TrendingUp,
  wallet: Wallet,
  "arrow-down": ArrowDown,
  "arrow-up": ArrowUp,
} as const;

type IconKey = keyof typeof iconMap;

defineProps<{
  title: string;
  value: string;
  icon: IconKey;
  trend?: string;
  trendDirection?: "up" | "down";
}>();
</script>

<template>
  <Card class="account-card">
    <CardHeader class="pb-2">
      <div class="flex items-center justify-between">
        <CardTitle class="text-sm font-medium text-gray-500">{{
          title
        }}</CardTitle>
        <component :is="iconMap[icon]" class="w-4 h-4 text-blue-500" />
      </div>
    </CardHeader>
    <CardContent>
      <p class="text-2xl font-bold text-gray-900">{{ value }}</p>
    </CardContent>
    <CardFooter v-if="trend" class="pt-1">
      <p
        :class="[
          'text-sm',
          trendDirection === 'up' ? 'text-green-600' : 'text-red-600',
        ]"
      >
        {{ trend }}
      </p>
    </CardFooter>
  </Card>
</template>
