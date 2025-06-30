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
  SidebarMenuItem,
  SidebarSeparator
} from '@/components/ui/sidebar'
import { Button } from '@/components/ui/button'

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
        <Button>Nouvelle transaction</Button>
    </SidebarFooter>
  </Sidebar>
</template>