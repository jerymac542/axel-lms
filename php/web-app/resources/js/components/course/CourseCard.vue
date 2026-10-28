<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { Progress } from '@/components/ui/progress'
import { Button } from '@/components/ui/button'
import { Card, CardHeader, CardContent, CardFooter } from '@/components/ui/card'
import { ClockIcon, UserIcon, StarIcon, BookOpenIcon, CheckCircleIcon, PlayIcon, LockIcon, FlagIcon, TrendingUpIcon } from '@/components/ui/icons'
import { formatDuration, formatRelativeTime } from '@/utils/cn'

interface Props {
  course: any
  showProgress?: boolean
  compact?: boolean
  variant?: 'default' | 'featured' | 'compact' | 'detailed'
  enrollAction?: boolean
  showInstructor?: boolean
  showStats?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showProgress: true,
  compact: false,
  variant: 'default',
  enrollAction: false,
  showInstructor: true,
  showStats: true,
})

const enrollmentProgress = computed(() => {
  if (!props.course.enrollment) return 0
  return props.course.enrollment.progress || 0
})

const isEnrolled = computed(() => !!props.course.enrollment)
const isCompleted = computed(() => props.course.enrollment?.status === 'completed')
const isInProgress = computed(() => props.course.enrollment?.status === 'active' || props.course.enrollment?.status === 'in_progress')
const isLocked = computed(() => props.course.enrollment?.status === 'pending' || props.course.enrollment?.status === 'waitlisted')

const stats = computed(() => ({
  lessons: props.course.lessons_count || props.course.modules?.reduce((sum: number, m: any) => sum + (m.lessons?.length || 0), 0) || 0,
  duration: props.course.duration || props.course.estimated_duration,
  students: props.course.stats?.total_enrollments || 0,
  rating: props.course.rating || 4.8,
}))

const difficultyColor = computed(() => {
  switch (props.course.level?.toLowerCase()) {
    case 'beginner': return 'success'
    case 'intermediate': return 'warning'
    case 'advanced': return 'destructive'
    default: return 'secondary'
  }
})
</script>

