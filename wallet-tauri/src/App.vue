<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "./components/ui/sidebar";
import AppSidebar from "./components/layout/AppSidebar.vue";
import MainContent from "./components/layout/MainContent.vue";
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <main>
      <SidebarTrigger />
      <MainContent />
    </main>
  </SidebarProvider>
</template>

<style></style>
