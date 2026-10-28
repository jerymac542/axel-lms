<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Link } from '@inertiajs/vue3'
import { Button } from '@/components/ui/button'
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { Progress } from '@/components/ui/progress'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Skeleton } from '@/components/ui/skeleton'
import { useAuth } from '@/composables/useAuth'
import { useToast } from '@/composables/useToast'
import { 
  BookOpenIcon, ClockIcon, TrophyIcon, AwardIcon, 
  CalendarIcon, FlameIcon, TrendingUpIcon, TargetIcon,
  PlusIcon, ArrowRightIcon, PlayIcon, BookmarkIcon,
  MoreHorizontalIcon, FilterIcon, SearchIcon
} from '@/components/ui/icons'
import { formatRelativeTime, cn } from '@/utils/cn'
import { CourseCard } from '@/components/course/CourseCard'
import { DashboardStats } from '@/components/dashboard/DashboardStats'
import { ContinueLearning } from '@/components/dashboard/ContinueLearning'
import { UpcomingDeadlines } from '@/components/dashboard/UpcomingDeadlines'
import { RecommendedCourses } from '@/components/dashboard/RecommendedCourses'
import { LearningStreak } from '@/components/dashboard/LearningStreak'
import { SkillProgress } from '@/components/dashboard/SkillProgress'

const router = useRouter()
const auth = useAuth()
const toast = useToast()

const activeTab = ref<'in-progress' | 'completed' | 'all'>('in-progress')
const isLoading = ref(false)
const searchQuery = ref('')

// Mock data - replace with actual API calls
const courses = ref([
  {
    id: '1',
    title: 'Advanced React Patterns',
    description: 'Master advanced React patterns including hooks, context, and performance optimization.',
    thumbnail_url: '/courses/react-advanced.jpg',
    progress: 65,
    status: 'in_progress',
    instructor: { name: 'Sarah Chen', avatar_url: '/avatars/sarah.jpg' },
    duration: '12 hours',
    lessons_completed: 8,
    total_lessons: 12,
    last_accessed: '2024-01-15T10:30:00Z',
    next_lesson: 'Custom Hooks Deep Dive',
  },
  {
    id: '2',
    title: 'Machine Learning Fundamentals',
    description: 'Introduction to ML concepts, algorithms, and practical applications.',
    thumbnail_url: '/courses/ml-fundamentals.jpg',
    progress: 100,
    status: 'completed',
    instructor: { name: 'Dr. Alex Kumar', avatar_url: '/avatars/alex.jpg' },
    duration: '20 hours',
    lessons_completed: 15,
    total_lessons: 15,
    completed_at: '2024-01-10T14:20:00Z',
    certificate_url: '/certificates/ml-fundamentals.pdf',
  },
  {
    id: '3',
    title: 'System Design Interview Prep',
    description: 'Master system design concepts for senior engineering interviews.',
    thumbnail_url: '/courses/system-design.jpg',
    progress: 30,
    status: 'in_progress',
    instructor: { name: 'Marcus Johnson', avatar_url: '/avatars/marcus.jpg' },
    duration: '8 hours',
    lessons_completed: 3,
    total_lessons: 10,
    last_accessed: '2024-01-12T09:15:00Z',
    next_lesson: 'Distributed Systems Basics',
  },
  {
    id: '4',
    title: 'GraphQL API Development',
    description: 'Build production-ready GraphQL APIs with Node.js and Apollo.',
    thumbnail_url: '/courses/graphql.jpg',
    progress: 0,
    status: 'not_started',
    instructor: { name: 'Emma Wilson', avatar_url: '/avatars/emma.jpg' },
    duration: '6 hours',
    lessons_completed: 0,
    total_lessons: 8,
  },
])

const stats = [
  { label: 'Courses in Progress', value: 3, icon: BookOpenIcon, color: 'primary', trend: '+1 this week' },
  { label: 'Completed This Month', value: 2, icon: CheckCircleIcon, color: 'success', trend: '+50% vs last month' },
  { label: 'Learning Streak', value: '12 days', icon: FlameIcon, color: 'warning', trend: 'Personal best!' },
  { label: 'Total Hours', value: '47h', icon: ClockIcon, color: 'info', trend: '3.2h this week' },
]

