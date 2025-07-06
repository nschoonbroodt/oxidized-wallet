<script setup lang="ts">
import { useRoute, RouterLink } from "vue-router";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarSeparator,
} from "@/components/ui/sidebar";
import { Button } from "@/components/ui/button";

import { computed } from "vue";
import { router } from "@/router";

const route = useRoute();

const menuItems = computed(() => {
  return router
    .getRoutes()
    .filter((route) => route.meta?.showInNav)
    .map((route) => ({
      path: route.path,
      title: route.meta?.title,
      icon: route.meta.icon,
    }));
});
</script>

<template>
  <Sidebar>
    <SidebarHeader> Oxidized Wallet </SidebarHeader>
    <SidebarSeparator />
    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in menuItems" :key="item.path">
              <SidebarMenuButton asChild>
                <RouterLink
                  :to="item.path"
                  class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary"
                  active-class="bg-accent text-accent-foreground"
                >
                  <component :is="item.icon" class="h-4 w-4" />
                  {{ item.title }}
                </RouterLink>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
    <SidebarFooter>
    </SidebarFooter>
  </Sidebar>
</template>
