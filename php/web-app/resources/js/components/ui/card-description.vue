<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'

const cardDescriptionVariants = cva(
  'text-sm text-muted-foreground'
)

interface Props extends VariantProps<typeof cardDescriptionVariants> {
  as?: string
}

const props = withDefaults(defineProps<Props>(), {
  as: 'p',
})

const classes = computed(() => cardDescriptionVariants())
</script>

<template>
  <component
    :is="props.as"
    :class="classes"
  >
    <slot />
  </component>
</template>