const upcomingDeadlines = [
  { id: '1', title: 'React Patterns - Final Project', course: 'Advanced React Patterns', due: '2024-01-20T23:59:00Z', type: 'assignment' },
  { id: '2', title: 'ML Fundamentals - Quiz 3', course: 'Machine Learning Fundamentals', due: '2024-01-18T17:00:00Z', type: 'quiz' },
  { id: '3', title: 'System Design - Peer Review', course: 'System Design Interview Prep', due: '2024-01-22T12:00:00Z', type: 'peer_review' },
]

const recentActivity = [
  { id: '1', type: 'lesson_completed', title: 'Completed: State Management with Redux', course: 'Advanced React Patterns', time: '2024-01-15T10:30:00Z', icon: CheckCircleIcon, color: 'success' },
  { id: '2', type: 'assignment_submitted', title: 'Submitted: ML Model Evaluation', course: 'Machine Learning Fundamentals', time: '2024-01-14T16:45:00Z', icon: FileTextIcon, color: 'primary' },
  { id: '3', type: 'discussion_reply', title: 'Replied to: "Best practices for caching"', course: 'Advanced React Patterns', time: '2024-01-14T14:20:00Z', icon: MessageSquareIcon, color: 'info' },
  { id: '4', type: 'badge_earned', title: 'Earned: "React Expert" badge', course: 'Advanced React Patterns', time: '2024-01-13T11:00:00Z', icon: AwardIcon, color: 'warning' },
  { id: '5', type: 'course_enrolled', title: 'Enrolled in: GraphQL API Development', course: 'GraphQL API Development', time: '2024-01-12T08:00:00Z', icon: BookOpenIcon, color: 'primary' },
]

const recommendedCourses = [
  { id: '5', title: 'TypeScript Masterclass', description: 'Advanced TypeScript patterns for large-scale applications.', thumbnail_url: '/courses/ts-masterclass.jpg', instructor: 'David Park', duration: '10 hours', rating: 4.9, students: 12500 },
  { id: '6', title: 'Kubernetes for Developers', description: 'Deploy and manage containerized applications at scale.', thumbnail_url: '/courses/k8s-devs.jpg', instructor: 'Lisa Wong', duration: '14 hours', rating: 4.8, students: 8900 },
  { id: '7', title: 'Advanced CSS Layouts', description: 'Modern CSS techniques including Grid, Flexbox, and Container Queries.', thumbnail_url: '/courses/advanced-css.jpg', instructor: 'Maria Santos', duration: '8 hours', rating: 4.7, students: 6700 },
]

const filteredCourses = computed(() => {
  let result = courses.value
  
  if (activeTab.value === 'in-progress') {
    result = result.filter(c => c.status === 'in_progress')
  } else if (activeTab.value === 'completed') {
    result = result.filter(c => c.status === 'completed')
  }
  
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(c => 
      c.title.toLowerCase().includes(query) || 
      c.description.toLowerCase().includes(query)
    )
  }
  
  return result
})

const tabs = [
  { value: 'in-progress', label: 'In Progress', count: courses.value.filter(c => c.status === 'in_progress').length },
  { value: 'completed', label: 'Completed', count: courses.value.filter(c => c.status === 'completed').length },
  { value: 'all', label: 'All Courses', count: courses.value.length },
]
</script>

