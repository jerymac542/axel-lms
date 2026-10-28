<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'

const cardFooterVariants = cva(
  'flex items-center p-6 pt-0'
)

interface Props extends VariantProps<typeof cardFooterVariants> {
  as?: string
}

const props = withDefaults(defineProps<Props>(), {
  as: 'div',
})

const classes = computed(() => cardFooterVariants())
</script>

<template>
  <component
    :is="props.as"
    :class="classes"
  >
    <slot />
  </component>
</template>