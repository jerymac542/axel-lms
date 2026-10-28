<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Link } from '@inertiajs/vue3'
import { Button } from '@/components/ui/button'
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Progress } from '@/components/ui/progress'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from '@/components/ui/accordion'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { Separator } from '@/components/ui/separator'
import { Skeleton } from '@/components/ui/skeleton'
import { useAuth } from '@/composables/useAuth'
import { useToast } from '@/composables/useToast'
import { 
  ChevronLeftIcon, ChevronRightIcon, PlayIcon, PauseIcon, 
  Volume2Icon, VolumeXIcon, FullscreenIcon, MinimizeIcon,
  BookOpenIcon, MessageSquareIcon, FileTextIcon, BookmarkIcon,
  DownloadIcon, SettingsIcon, MoreHorizontalIcon, 
  CheckCircleIcon, ClockIcon, FlagIcon, HelpCircleIcon,
  ArrowLeftIcon, ArrowRightIcon, ExpandIcon, ChevronDownIcon
} from '@/components/ui/icons'
import { formatDuration, formatRelativeTime } from '@/utils/cn'
import { LessonPlayer } from '@/components/course/LessonPlayer'
import { CourseProgress } from '@/components/course/CourseProgress'
import { NotesPanel } from '@/components/course/NotesPanel'
import { DiscussionThread } from '@/components/course/DiscussionThread'

const route = useRoute()
const router = useRouter()
const auth = useAuth()
const toast = useToast()

// Mock data - replace with actual API
const course = ref({
  id: route.params.courseId as string,
  title: 'Advanced React Patterns',
  description: 'Master advanced React patterns including hooks, context, and performance optimization.',
  thumbnail_url: '/courses/react-advanced.jpg',
  progress: 65,
  instructor: { id: '1', name: 'Sarah Chen', avatar_url: '/avatars/sarah.jpg' },
  duration: '12 hours',
  modules: [
    {
      id: 'm1',
      title: 'Module 1: Advanced Hooks',
      description: 'Deep dive into custom hooks, useReducer, and useCallback',
      lessons: [
        { id: 'l1', title: 'Introduction to Advanced Hooks', type: 'video', duration: '15:30', completed: true, duration_seconds: 930 },
        { id: 'l2', title: 'useReducer vs useState', type: 'video', duration: '22:45', completed: true, duration_seconds: 1365 },
        { id: 'l3', title: 'Custom Hooks Best Practices', type: 'video', duration: '18:20', completed: true, duration_seconds: 1100 },
        { id: 'l4', title: 'Performance with useCallback', type: 'video', duration: '25:10', completed: false, duration_seconds: 1510 },
        { id: 'l5', title: 'Hooks Quiz', type: 'quiz', duration: '10 min', completed: false },
      ],
    },
    {
      id: 'm2',
      title: 'Module 2: Context & State Management',
      description: 'React Context API, useContext, and global state patterns',
      lessons: [
        { id: 'l6', title: 'Context API Fundamentals', type: 'video', duration: '20:00', completed: false, duration_seconds: 1200 },
        { id: 'l7', title: 'Global State with Context', type: 'video', duration: '28:30', completed: false, duration_seconds: 1710 },
        { id: 'l8', title: 'Avoiding Context Pitfalls', type: 'reading', duration: '15 min', completed: false },
        { id: 'l9', title: 'State Management Assignment', type: 'assignment', duration: '45 min', completed: false },
      ],
    },
    {
      id: 'm3',
      title: 'Module 3: Performance Optimization',
      description: 'Memoization, virtualization, and bundle optimization',
      lessons: [
        { id: 'l10', title: 'React.memo and useMemo', type: 'video', duration: '22:00', completed: false, duration_seconds: 1320 },
        { id: 'l11', title: 'Code Splitting & Lazy Loading', type: 'video', duration: '19:45', completed: false, duration_seconds: 1185 },
        { id: 'l12', title: 'Virtual Lists with react-window', type: 'video', duration: '24:30', completed: false, duration_seconds: 1470 },
        { id: 'l13', title: 'Final Project: Dashboard App', type: 'project', duration: '2 hours', completed: false },
      ],
    },
  ],
})

