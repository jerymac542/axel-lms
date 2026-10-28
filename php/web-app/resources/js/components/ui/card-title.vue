<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'

const cardTitleVariants = cva(
  'text-2xl font-semibold leading-none tracking-tight'
)

interface Props extends VariantProps<typeof cardTitleVariants> {
  as?: string
}

const props = withDefaults(defineProps<Props>(), {
  as: 'h3',
})

const classes = computed(() => cardTitleVariants())
</script>

<template>
  <component
    :is="props.as"
    :class="classes"
  >
    <slot />
  </component>
</template>