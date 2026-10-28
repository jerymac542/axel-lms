<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'
import { Slot, type HTMLAttributes } from 'vue'

const buttonVariants = cva(
  'inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-lg text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
  {
    variants: {
      variant: {
        default: 'bg-primary-600 text-primary-foreground hover:bg-primary-700 shadow-sm hover:shadow-md',
        destructive: 'bg-destructive-600 text-destructive-foreground hover:bg-destructive-700 shadow-sm hover:shadow-md',
        outline: 'border border-input bg-transparent hover:bg-accent hover:text-accent-foreground',
        secondary: 'bg-secondary-100 text-secondary-900 hover:bg-secondary-200 dark:bg-secondary-800 dark:text-secondary-100 dark:hover:bg-secondary-700',
        ghost: 'hover:bg-accent hover:text-accent-foreground',
        link: 'text-primary-600 dark:text-primary-400 underline-offset-4 hover:underline',
        success: 'bg-success-600 text-success-foreground hover:bg-success-700',
        warning: 'bg-warning-600 text-warning-foreground hover:bg-warning-700',
      },
      size: {
        default: 'h-10 px-4 py-2',
        sm: 'h-8 rounded-md px-3 text-xs',
        lg: 'h-12 rounded-lg px-8 text-lg',
        xl: 'h-14 rounded-xl px-10 text-xl',
        icon: 'h-10 w-10',
        'icon-sm': 'h-8 w-8',
        'icon-lg': 'h-12 w-12',
      },
      loading: {
        true: 'opacity-70 cursor-wait',
        false: '',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
      loading: false,
    },
  }
)

interface Props extends VariantProps<typeof buttonVariants> {
  asChild?: boolean
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  size: 'default',
  loading: false,
  type: 'button',
  disabled: false,
  asChild: false,
})

const slots = useSlots()

const classes = computed(() => buttonVariants({
  variant: props.variant,
  size: props.size,
  loading: props.loading,
}))

const isLoading = computed(() => props.loading || props.disabled)
</script>

<template>
  <button
    :class="classes"
    :type="props.type"
    :disabled="isLoading"
    :aria-busy="props.loading"
    :aria-disabled="props.disabled"
  >
    <span v-if="props.loading" class="flex items-center gap-2">
      <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
      </svg>
      <span>{{ $slots.loading?.() || 'Loading...' }}</span>
    </span>
    <span v-else class="flex items-center gap-2">
      <slot />
    </span>
  </button>
</template>