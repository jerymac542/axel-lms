<script setup lang="ts">
import { Header } from '@/components/layout/Header'
import { Sidebar } from '@/components/layout/Sidebar'
import { Slot } from 'vue'
import { useAuth } from '@/composables/useAuth'
import { cn } from '@/utils/cn'

const auth = useAuth()

defineProps<{
  title?: string
  description?: string
}>()

const slots = useSlots()

// Check admin access
if (!auth.hasRole('admin')) {
  // Redirect or show unauthorized - handled by middleware
}
</script>

<template>
  <div class="min-h-screen bg-background">
    <Header />

    <div class="flex pt-16">
      <Sidebar />

      <main class="flex-1 min-w-0 lg:ml-64 transition-all duration-300">
        <!-- Admin Header -->
        <div class="border-b border-border bg-background/50 px-6 py-4">
          <div class="container mx-auto max-w-full">
            <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
              <div class="flex items-center gap-3">
                <div class="p-2 rounded-lg bg-destructive-100 dark:bg-destructive-900/30">
                  <ShieldIcon class="h-5 w-5 text-destructive-600 dark:text-destructive-400" />
                </div>
                <div>
                  <h1 v-if="title" class="text-xl font-semibold tracking-tight">{{ title }}</h1>
                  <p v-if="description" class="text-sm text-muted-foreground">{{ description }}</p>
                  <p class="text-xs text-muted-foreground">Administration Panel</p>
                </div>
              </div>
              <div v-if="$slots.header" class="flex items-center gap-2">
                <slot name="header" />
              </div>
            </div>
          </div>
        </div>

        <!-- Admin Content -->
        <div class="container mx-auto max-w-full px-4 py-6">
          <slot />
        </div>
      </main>
    </div>
  </div>
</template>

<script>
const ShieldIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
</script>