<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'
import type { HTMLAttributes } from 'vue'

const labelVariants = cva(
  'text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70'
)

interface Props extends VariantProps<typeof labelVariants> {
  for?: string
}

const props = withDefaults(defineProps<Props>(), {})

const classes = computed(() => labelVariants())
</script>

<template>
  <label
    :class="classes"
    :for="props.for"
  >
    <slot />
    <span v-if="$$slots.required" class="text-destructive-500 ml-1" aria-hidden="true">*</span>
  </label>
</template>