<template>
  <DashboardLayout 
    title="Dashboard" 
    description="Continue your learning journey and track your progress"
  >
    <template #header>
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div class="flex items-center gap-3">
          <Button variant="outline" size="icon" class="md:hidden">
            <MenuIcon class="h-5 w-5" />
          </Button>
          <Link href="/courses" class="flex items-center gap-2 text-primary-600 hover:underline">
            <PlusIcon class="h-5 w-5" />
            <span class="hidden sm:inline">Enroll in Course</span>
          </Link>
        </div>
        <div class="flex items-center gap-2">
          <Button variant="outline" size="icon">
            <FilterIcon class="h-4 w-4" />
          </Button>
          <Button variant="outline" size="icon">
            <SearchIcon class="h-4 w-4" />
          </Button>
        </div>
      </div>
    </template>

    <!-- Stats Cards -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
      <Card v-for="stat in stats" :key="stat.label" class="relative overflow-hidden">
        <div class="absolute inset-0 bg-gradient-to-br from-{{stat.color}}-500/10 to-transparent" />
        <CardContent class="relative p-6">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">{{ stat.label }}</p>
              <p class="text-3xl font-display font-bold text-foreground mt-1">{{ stat.value }}</p>
              <p class="text-xs text-muted-foreground mt-1">{{ stat.trend }}</p>
            </div>
            <div class="p-3 rounded-xl bg-{{stat.color}}-100 dark:bg-{{stat.color}}-900/30">
              <component :is="stat.icon" class="h-6 w-6 text-{{stat.color}}-600 dark:text-{{stat.color}}-400" />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Main Content Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left Column - Courses & Activity -->
      <div class="lg:col-span-2 space-y-6">
        <!-- Continue Learning -->
        <ContinueLearning :courses="courses.value.filter(c => c.status === 'in_progress').slice(0, 3)" />
        
        <!-- My Courses -->
        <Card>
          <CardHeader>
            <div class="flex items-center justify-between">
              <div>
                <CardTitle>My Courses</CardTitle>
                <CardDescription>Track your progress and continue learning</CardDescription>
              </div>
              <Link href="/my-courses" class="text-sm text-primary-600 hover:underline">View all</Link>
            </div>
          </CardHeader>
          <CardContent>
            <Tabs default-value="in-progress" class="w-full">
              <TabsList class="grid w-full grid-cols-3">
                <TabsTrigger v-for="tab in tabs" :key="tab.value" :value="tab.value">
                  {{ tab.label }}
                  <Badge variant="secondary" class="ml-2">{{ tab.count }}</Badge>
                </TabsTrigger>
              </TabsList>
              
              <TabsContent value="in-progress" class="mt-4">
                <div v-if="filteredCourses.length === 0" class="text-center py-12">
                  <Skeleton class="h-40 w-full max-w-md mx-auto rounded-xl" />
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <CourseCard 
                    v-for="course in filteredCourses" 
                    :key="course.id" 
                    :course="course" 
                    :show-progress="true"
                    :compact="true"
                  />
                </div>
              </TabsContent>
              
              <TabsContent value="completed" class="mt-4">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <CourseCard 
                    v-for="course in filteredCourses" 
                    :key="course.id" 
                    :course="course" 
                    :show-progress="true"
                    :compact="true"
                  />
                </div>
              </TabsContent>
              
              <TabsContent value="all" class="mt-4">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <CourseCard 
                    v-for="course in filteredCourses" 
                    :key="course.id" 
                    :course="course" 
                    :show-progress="true"
                    :compact="true"
                  />
                </div>
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>

        <!-- Recent Activity -->
        <Card>
          <CardHeader>
            <CardTitle>Recent Activity</CardTitle>
            <CardDescription>Your latest learning activity</CardDescription>
          </CardHeader>
          <CardContent>
            <div class="space-y-4">
              <div v-for="activity in recentActivity" :key="activity.id" class="flex items-start gap-4 p-4 rounded-lg hover:bg-accent transition-colors">
                <div class="p-2 rounded-lg bg-{{activity.color}}-100 dark:bg-{{activity.color}}-900/30">
                  <component :is="activity.icon" class="h-5 w-5 text-{{activity.color}}-600 dark:text-{{activity.color}}-400" />
                </div>
                <div class="flex-1 min-w-0">
                  <p class="font-medium text-foreground">{{ activity.title }}</p>
                  <p class="text-sm text-muted-foreground">{{ activity.course }}</p>
                  <p class="text-xs text-muted-foreground mt-1">{{ formatRelativeTime(activity.time) }}</p>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      <!-- Right Column - Sidebar Widgets -->
      <div class="space-y-6">
        <!-- Upcoming Deadlines -->
        <UpcomingDeadlines :deadlines="upcomingDeadlines" />
        
        <!-- Learning Streak -->
        <LearningStreak :current-streak="12" :longest-streak="24" :weekly-goal="5" />
        
        <!-- Skill Progress -->
        <SkillProgress 
          :skills="[
            { name: 'React', level: 85, color: 'primary' },
            { name: 'TypeScript', level: 78, color: 'info' },
            { name: 'Node.js', level: 72, color: 'success' },
            { name: 'GraphQL', level: 65, color: 'accent' },
            { name: 'Docker', level: 55, color: 'warning' },
          ]"
        />
        
        <!-- Recommended Courses -->
        <Card>
          <CardHeader>
            <div class="flex items-center justify-between">
              <CardTitle>Recommended for You</CardTitle>
            </div>
          </CardHeader>
          <CardContent>
            <div class="space-y-4">
              <div v-for="course in recommendedCourses" :key="course.id" class="flex gap-3">
                <div class="w-20 h-12 rounded-lg bg-muted relative flex-shrink-0 overflow-hidden">
                  <img :src="course.thumbnail_url" :alt="course.title" class="w-full h-full object-cover" />
                </div>
                <div class="flex-1 min-w-0">
                  <p class="font-medium text-sm truncate">{{ course.title }}</p>
                  <p class="text-xs text-muted-foreground">{{ course.instructor }} • {{ course.duration }}</p>
                  <div class="flex items-center gap-2 mt-1">
                    <StarIcon class="h-3 w-3 text-warning-500 fill-current" />
                    <span class="text-xs text-muted-foreground">{{ course.rating }} ({{ course.students }}+)</span>
                  </div>
                </div>
                <Button variant="ghost" size="icon" class="text-muted-foreground hover:text-foreground">
                  <BookmarkIcon class="h-4 w-4" />
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </DashboardLayout>
</template>

