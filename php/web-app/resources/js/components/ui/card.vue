<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'
import type { HTMLAttributes } from 'vue'

const cardVariants = cva(
  'rounded-xl border bg-card text-card-foreground shadow-sm',
  {
    variants: {
      variant: {
        default: '',
        outlined: 'border-2',
        elevated: 'shadow-lg',
        flat: 'shadow-none border-0 bg-transparent',
      },
      padding: {
        default: 'p-6',
        sm: 'p-4',
        lg: 'p-8',
        none: 'p-0',
      },
    },
    defaultVariants: {
      variant: 'default',
      padding: 'default',
    },
  }
)

interface Props extends VariantProps<typeof cardVariants> {
  as?: string
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  padding: 'default',
  as: 'div',
})

const classes = computed(() => cardVariants({
  variant: props.variant,
  padding: props.padding,
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