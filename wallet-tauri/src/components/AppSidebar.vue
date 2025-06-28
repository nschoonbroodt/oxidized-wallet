<script setup lang="ts">
import { BarChart3, Home, Wallet, ArrowLeftRight } from "lucide-vue-next"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from '@/components/ui/sidebar'
import SidebarSeparator from './ui/sidebar/SidebarSeparator.vue';

  import { ref } from 'vue'

  const activeItem = ref('Tableau de bord')

const items = [
  {
    title: "Tableau de bord",
    url: "#",
    icon: Home,
  },
  {
    title: "Comptes",
    url: "#",
    icon: Wallet,
  },
  {
    title: "Transactions",
    url: "#",
    icon: ArrowLeftRight,
  },
  {
    title: "Rapports",
    url: "#",
    icon: BarChart3,
  }
];
</script>

<template>
  <Sidebar>
    <SidebarHeader>
        Oxidized Wallet
    </SidebarHeader>
    <SidebarSeparator/>
    <SidebarContent>
      <SidebarGroup >
        <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem v-for="item in items" :key="item.title">
                <SidebarMenuButton asChild>
                    <a :href="item.url"  @click="activeItem = item.title" :class="{ 'bg-accent': activeItem === item.title }">
                      <component :is="item.icon" />
                      <span>{{item.title}}</span>
                    </a>
                </SidebarMenuButton>
              </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
    <SidebarFooter>
        <div class="flex items-center gap-2 p-2">
            <User class="h-4 w-4" />
            <span class="text-sm">Nicolas</span>
        </div>
    </SidebarFooter>
  </Sidebar>
</template>