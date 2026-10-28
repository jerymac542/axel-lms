<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'
import { Button } from '@/components/ui/button'

interface Props {
  title: string
  description?: string
  icon?: any
  action?: {
    label: string
    onClick: () => void
    variant?: 'default' | 'outline' | 'secondary'
  }
  class?: string
}

const props = withDefaults(defineProps<Props>(), {
  icon: null,
})
</script>

<template>
  <div :class="cn('flex flex-col items-center justify-center text-center py-12 px-4', class)">
    <div class="mb-4 p-3 rounded-full bg-muted-100 dark:bg-muted-800">
      <component v-if="icon" :is="icon" class="h-8 w-8 text-muted-foreground" />
      <div v-else class="h-8 w-8 rounded-full bg-muted-200 dark:bg-muted-700" />
    </div>
    
    <h3 class="text-lg font-semibold text-foreground mb-2">{{ title }}</h3>
    
    <p v-if="description" class="text-sm text-muted-foreground max-w-sm mb-6">
      {{ description }}
    </p>
    
    <Button 
      v-if="action" 
      :variant="action.variant || 'default'" 
      @click="action.onClick"
    >
      {{ action.label }}
    </Button>
  </div>
</template>