<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRoute } from 'vue-router'
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Settings, RefreshCw, Info } from 'lucide-vue-next'

const route = useRoute()
const lastSync = ref<Date | null>(null)

const pageTitle = computed(() => {
  return route.meta?.title || 'Oxidized Wallet'
})

const lastSyncText = computed(() => {
  if (!lastSync.value) return 'Jamais synchronisé'
  const now = new Date()
  const diffMinutes = Math.floor((now.getTime() - lastSync.value.getTime()) / (1000 * 60))
  
  if (diffMinutes < 1) return 'Synchronisé à l\'instant'
  if (diffMinutes < 60) return `Synchronisé il y a ${diffMinutes} min`
  
  const diffHours = Math.floor(diffMinutes / 60)
  if (diffHours < 24) return `Synchronisé il y a ${diffHours}h`
  
  const diffDays = Math.floor(diffHours / 24)
  return `Synchronisé il y a ${diffDays}j`
})

const refreshData = () => {
  // Since this is a local-first app, "sync" means refreshing local data
  lastSync.value = new Date()
  // In a real implementation, this would trigger data refresh
  window.location.reload()
}

const showAbout = () => {
  alert('Oxidized Wallet v0.1.0\nGestion financière personnelle avec comptabilité en partie double\nDéveloppé avec Rust + Tauri + Vue 3')
}
</script>

<template>
  <div class="flex items-center justify-between w-full">
    <div class="flex items-center gap-4">
      <h1 class="text-xl font-semibold">{{ pageTitle }}</h1>
    </div>
    
    <div class="flex items-center gap-2">
      <div class="text-sm text-muted-foreground">
        {{ lastSyncText }}
      </div>
      
      <Button
        variant="ghost"
        size="icon"
        @click="refreshData"
        title="Actualiser les données"
      >
        <RefreshCw class="h-4 w-4" />
      </Button>
      
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon" title="Paramètres">
            <Settings class="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem @click="showAbout">
            <Info class="mr-2 h-4 w-4" />
            À propos
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  </div>
</template>
