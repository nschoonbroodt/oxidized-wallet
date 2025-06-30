<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "./components/ui/sidebar";
import AppSidebar from "./components/layout/AppSidebar.vue";
import MainContent from "./components/layout/MainContent.vue";
import { useRoute } from "vue-router";
import { computed } from "vue";

const route = useRoute();
const pageTitle = computed(() => route.meta?.title || "Oxidized Wallet");
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <main>
      <header class="border-b p-4">
        <SidebarTrigger />
        <h1 class="text-xl font-semibold">{{ pageTitle }}</h1>
      </header>
      <RouterView />
    </main>
  </SidebarProvider>
</template>

<style></style>
