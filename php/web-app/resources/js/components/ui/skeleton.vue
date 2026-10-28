<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'

interface Props {
  variant?: 'text' | 'circular' | 'rectangular' | 'card' | 'avatar'
  width?: string | number
  height?: string | number
  class?: string
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'text',
  width: '100%',
  height: '1rem',
})

const baseClasses = 'animate-pulse rounded bg-muted-200 dark:bg-muted-700'

const variantClasses = {
  text: 'h-4 w-full',
  circular: 'rounded-full',
  rectangular: 'rounded-lg',
  card: 'rounded-xl',
  avatar: 'rounded-full',
}
</script>

<template>
  <div 
    :class="cn(
      baseClasses,
      variantClasses[variant],
      class
    )"
    :style="{ width, height }"
    role="status"
    aria-label="Loading"
  />
</template>