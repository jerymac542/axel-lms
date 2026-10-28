<script setup lang="ts">
import { Header } from '@/components/layout/Header'
import { Sidebar } from '@/components/layout/Sidebar'
import { Footer } from '@/components/layout/Footer'
import { CourseProgress } from '@/components/course/CourseProgress'
import { CourseOutline } from '@/components/course/CourseOutline'
import { NotesPanel } from '@/components/course/NotesPanel'
import { DiscussionThread } from '@/components/course/DiscussionThread'
import { Slot } from 'vue'
import { ref, computed } from 'vue'

defineProps<{
  course: any
  enrollment: any
  currentModule?: any
  currentLesson?: any
}>()

const slots = useSlots()
const showOutline = ref(true)
const showNotes = ref(false)
const showDiscussion = ref(false)
const sidebarWidth = ref(320)
</script>

<template>
  <div class="min-h-screen bg-background">
    <!-- Header -->
    <Header />

    <!-- Main Layout -->
    <div class="flex pt-16">
      <!-- Course Sidebar (Outline) -->
      <aside
        v-show="showOutline"
        class="fixed lg:relative inset-y-0 left-0 z-40 w-80 bg-card border-r border-border overflow-y-auto transition-transform duration-300 lg:translate-x-0"
        :class="{ '-translate-x-full': !showOutline }"
        aria-label="Course outline"
      >
        <div class="flex flex-col h-full">
          <!-- Course Header -->
          <div class="p-4 border-b border-border bg-background/50 sticky top-0 z-10">
            <div class="flex items-center justify-between mb-3">
              <Link :href="`/courses/${course.id}`" class="flex items-center gap-2">
                <ChevronLeftIcon class="h-5 w-5 text-muted-foreground hover:text-foreground" />
                <span class="font-medium truncate">Back to Course</span>
              </Link>
              <button
                @click="showOutline = false"
                class="lg:hidden p-2 rounded-md text-muted-foreground hover:bg-accent"
                aria-label="Close outline"
              >
                <XIcon class="h-5 w-5" />
              </button>
            </div>
            <CourseProgress :course="course" :enrollment="enrollment" class="mt-2" />
          </div>

          <!-- Course Outline -->
          <div class="flex-1 overflow-y-auto p-3">
            <CourseOutline
              :course="course"
              :enrollment="enrollment"
              :current-module="currentModule"
              :current-lesson="currentLesson"
              @toggle-module="() => {}"
              @select-lesson="() => {}"
            />
          </div>

          <!-- Bottom Actions -->
          <div class="p-3 border-t border-border bg-background/50">
            <div class="flex gap-2">
              <button
                @click="showNotes = !showNotes"
                class="flex-1 flex items-center justify-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors"
                :class="showNotes ? 'bg-primary-600 text-primary-foreground' : 'text-muted-foreground hover:text-foreground hover:bg-accent'"
              >
                <FileTextIcon class="h-4 w-4" />
                <span class="hidden sm:inline">Notes</span>
              </button>
              <button
                @click="showDiscussion = !showDiscussion"
                class="flex-1 flex items-center justify-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors"
                :class="showDiscussion ? 'bg-primary-600 text-primary-foreground' : 'text-muted-foreground hover:text-foreground hover:bg-accent'"
              >
                <MessageSquareIcon class="h-4 w-4" />
                <span class="hidden sm:inline">Discuss</span>
              </button>
            </div>
          </div>
        </div>
      </aside>

      <!-- Mobile Outline Toggle -->
      <button
        v-show="!showOutline"
        @click="showOutline = true"
        class="fixed bottom-4 left-4 z-50 lg:hidden rounded-full bg-primary-600 p-3 text-primary-foreground shadow-lg hover:bg-primary-700 transition-colors"
        aria-label="Open course outline"
      >
        <MenuIcon class="h-6 w-6" />
      </button>

      <!-- Main Content Area -->
      <main class="flex-1 min-w-0 lg:ml-0">
        <!-- Course Header Bar -->
        <div class="sticky top-16 z-30 bg-background/80 backdrop-blur-lg border-b border-border px-4 py-3">
          <div class="container mx-auto flex items-center justify-between gap-4">
            <div class="flex items-center gap-3 min-w-0">
              <button
                @click="showOutline = true"
                class="lg:hidden p-2 rounded-md text-muted-foreground hover:bg-accent"
                aria-label="Open outline"
              >
                <MenuIcon class="h-5 w-5" />
              </button>
              <div class="flex-1 min-w-0">
                <h1 class="text-lg font-semibold truncate">{{ course?.title }}</h1>
                <p class="text-sm text-muted-foreground truncate">{{ currentLesson?.title || 'Select a lesson to continue learning' }}</p>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <slot name="header-actions" />
            </div>
          </div>
        </div>

        <!-- Lesson Content -->
        <div class="container mx-auto px-4 py-6 max-w-4xl">
          <slot />
        </div>

        <!-- Side Panels (Notes/Discussion) -->
        <aside
          v-show="showNotes || showDiscussion"
          class="fixed inset-y-0 right-0 z-40 w-96 bg-card border-l border-border overflow-y-auto transition-transform duration-300 translate-x-full lg:translate-x-0"
          :class="{ 'translate-x-0': showNotes || showDiscussion }"
        >
          <div class="flex flex-col h-full">
            <div class="flex items-center justify-between p-3 border-b border-border bg-background/50 sticky top-16 z-10">
              <Tabs default-value="notes" class="flex-1">
                <TabsList class="h-8">
                  <TabsTrigger value="notes" @click="showNotes = true; showDiscussion = false" :disabled="!showNotes && !showDiscussion">
                    <FileTextIcon class="h-4 w-4 mr-2" /> Notes
                    <Badge v-if="notesCount > 0" class="ml-2">{{ notesCount }}</Badge>
                  </TabsTrigger>
                  <TabsTrigger value="discussion" @click="showDiscussion = true; showNotes = false" :disabled="!showNotes && !showDiscussion">
                    <MessageSquareIcon class="h-4 w-4 mr-2" /> Discuss
                    <Badge v-if="discussionCount > 0" class="ml-2">{{ discussionCount }}</Badge>
                  </TabsTrigger>
                </TabsList>
              </Tabs>
              <button
                @click="showNotes = false; showDiscussion = false"
                class="p-2 rounded-md text-muted-foreground hover:bg-accent"
                aria-label="Close panel"
              >
                <XIcon class="h-5 w-5" />
              </button>
            </div>

            <div class="flex-1 overflow-y-auto">
              <NotesPanel
                v-show="showNotes"
                :course-id="course.id"
                :lesson-id="currentLesson?.id"
                class="p-4"
              />
              <DiscussionThread
                v-show="showDiscussion"
                :course-id="course.id"
                :lesson-id="currentLesson?.id"
                class="p-4"
              />
            </div>
          </div>
        </aside>
      </main>

      <!-- Footer -->
      <Footer class="lg:hidden" />
    </div>
  </div>
</template>

<script>
const ChevronLeftIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 19l-7-7 7-7' })]) }})
const XIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M6 18L18 6M6 6l12 12' })]) }})
const MenuIcon = defineComponent({ render() { return h('svg', { class: 'h-6 w-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 6h16M4 12h16M4 18h16' })]) }})
const FileTextIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })]) }})
const MessageSquareIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 12h4M8 12l2-2m0 0l2 2m-2-2v8m-8 0v8m0-8h8m-8 0h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949M3 12c0 4.418 4.03 8 9 8a9.863 9.863 0 014.255-.949' })]) }})
</script>