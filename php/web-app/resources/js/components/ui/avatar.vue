<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'
import type { HTMLAttributes } from 'vue'

const avatarVariants = cva(
  'relative flex shrink-0 overflow-hidden rounded-full',
  {
    variants: {
      size: {
        xs: 'h-6 w-6 text-xs',
        sm: 'h-8 w-8 text-sm',
        default: 'h-10 w-10 text-base',
        lg: 'h-12 w-12 text-lg',
        xl: 'h-16 w-16 text-xl',
        '2xl': 'h-20 w-20 text-2xl',
        '3xl': 'h-24 w-24 text-3xl',
      },
      shape: {
        circle: 'rounded-full',
        square: 'rounded-xl',
      },
    },
    defaultVariants: {
      size: 'default',
      shape: 'circle',
    },
  }
)

interface Props extends VariantProps<typeof avatarVariants> {
  src?: string
  alt?: string
  fallback?: string
  fallbackVariant?: 'initials' | 'icon' | 'none'
  status?: 'online' | 'offline' | 'busy' | 'away'
  statusPosition?: 'bottom-right' | 'bottom-left' | 'top-right' | 'top-left'
}

const props = withDefaults(defineProps<Props>(), {
  size: 'default',
  shape: 'circle',
  fallbackVariant: 'initials',
  status: 'offline',
  statusPosition: 'bottom-right',
})

const classes = computed(() => avatarVariants({
  size: props.size,
  shape: props.shape,
}))

const statusColors = {
  online: 'bg-success-500',
  offline: 'bg-muted-400',
  busy: 'bg-destructive-500',
  away: 'bg-warning-500',
}

const statusSizes = {
  xs: 'h-1.5 w-1.5',
  sm: 'h-2 w-2',
  default: 'h-2.5 w-2.5',
  lg: 'h-3 w-3',
  xl: 'h-3.5 w-3.5',
  '2xl': 'h-4 w-4',
  '3xl': 'h-5 w-5',
}

const getInitials = (name: string) => {
  return name
    .split(' ')
    .map(part => part[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)
}
</script>

<template>
  <div
    :class="classes"
    :style="{ '--avatar-size': sizeMap[props.size] }"
  >
    <img
      v-if="props.src"
      :src="props.src"
      :alt="props.alt || 'Avatar'"
      class="aspect-square h-full w-full object-cover"
      @error="handleImageError"
    />
    <div
      v-else
      class="flex h-full w-full items-center justify-center rounded-full bg-muted"
    >
      <span v-if="props.fallbackVariant === 'initials' && props.fallback" class="font-medium">
        {{ getInitials(props.fallback) }}
      </span>
      <span v-else-if="props.fallbackVariant === 'icon'" class="text-muted-foreground">
        <slot name="icon">
          <svg class="h-1/2 w-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
        </slot>
      </span>
    </div>

    <div
      v-if="props.status !== 'offline'"
      :class="[
        'absolute rounded-full border-2 border-background',
        statusColors[props.status],
        statusSizes[props.size],
        statusPositionClasses[props.statusPosition],
      ]"
      :aria-label="`Status: ${props.status}`"
    />
  </div>
</template>

<script>
export default {
  data() {
    return {
      imageError: false,
      sizeMap: {
        xs: '24px',
        sm: '32px',
        default: '40px',
        lg: '48px',
        xl: '64px',
        '2xl': '80px',
        '3xl': '96px',
      },
      statusColors: {
        online: 'bg-success-500',
        offline: 'bg-muted-400',
        busy: 'bg-destructive-500',
        away: 'bg-warning-500',
      },
      statusSizes: {
        xs: 'h-1.5 w-1.5',
        sm: 'h-2 w-2',
        default: 'h-2.5 w-2.5',
        lg: 'h-3 w-3',
        xl: 'h-3.5 w-3.5',
        '2xl': 'h-4 w-4',
        '3xl': 'h-5 w-5',
      },
      statusPositionClasses: {
        'bottom-right': 'bottom-0 right-0',
        'bottom-left': 'bottom-0 left-0',
        'top-right': 'top-0 right-0',
        'top-left': 'top-0 left-0',
      },
    }
  },
  methods: {
    handleImageError() {
      this.imageError = true
    },
  },
}
</script>