const enrollment = ref({
  id: 'enr-1',
  course_id: course.value.id,
  user_id: 'current-user',
  role: 'student',
  status: 'active',
  progress: 65,
  score: 87.5,
  enrolled_at: '2024-01-01T00:00:00Z',
})

const currentLesson = ref(course.value.modules[0].lessons[3]) // 4th lesson (not completed)
const currentModule = ref(course.value.modules[0])

const showOutline = ref(true)
const showNotes = ref(false)
const showDiscussion = ref(false)
const isFullscreen = ref(false)
const playbackSpeed = ref(1)
const volume = ref(1)
const isMuted = ref(false)
const videoProgress = ref(0)
const videoDuration = ref(currentLesson.value.duration_seconds || 0)

const tabs = [
  { value: 'content', label: 'Content', icon: BookOpenIcon },
  { value: 'notes', label: 'Notes', icon: FileTextIcon },
  { value: 'discussion', label: 'Discuss', icon: MessageSquareIcon },
]

const activeTab = ref('content')

const totalLessons = computed(() => 
  course.value.modules.reduce((sum, m) => sum + m.lessons.length, 0)
)

const completedLessons = computed(() => 
  course.value.modules.reduce((sum, m) => sum + m.lessons.filter(l => l.completed).length, 0)
)

const currentLessonIndex = computed(() => {
  let index = 0
  for (const module of course.value.modules) {
    const lessonIndex = module.lessons.findIndex(l => l.id === currentLesson.value.id)
    if (lessonIndex !== -1) return index + lessonIndex
    index += module.lessons.length
  }
  return -1
})

const nextLesson = computed(() => {
  const idx = currentLessonIndex.value
  if (idx === -1 || idx >= totalLessons.value - 1) return null
  
  let count = 0
  for (const module of course.value.modules) {
    for (const lesson of module.lessons) {
      if (count === idx + 1) return lesson
      count++
    }
  }
  return null
})

const previousLesson = computed(() => {
  const idx = currentLessonIndex.value
  if (idx <= 0) return null
  
  let count = 0
  for (const module of course.value.modules) {
    for (const lesson of module.lessons) {
      if (count === idx - 1) return lesson
      count++
    }
  }
  return null
})

const toggleLessonComplete = (lessonId: string) => {
  for (const module of course.value.modules) {
    const lesson = module.lessons.find(l => l.id === lessonId)
    if (lesson) {
      lesson.completed = !lesson.completed
      // Update course progress
      updateProgress()
      break
    }
  }
}

const updateProgress = () => {
  course.value.progress = Math.round((completedLessons.value / totalLessons.value) * 100)
  enrollment.value.progress = course.value.progress
  
  // Save to backend
  // await axios.put(`/api/enrollments/${enrollment.value.id}/progress`, { progress: course.value.progress })
}

const selectLesson = (lesson: any, module: any) => {
  currentLesson.value = lesson
  currentModule.value = module
  showOutline.value = false
  activeTab.value = 'content'
}

const toggleFullscreen = () => {
  isFullscreen.value = !isFullscreen.value
  if (isFullscreen.value) {
    document.documentElement.requestFullscreen?.()
  } else {
    document.exitFullscreen?.()
  }
}

const handleVideoTimeUpdate = (currentTime: number, duration: number) => {
  videoProgress.value = currentTime
  videoDuration.value = duration
}

const handleVideoEnded = () => {
  // Auto-complete video lessons
  if (currentLesson.value.type === 'video' && !currentLesson.value.completed) {
    toggleLessonComplete(currentLesson.value.id)
  }
  
  // Auto-advance to next lesson
  if (nextLesson.value) {
    setTimeout(() => {
      selectLesson(nextLesson.value, currentModule.value)
    }, 2000)
  }
}

const formatTime = (seconds: number) => {
  const mins = Math.floor(seconds / 60)
  const secs = Math.floor(seconds % 60)
  return `${mins}:${secs.toString().padStart(2, '0')}`
}
</script>

