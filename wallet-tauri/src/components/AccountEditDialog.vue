<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import type { Account } from '@/bindings'

export interface Props {
  account: Account | null
  allAccounts: Account[]
  isOpen: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  close: []
  save: [account: Account]
}>()

const name = ref('')
const description = ref('')
const isSubmitting = ref(false)
const error = ref<string | null>(null)

// Find parent account name
const parentAccountName = computed(() => {
  if (!props.account?.parent_id || !props.allAccounts) return 'Root Account'
  
  const parent = props.allAccounts.find(acc => acc.id === props.account!.parent_id)
  return parent ? parent.name : 'Unknown Parent'
})

// Watch for account changes to populate form
watch(() => props.account, (newAccount) => {
  if (newAccount) {
    name.value = newAccount.name
    description.value = newAccount.description || ''
  } else {
    name.value = ''
    description.value = ''
  }
  error.value = null
}, { immediate: true })

const handleSubmit = async () => {
  if (!props.account || !name.value.trim()) {
    error.value = 'Account name is required'
    return
  }

  if (!props.account.id) {
    error.value = 'Account ID is missing'
    return
  }

  isSubmitting.value = true
  error.value = null

  try {
    const { commands, unwrapResult } = await import('@/services/api')
    
    const result = await commands.updateAccount(
      props.account.id,
      name.value.trim(),
      description.value.trim() || null
    )
    
    const updatedAccount = unwrapResult(result)
    emit('save', updatedAccount)
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e)
    console.error('Failed to update account:', e)
  } finally {
    isSubmitting.value = false
  }
}

const handleClose = () => {
  if (!isSubmitting.value) {
    emit('close')
  }
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg p-6 w-full max-w-md mx-4">
      <h2 class="text-xl font-semibold mb-4">Edit Account</h2>
      
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- Account Type (readonly) -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Account Type
          </label>
          <input
            type="text"
            :value="account?.account_type"
            readonly
            class="w-full px-3 py-2 border border-gray-200 rounded-md bg-gray-50 text-gray-600 cursor-not-allowed"
          />
        </div>

        <!-- Parent Account (readonly) -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Parent Account
          </label>
          <input
            type="text"
            :value="parentAccountName"
            readonly
            class="w-full px-3 py-2 border border-gray-200 rounded-md bg-gray-50 text-gray-600 cursor-not-allowed"
          />
        </div>
        
        <!-- Account Name (editable) -->
        <div>
          <label for="account-name" class="block text-sm font-medium text-gray-700 mb-1">
            Account Name *
          </label>
          <input
            id="account-name"
            v-model="name"
            type="text"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            :disabled="isSubmitting"
          />
        </div>
        
        <!-- Description (editable) -->
        <div>
          <label for="account-description" class="block text-sm font-medium text-gray-700 mb-1">
            Description
          </label>
          <textarea
            id="account-description"
            v-model="description"
            rows="3"
            placeholder="Optional description for this account"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            :disabled="isSubmitting"
          ></textarea>
        </div>
        
        <div v-if="error" class="text-red-600 text-sm bg-red-50 p-3 rounded-md">
          {{ error }}
        </div>
        
        <div class="flex gap-3 pt-4">
          <button
            type="button"
            @click="handleClose"
            class="flex-1 px-4 py-2 text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 transition-colors"
            :disabled="isSubmitting"
          >
            Cancel
          </button>
          <button
            type="submit"
            class="flex-1 px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors disabled:opacity-50"
            :disabled="isSubmitting || !name.trim()"
          >
            {{ isSubmitting ? 'Saving...' : 'Save Changes' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>