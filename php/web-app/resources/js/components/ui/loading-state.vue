<script setup lang="ts">
import { cn } from '@/utils/cn'
import { Loader2Icon } from '@/components/ui/icons'

interface Props {
  size?: 'sm' | 'md' | 'lg'
  text?: string
  class?: string
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md',
  text: 'Loading...',
})

const sizeClasses = {
  sm: 'h-4 w-4',
  md: 'h-8 w-8',
  lg: 'h-12 w-12',
}

const textSizeClasses = {
  sm: 'text-xs',
  md: 'text-sm',
  lg: 'text-base',
}
</script>

<template>
  <div :class="cn('flex flex-col items-center justify-center gap-3 py-8', class)">
    <Loader2Icon :class="cn('text-muted-foreground animate-spin', sizeClasses[size])" />
    <p v-if="text" :class="cn('text-muted-foreground', textSizeClasses[size])">{{ text }}</p>
  </div>
</template>