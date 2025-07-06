<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "./components/ui/sidebar";
import AppSidebar from "./components/layout/AppSidebar.vue";
import TopBar from "./components/layout/TopBar.vue";
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <main>
      <div class="flex items-center border-b p-4">
        <SidebarTrigger class="mr-4" />
        <div class="flex-1">
          <TopBar />
        </div>
      </div>
      <RouterView />
    </main>
  </SidebarProvider>
</template>

<style></style>
