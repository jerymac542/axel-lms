<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'

const avatarImageVariants = cva(
  'aspect-square h-full w-full object-cover'
)

interface Props extends VariantProps<typeof avatarImageVariants> {
  src: string
  alt?: string
}

const props = withDefaults(defineProps<Props>(), {})

const classes = computed(() => avatarImageVariants())
</script>

<template>
  <img
    :src="props.src"
    :alt="props.alt || 'Avatar'"
    :class="classes"
  />
</template>