<template>
  <Card 
    :class="cn(
      'group relative overflow-hidden transition-all duration-300 hover:shadow-xl',
      variant === 'featured' && 'h-full',
      variant === 'detailed' && 'grid grid-cols-1 md:grid-cols-3 gap-0',
      compact && 'max-w-xs',
    )"
  >
    <!-- Thumbnail -->
    <div 
      :class="cn(
        'relative overflow-hidden bg-muted',
        variant === 'detailed' && 'md:col-span-1 md:h-full',
        'aspect-video'
      )"
    >
      <img 
        v-if="course.thumbnail_url" 
        :src="course.thumbnail_url" 
        :alt="course.title"
        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
        loading="lazy"
      />
      <div v-else class="w-full h-full flex items-center justify-center bg-gradient-to-br from-primary-100 to-accent-100 dark:from-primary-900/30 dark:to-accent-900/30">
        <BookOpenIcon class="h-12 w-12 text-muted-foreground/50" />
      </div>

      <!-- Status Badges -->
      <div class="absolute top-3 left-3 flex flex-col gap-1.5">
        <Badge v-if="course.status === 'published'" variant="success" class="gap-1">
          <CheckCircleIcon class="h-3 w-3" />
          Published
        </Badge>
        <Badge v-else-if="course.status === 'draft'" variant="outline">
          Draft
        </Badge>
        <Badge v-if="course.visibility === 'free'" variant="secondary" class="gap-1">
          Free
        </Badge>
        <Badge v-else-if="course.visibility === 'premium'" variant="default" class="gap-1">
          Premium
        </Badge>
        <Badge v-if="isLocked" variant="outline" class="gap-1">
          <LockIcon class="h-3 w-3" />
          {{ course.enrollment?.status === 'waitlisted' ? 'Waitlisted' : 'Pending' }}
        </Badge>
      </div>

      <!-- Progress Overlay -->
      <div v-if="showProgress && enrollmentProgress > 0" class="absolute bottom-0 left-0 right-0 h-1.5 bg-black/50">
        <div 
          class="h-full bg-primary-600 transition-all duration-500" 
          :style="{ width: enrollmentProgress + '%' }"
        />
      </div>

      <!-- Play Overlay -->
      <div v-if="!compact && variant !== 'detailed' && isInProgress" class="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-300">
        <Button size="xl" variant="default" class="gap-2 shadow-xl">
          <PlayIcon class="h-5 w-5" />
          <span>Continue Learning</span>
        </Button>
      </div>

      <!-- Featured Badge -->
      <div v-if="course.featured" class="absolute top-3 right-3">
        <Badge variant="warning" class="gap-1">
          <FlagIcon class="h-3 w-3" />
          Featured
        </Badge>
      </div>

      <!-- Trending Badge -->
      <div v-if="course.trending" class="absolute bottom-3 left-3">
        <Badge variant="secondary" class="gap-1">
          <TrendingUpIcon class="h-3 w-3" />
          Trending
        </Badge>
      </div>
    </div>

    <!-- Detailed Variant - Side Content -->
    <div v-if="variant === 'detailed'" class="md:col-span-2 p-6 flex flex-col">
      <div class="flex-1">
        <!-- Header Meta -->
        <div class="flex items-start justify-between gap-2 mb-3">
          <div class="flex flex-wrap items-center gap-2">
            <Badge v-if="course.level" :variant="difficultyColor" class="text-xs">
              {{ course.level }}
            </Badge>
            <Badge v-if="course.language" variant="outline" class="text-xs">
              {{ course.language.toUpperCase() }}
            </Badge>
            <Badge v-if="course.category" variant="secondary" class="text-xs">
              {{ course.category }}
            </Badge>
          </div>
        </div>
        
        <h3 class="font-semibold text-xl leading-snug line-clamp-2 mb-2 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors">
          {{ course.title }}
        </h3>
        
        <p class="text-muted-foreground line-clamp-3 mb-4">
          {{ course.short_description || course.description }}
        </p>

        <!-- Instructor -->
        <div v-if="showInstructor && course.instructor" class="flex items-center gap-3 mb-4 p-3 rounded-lg bg-muted/50">
          <Avatar :size="md" :src="course.instructor.avatar_url" :fallback="course.instructor.name">
            <template #icon>
              <UserIcon class="h-4 w-4" />
            </template>
          </Avatar>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium truncate">{{ course.instructor.name }}</p>
            <p class="text-xs text-muted-foreground">{{ course.instructor.title || 'Instructor' }} • {{ formatRelativeTime(course.instructor.joined_at || course.created_at) }}</p>
          </div>
          <Badge v-if="course.instructor.verified" variant="success" class="gap-1">
            <CheckCircleIcon class="h-3 w-3" />
            Verified
          </Badge>
        </div>

        <!-- Stats -->
        <div v-if="showStats" class="flex flex-wrap items-center gap-4 mb-4">
          <div class="flex items-center gap-1.5 text-sm text-muted-foreground">
            <ClockIcon class="h-4 w-4" />
            <span>{{ stats.duration }}</span>
          </div>
          <div class="flex items-center gap-1.5 text-sm text-muted-foreground">
            <BookOpenIcon class="h-4 w-4" />
            <span>{{ stats.lessons }} lessons</span>
          </div>
          <div class="flex items-center gap-1.5 text-sm text-muted-foreground">
            <UserIcon class="h-4 w-4" />
            <span>{{ stats.students.toLocaleString() }} students</span>
          </div>
          <div class="flex items-center gap-1.5 text-sm">
            <StarIcon class="h-4 w-4 fill-current text-warning-500" />
            <span class="font-medium">{{ stats.rating }}</span>
          </div>
        </div>

        <!-- Learning Objectives -->
        <div v-if="course.learning_objectives?.length" class="mb-4">
          <h4 class="text-sm font-medium text-foreground mb-2">What you'll learn</h4>
          <ul class="space-y-1 text-sm text-muted-foreground">
            <li v-for="(objective, index) in course.learning_objectives.slice(0, 4)" :key="index" class="flex items-center gap-2">
              <CheckCircleIcon class="h-4 w-4 text-success-500 flex-shrink-0" />
              <span class="line-clamp-1">{{ objective }}</span>
            </li>
            <li v-if="course.learning_objectives.length > 4" class="text-primary-600 dark:text-primary-400">
              +{{ course.learning_objectives.length - 4 }} more...
            </li>
          </ul>
        </div>
      </div>

      <!-- Footer Actions -->
      <div class="border-t border-border pt-4 flex flex-col sm:flex-row gap-3">
        <Button 
          v-if="enrollAction && !isEnrolled" 
          class="flex-1 gap-2"
          size="lg"
        >
          Enroll Now
          <ArrowRightIcon class="h-4 w-4" />
        </Button>
        <Button 
          v-else-if="isCompleted" 
          variant="success" 
          class="flex-1 gap-2"
          size="lg"
        >
          <CheckCircleIcon class="h-4 w-4" />
          View Certificate
        </Button>
        <Button 
          v-else-if="isInProgress" 
          class="flex-1 gap-2"
          size="lg"
        >
          <PlayIcon class="h-4 w-4" />
          Continue Learning
        </Button>
        <Button 
          v-else 
          variant="outline" 
          class="flex-1"
          size="lg"
        >
          View Details
        </Button>
      </div>
    </div>

    <!-- Default/Compact Variant Content -->
    <div v-if="variant !== 'detailed'" :class="cn(variant === 'featured' ? 'p-5' : compact ? 'p-4' : 'p-5')">
      <!-- Header Meta -->
      <div class="flex items-start justify-between gap-2 mb-2">
        <div class="flex flex-wrap items-center gap-1.5">
          <Badge v-if="course.level" :variant="difficultyColor" class="text-xs">
            {{ course.level }}
          </Badge>
          <Badge v-if="course.language" variant="outline" class="text-xs">
            {{ course.language.toUpperCase() }}
          </Badge>
        </div>
      </div>
      
      <h3 class="font-semibold text-lg leading-snug line-clamp-2 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors mb-2">
        {{ course.title }}
      </h3>
      
      <p v-if="course.short_description" class="text-sm text-muted-foreground line-clamp-2 mb-4">
        {{ course.short_description }}
      </p>

      <!-- Instructor & Meta -->
      <div v-if="showInstructor && course.instructor" class="flex items-center gap-2 mb-4">
        <Avatar :size="sm" :src="course.instructor.avatar_url" :fallback="course.instructor.name" class="mr-2">
          <template #icon>
            <UserIcon class="h-3 w-3" />
          </template>
        </Avatar>
        <div class="flex-1 min-w-0">
          <p class="text-xs font-medium truncate">{{ course.instructor.name }}</p>
        </div>
      </div>

      <!-- Stats -->
      <div v-if="showStats" class="flex flex-wrap items-center gap-3 mb-4">
        <span class="flex items-center gap-1 text-xs text-muted-foreground">
          <ClockIcon class="h-3 w-3" />
          <span>{{ stats.duration }}</span>
        </span>
        <span class="flex items-center gap-1 text-xs text-muted-foreground">
          <BookOpenIcon class="h-3 w-3" />
          <span>{{ stats.lessons }} lessons</span>
        </span>
        <span class="flex items-center gap-1 text-xs">
          <StarIcon class="h-3 w-3 fill-current text-warning-500" />
          <span class="font-medium">{{ stats.rating }}</span>
        </span>
        <span class="flex items-center gap-1 text-xs text-muted-foreground">
          <UserIcon class="h-3 w-3" />
          <span>{{ stats.students.toLocaleString() }}</span>
        </span>
      </div>

      <!-- Progress Bar -->
      <div v-if="showProgress && enrollmentProgress > 0" class="mb-4">
        <div class="flex items-center justify-between text-xs mb-1">
          <span class="font-medium">Your Progress</span>
          <span class="text-muted-foreground">{{ enrollmentProgress }}%</span>
        </div>
        <Progress :value="enrollmentProgress" class="h-2" />
      </div>

      <!-- Learning Objectives Preview -->
      <div v-if="course.learning_objectives?.length" class="mb-4">
        <h4 class="text-xs font-medium text-foreground mb-1">What you'll learn</h4>
        <ul class="space-y-0.5 text-xs text-muted-foreground">
          <li v-for="(objective, index) in course.learning_objectives.slice(0, 3)" :key="index" class="flex items-center gap-1.5">
            <CheckCircleIcon class="h-3 w-3 text-success-500 flex-shrink-0" />
            <span class="line-clamp-1">{{ objective }}</span>
          </li>
          <li v-if="course.learning_objectives.length > 3" class="text-primary-600 dark:text-primary-400">
            +{{ course.learning_objectives.length - 3 }} more...
          </li>
        </ul>
      </div>
    </div>

    <!-- Footer Actions -->
    <CardFooter :class="cn(variant === 'featured' ? 'p-5' : compact ? 'p-4 pt-0' : 'p-5 pt-0')" class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Button 
          v-if="enrollAction && !isEnrolled" 
          :size="compact ? 'sm' : 'md'"
          class="flex-1 sm:flex-none gap-2"
        >
          Enroll Now
          <ArrowRightIcon class="h-4 w-4" />
        </Button>
        <Button 
          v-else-if="isCompleted" 
          variant="success" 
          :size="compact ? 'sm' : 'md'"
          class="flex-1 sm:flex-none gap-2"
        >
          <CheckCircleIcon class="h-4 w-4" />
          View Certificate
        </Button>
        <Button 
          v-else-if="isInProgress" 
          :size="compact ? 'sm' : 'md'"
          class="flex-1 sm:flex-none gap-2"
        >
          <PlayIcon class="h-4 w-4" />
          Continue
        </Button>
        <Button 
          v-else 
          variant="outline" 
          :size="compact ? 'sm' : 'md'"
          class="flex-1 sm:flex-none"
        >
          View Details
        </Button>
      </div>
      
      <div v-if="!compact" class="flex items-center gap-1">
        <BookmarkIcon class="h-4 w-4 text-muted-foreground hover:text-primary-600 cursor-pointer transition-colors" />
        <span class="text-xs text-muted-foreground">{{ course.bookmarks_count || 0 }}</span>
      </div>
    </CardFooter>
  </Card>
</template>

<script>
const ArrowRightIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17 8l4 4m0 0l-4 4m4-4H3' })]) }})
const ClockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const UserIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z' })]) }})
const StarIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14l1.18-6.88L2 9.27l6.91-1.01L12 2z' })]) }})
const BookOpenIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const CheckCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const PlayIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M8 5v14l11-7z' })]) }})
const LockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z' })]) }})
const FlagIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1v12z' })]) }})
const TrendingUpIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 7h8m0 0v8m0-8l-8 8-4-4-6 6' })]) }})
</script>