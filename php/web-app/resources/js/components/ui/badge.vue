<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'
import type { HTMLAttributes } from 'vue'

const badgeVariants = cva(
  'inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
  {
    variants: {
      variant: {
        default: 'border-transparent bg-primary-100 text-primary-800 dark:bg-primary-900 dark:text-primary-100',
        secondary: 'border-transparent bg-secondary-100 text-secondary-800 dark:bg-secondary-800 dark:text-secondary-100',
        destructive: 'border-transparent bg-destructive-100 text-destructive-800 dark:bg-destructive-900 dark:text-destructive-100',
        success: 'border-transparent bg-success-100 text-success-800 dark:bg-success-900 dark:text-success-100',
        warning: 'border-transparent bg-warning-100 text-warning-800 dark:bg-warning-900 dark:text-warning-100',
        info: 'border-transparent bg-info-100 text-info-800 dark:bg-info-900 dark:text-info-100',
        outline: 'text-foreground',
      },
      size: {
        default: 'px-2.5 py-0.5 text-xs',
        sm: 'px-2 py-0.5 text-[10px]',
        lg: 'px-3 py-1 text-sm',
      },
      dot: {
        true: 'relative pl-5 before:absolute before:left-1.5 before:top-1/2 before:-translate-y-1/2 before:h-1.5 before:w-1.5 before:rounded-full before:bg-current',
        false: '',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
      dot: false,
    },
  }
)

interface Props extends VariantProps<typeof badgeVariants> {
  as?: string
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  size: 'default',
  dot: false,
  as: 'span',
})

const classes = computed(() => badgeVariants({
  variant: props.variant,
  size: props.size,
  dot: props.dot,
}))
</script>

<template>
  <component
    :is="props.as"
    :class="classes"
  >
    <slot />
  </component>
</template>