<script>
const BookOpenIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const ClockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const TrophyIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
const AwardIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const FlameIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 11a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const TrendingUpIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 7h8m0 0v8m0-8l-8 8-4-4-6 6' })]) }})
const TargetIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('circle', { cx: '12', cy: '12', r: '10', 'stroke-width': '2' }), h('circle', { cx: '12', cy: '12', r: '6', 'stroke-width': '2' }), h('circle', { cx: '12', cy: '12', r: '2', 'stroke-width': '2' })]) }})
const PlusIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4v16m8-8H4' })]) }})
const ArrowRightIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17 8l4 4m0 0l-4 4m4-4H3' })]) }})
const PlayIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M8 5v14l11-7z' })]) }})
const BookmarkIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z' })]) }})
const MoreHorizontalIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('circle', { cx: '12', cy: '12', r: '1', 'stroke-width': '2' }), h('circle', { cx: '19', cy: '12', r: '1', 'stroke-width': '2' }), h('circle', { cx: '5', cy: '12', r: '1', 'stroke-width': '2' })]) }})
const FilterIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('polygon', { points: '22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10 12.46V19M14 12.46V19M10 12.46L14 12.46M2 3h20' })]) }})
const SearchIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('circle', { cx: '11', cy: '11', r: '8', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 21l-4.35-4.35' })]) }})
const MenuIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 6h16M4 12h16M4 18h16' })]) }})
const CheckCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const FileTextIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })]) }})
const MessageSquareIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 12h4M8 12l2-2m0 0l2 2m-2-2v8m-8 0v8m0-8h8m-8 0h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949M3 12c0 4.418 4.03 8 9 8a9.863 9.863 0 014.255-.949' })]) }})
const StarIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14l1.18-6.88L2 9.27l6.91-1.01L12 2z' })]) }})
</script>