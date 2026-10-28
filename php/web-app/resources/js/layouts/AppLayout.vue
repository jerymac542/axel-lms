<script setup lang="ts">
import { Header } from '@/components/layout/Header'
import { Sidebar } from '@/components/layout/Sidebar'
import { Footer } from '@/components/layout/Footer'
import { Slot } from 'vue'

defineProps<{
  title?: string
  description?: string
}>()

const slots = useSlots()
</script>

<template>
  <div class="min-h-screen bg-background">
    <!-- Header -->
    <Header />

    <!-- Main Content Area -->
    <div class="flex pt-16">
      <!-- Sidebar -->
      <Sidebar />

      <!-- Main Content -->
      <main
        class="flex-1 min-w-0 lg:ml-64 transition-all duration-300"
        :class="{ 'ml-16': $attrs.sidebarCollapsed }"
      >
        <!-- Page Header -->
        <div v-if="title || description || $slots.header" class="border-b border-border bg-background/50 px-6 py-4">
          <div class="container mx-auto">
            <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
              <div>
                <h1 v-if="title" class="text-2xl font-semibold tracking-tight">{{ title }}</h1>
                <p v-if="description" class="mt-1 text-sm text-muted-foreground">{{ description }}</p>
              </div>
              <div v-if="$slots.header" class="flex items-center gap-2">
                <slot name="header" />
              </div>
            </div>
          </div>
        </div>

        <!-- Page Content -->
        <div class="container mx-auto px-4 py-6">
          <slot />
        </div>
      </main>
    </div>

    <!-- Footer -->
    <Footer />
  </div>
</template>