<template>
  <CourseLayout 
    :course="course" 
    :enrollment="enrollment"
    :current-module="currentModule"
    :current-lesson="currentLesson"
  >
    <template #header-actions>
      <div class="flex items-center gap-2">
        <Button variant="outline" size="icon" @click="toggleFullscreen" :aria-label="isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'">
          <FullscreenIcon v-if="!isFullscreen" class="h-4 w-4" />
          <MinimizeIcon v-else class="h-4 w-4" />
        </Button>
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button variant="outline" size="icon">
              <MoreHorizontalIcon class="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem @click="toggleLessonComplete(currentLesson.id)">
              <CheckCircleIcon class="h-4 w-4" />
              {{ currentLesson.completed ? 'Mark Incomplete' : 'Mark Complete' }}
            </DropdownMenuItem>
            <DropdownMenuItem>
              <BookmarkIcon class="h-4 w-4" />
              Bookmark Lesson
            </DropdownMenuItem>
            <DropdownMenuItem v-if="currentLesson.type === 'video'">
              <DownloadIcon class="h-4 w-4" />
              Download Video
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
              <SettingsIcon class="h-4 w-4" />
              Playback Speed: {{ playbackSpeed }}x
            </DropdownMenuItem>
            <DropdownMenuItem>
              <HelpCircleIcon class="h-4 w-4" />
              Report Issue
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </template>

    <!-- Video Player Area -->
    <div class="aspect-video bg-muted relative rounded-xl overflow-hidden mb-6">
      <LessonPlayer
        :lesson="currentLesson"
        :course="course"
        :enrollment="enrollment"
        :autoplay="true"
        @time-update="handleVideoTimeUpdate"
        @ended="handleVideoEnded"
      />
      
      <!-- Progress Bar Overlay -->
      <div v-if="currentLesson.type === 'video'" class="absolute bottom-0 left-0 right-0 h-2 bg-black/50">
        <div 
          class="h-full bg-primary-600 transition-all duration-100" 
          :style="{ width: videoDuration > 0 ? (videoProgress / videoDuration) * 100 + '%' : '0%' }"
        />
      </div>
    </div>

    <!-- Lesson Info -->
    <div class="mb-6">
      <div class="flex items-center justify-between mb-3">
        <div class="flex items-center gap-3">
          <Badge variant="secondary">{{ currentModule.title }}</Badge>
          <Badge :variant="currentLesson.completed ? 'success' : 'outline'">
            {{ currentLesson.completed ? 'Completed' : 'In Progress' }}
          </Badge>
        </div>
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
          <ClockIcon class="h-4 w-4" />
          <span>{{ currentLesson.duration }}</span>
        </div>
      </div>
      
      <h1 class="text-2xl font-bold text-foreground mb-2">{{ currentLesson.title }}</h1>
      <p class="text-muted-foreground mb-4">{{ currentLesson.description || 'No description available' }}</p>

      <!-- Progress Bar -->
      <div class="flex items-center gap-3">
        <Progress :value="course.progress" class="flex-1 max-w-md" />
        <span class="text-sm font-medium text-muted-foreground w-12 text-right">{{ course.progress }}%</span>
      </div>
    </div>

    <!-- Main Content Tabs -->
    <Tabs default-value="content" class="w-full">
      <TabsList class="grid w-full grid-cols-3">
        <TabsTrigger v-for="tab in tabs" :key="tab.value" :value="tab.value">
          <component :is="tab.icon" class="h-4 w-4 mr-2" />
          {{ tab.label }}
        </TabsTrigger>
      </TabsList>

      <TabsContent value="content" class="mt-6">
        <LessonPlayer
          :lesson="currentLesson"
          :course="course"
          :enrollment="enrollment"
          :autoplay="true"
          @time-update="handleVideoTimeUpdate"
          @ended="handleVideoEnded"
        />
      </TabsContent>

      <TabsContent value="notes" class="mt-6">
        <NotesPanel :course-id="course.id" :lesson-id="currentLesson.id" />
      </TabsContent>

      <TabsContent value="discussion" class="mt-6">
        <DiscussionThread :course-id="course.id" :lesson-id="currentLesson.id" />
      </TabsContent>
    </Tabs>

    <!-- Navigation -->
    <div class="mt-8 flex items-center justify-between pt-6 border-t border-border">
      <Button 
        variant="outline" 
        :disabled="!previousLesson" 
        @click="previousLesson && selectLesson(previousLesson, currentModule)"
        class="gap-2"
      >
        <ArrowLeftIcon class="h-4 w-4" />
        <span>Previous</span>
      </Button>

      <div class="flex items-center gap-4">
        <span class="text-sm text-muted-foreground">
          Lesson {{ currentLessonIndex + 1 }} of {{ totalLessons }}
        </span>
      </div>

      <Button 
        :disabled="!nextLesson" 
        @click="nextLesson && selectLesson(nextLesson, currentModule)"
        class="gap-2"
      >
        <span>Next</span>
        <ArrowRightIcon class="h-4 w-4" />
      </Button>
    </div>
  </CourseLayout>
