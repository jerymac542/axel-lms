<script setup lang="ts">
import { Header } from '@/components/layout/Header'
import { Footer } from '@/components/layout/Footer'
import { Slot } from 'vue'

defineProps<{
  title?: string
  description?: string
  showHeader?: boolean
  showFooter?: boolean
}>()

const slots = useSlots()
</script>

<template>
  <div class="min-h-screen bg-background flex flex-col">
    <!-- Header -->
    <Header v-if="showHeader !== false" />

    <!-- Main Content -->
    <main class="flex-1 w-full max-w-screen-md mx-auto px-4 py-12 lg:py-16">
      <slot />
    </main>

    <!-- Footer -->
    <Footer v-if="showFooter !== false" />
  </div>
</template>