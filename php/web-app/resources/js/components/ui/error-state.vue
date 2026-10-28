<script setup lang="ts">
import { cn } from '@/utils/cn'
import { Button } from '@/components/ui/button'
import { AlertCircleIcon, RefreshCwIcon } from '@/components/ui/icons'

interface Props {
  title?: string
  description?: string
  error?: Error | string
  onRetry?: () => void
  class?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Something went wrong',
  description: 'Please try again or contact support if the problem persists.',
})
</script>

<template>
  <div :class="cn('flex flex-col items-center justify-center text-center py-12 px-4', class)">
    <div class="mb-4 p-3 rounded-full bg-destructive-100 dark:bg-destructive-900/30">
      <AlertCircleIcon class="h-8 w-8 text-destructive-600 dark:text-destructive-400" />
    </div>
    
    <h3 class="text-lg font-semibold text-foreground mb-2">{{ title }}</h3>
    
    <p class="text-sm text-muted-foreground max-w-sm mb-6">{{ description }}</p>
    
    <div v-if="error" class="mb-4 p-3 rounded-lg bg-muted-100 dark:bg-muted-800 text-sm text-muted-foreground max-w-md text-left">
      <pre class="whitespace-pre-wrap">{{ typeof error === 'string' ? error : error.message }}</pre>
    </div>
    
    <Button 
      v-if="onRetry" 
      @click="onRetry"
      class="gap-2"
    >
      <RefreshCwIcon class="h-4 w-4" />
      Try Again
    </Button>
  </div>
</template>