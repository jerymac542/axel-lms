<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'

const avatarFallbackVariants = cva(
  'flex h-full w-full items-center justify-center rounded-full bg-muted'
)

interface Props extends VariantProps<typeof avatarFallbackVariants> {
  children?: string
}

const props = withDefaults(defineProps<Props>(), {})

const classes = computed(() => avatarFallbackVariants())
</script>

<template>
  <div :class="classes">
    <slot>{{ props.children }}</slot>
  </div>
</template>