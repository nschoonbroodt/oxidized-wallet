<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "./components/ui/sidebar";
import AppSidebar from "./components/layout/AppSidebar.vue";
import TopBar from "./components/layout/TopBar.vue";
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <main class="flex-1 flex flex-col min-h-screen">
      <div class="flex items-center border-b p-4">
        <SidebarTrigger class="mr-4" />
        <div class="flex-1">
          <TopBar />
        </div>
      </div>
      <div class="flex-1">
        <RouterView />
      </div>
    </main>
  </SidebarProvider>
</template>

<style></style>