</template>

<script>
const ChevronLeftIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 19l-7-7 7-7' })]) }})
const ChevronRightIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 5l7 7-7 7' })]) }})
const PlayIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M8 5v14l11-7z' })]) }})
const PauseIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '6', y: '4', width: '4', height: '16' }), h('rect', { x: '14', y: '4', width: '4', height: '16' })]) }})
const Volume2Icon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('polygon', { points: '11 5 6 9 2 9 2 15 6 15 11 19 11 5' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19.07 4.93a10 10 0 010 14.14M15.54 8.46a5 5 0 010 7.07' })]) }})
const VolumeXIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('polygon', { points: '11 5 6 9 2 9 2 15 6 15 11 19 11 5' }), h('line', { x1: '23', y1: '9', x2: '17', y2: '15', 'stroke-width': '2', 'stroke-linecap': 'round', 'stroke-linejoin': 'round' }), h('line', { x1: '17', y1: '9', x2: '23', y2: '15', 'stroke-width': '2', 'stroke-linecap': 'round', 'stroke-linejoin': 'round' })]) }})
const FullscreenIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 8V4m0 0h4M4 8h4m-4 8v4m0 0h4m-4 0v-4m8 0H4m0 0h4m0 0v-4m8 8H4m0 0h4m0 0v4m8 0h4m0 0v-4' })]) }})
const MinimizeIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 8V4m0 0h4M4 8h4m-4 8v4m0 0h4m-4 0v-4m8 0H4m0 0h4m0 0v-4m8 8H4m0 0h4m0 0v4m8 0h4m0 0v-4' })]) }})
const BookOpenIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const MessageSquareIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 12h4M8 12l2-2m0 0l2 2m-2-2v8m-8 0v8m0-8h8m-8 0h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949M3 12c0 4.418 4.03 8 9 8a9.863 9.863 0 014.255-.949' })]) }})
const FileTextIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })]) }})
const BookmarkIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z' })]) }})
const DownloadIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M7 10l5 5 5-5M17 10v10' })]) }})
const SettingsIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const MoreHorizontalIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('circle', { cx: '12', cy: '12', r: '1', 'stroke-width': '2' }), h('circle', { cx: '19', cy: '12', r: '1', 'stroke-width': '2' }), h('circle', { cx: '5', cy: '12', r: '1', 'stroke-width': '2' })]) }})
const CheckCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const ClockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const FlagIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1v12z' })]) }})
const HelpCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const ArrowLeftIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 19l-7-7 7-7' })]) }})
const ArrowRightIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 5l7 7-7 7' })]) }})
const ExpandIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 8V4m0 0h4M4 8h4m-4 8v4m0 0h4m-4 0v-4m8 0H4m0 0h4m0 0v-4m8 8H4m0 0h4m0 0v4m8 0h4m0 0v-4' })]) }})
const ChevronDownIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 9l-7 7-7-7' })]) }})
</script>