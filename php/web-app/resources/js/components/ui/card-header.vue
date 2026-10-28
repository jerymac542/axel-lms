<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'

const cardHeaderVariants = cva(
  'flex flex-col space-y-1.5 p-6'
)

interface Props extends VariantProps<typeof cardHeaderVariants> {
  as?: string
}

const props = withDefaults(defineProps<Props>(), {
  as: 'div',
})

const classes = computed(() => cardHeaderVariants())
</script>

<template>
  <component
    :is="props.as"
    :class="classes"
  >
    <slot />
  </component>
</template>