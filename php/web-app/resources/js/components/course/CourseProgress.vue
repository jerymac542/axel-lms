<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'
import { Progress } from '@/components/ui/progress'
import { Badge } from '@/components/ui/badge'
import { 
  CheckCircleIcon, ClockIcon, BookOpenIcon, AwardIcon, 
  TrophyIcon, FlameIcon, TargetIcon, ArrowRightIcon 
} from '@/components/ui/icons'
import { formatDuration } from '@/utils/cn'

interface Props {
  course: any
  enrollment: any
  variant?: 'default' | 'compact' | 'detailed'
  showStats?: boolean
  showActions?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  showStats: true,
  showActions: true,
})

const progress = computed(() => props.enrollment?.progress || 0)
const isCompleted = computed(() => props.enrollment?.status === 'completed')
const isInProgress = computed(() => props.enrollment?.status === 'active' || props.enrollment?.status === 'in_progress')
const completedLessons = computed(() => props.enrollment?.completed_lessons || 0)
const totalLessons = computed(() => props.course.lessons_count || props.course.total_lessons || 0)
const currentStreak = computed(() => props.enrollment?.learning_streak || 0)
const estimatedTimeRemaining = computed(() => {
  if (!props.course.estimated_duration) return null
  const totalMinutes = parseDuration(props.course.estimated_duration)
  const completedMinutes = (totalMinutes * progress.value) / 100
  return totalMinutes - completedMinutes
})

const parseDuration = (duration: string) => {
  // Parse ISO 8601 duration (PT12H30M) or simple format
  const match = duration.match(/PT(?:(\d+)H)?(?:(\d+)M)?/)
  if (match) {
    const hours = parseInt(match[1] || '0')
    const minutes = parseInt(match[2] || '0')
    return hours * 60 + minutes
  }
  return 0
}

const formatTimeRemaining = (minutes: number) => {
  if (minutes <= 0) return 'Almost done!'
  const hours = Math.floor(minutes / 60)
  const mins = Math.round(minutes % 60)
  if (hours > 0) return `${hours}h ${mins}m remaining`
  return `${mins}m remaining`
}
</script>

<template>
  <div :class="cn('space-y-4', variant === 'compact' && 'p-4', variant === 'detailed' && 'p-6')">
    <!-- Main Progress Ring -->
    <div class="flex items-center justify-between">
      <div class="relative" :class="variant === 'compact' ? 'w-20 h-20' : 'w-24 h-24'">
        <svg :width="variant === 'compact' ? 80 : 96" :height="variant === 'compact' ? 80 : 96" class="transform -rotate-90">
          <circle
            class="text-secondary-200 dark:text-secondary-800"
            stroke-width="6"
            stroke="currentColor"
            fill="transparent"
            r="34"
            cx="40"
            cy="40"
          />
          <circle
            :class="cn(
              'transition-all duration-500 ease-out',
              isCompleted.value ? 'text-success-500' : 'text-primary-600'
            )"
            stroke-width="6"
            stroke-dasharray="213.6"
            :stroke-dashoffset="213.6 - (progress / 100) * 213.6"
            stroke-linecap="round"
            stroke="currentColor"
            fill="transparent"
            r="34"
            cx="40"
            cy="40"
          />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center">
          <div class="text-center">
            <span class="text-2xl font-bold text-foreground">{{ progress.toFixed(0) }}%</span>
            <div v-if="isCompleted" class="flex items-center justify-center gap-1 mt-1 text-success-600 dark:text-success-400">
              <CheckCircleIcon class="h-4 w-4" />
              <span class="text-xs font-medium">Completed</span>
            </div>
          </div>
        </div>
      </div>

      <div class="flex-1 min-w-0 ml-4">
        <div class="flex items-center gap-2 mb-2">
          <h3 class="font-semibold text-lg">{{ isCompleted ? 'Course Completed' : 'Course Progress' }}</h3>
          <Badge v-if="isCompleted" variant="success" class="gap-1">
            <CheckCircleIcon class="h-3 w-3" />
            Completed
          </Badge>
          <Badge v-else-if="isInProgress" variant="default" class="gap-1">
            <span class="animate-pulse w-1.5 h-1.5 rounded-full bg-current" />
            In Progress
          </Badge>
        </div>
        
        <Progress :value="progress" class="h-3" />
        
        <div class="flex items-center justify-between text-xs text-muted-foreground mt-2">
          <span>{{ completedLessons }} of {{ totalLessons }} lessons completed</span>
          <span v-if="estimatedTimeRemaining">{{ formatTimeRemaining(estimatedTimeRemaining) }}</span>
        </div>
      </div>
    </div>

    <!-- Linear Progress Bar (for compact) -->
    <Progress v-if="variant === 'compact'" :value="progress" class="h-2" />

    <!-- Stats Grid -->
    <div v-if="showStats" :class="cn('grid gap-4', variant === 'compact' ? 'grid-cols-2' : 'grid-cols-2 md:grid-cols-4')">
      <div class="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
        <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
          <BookOpenIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
        </div>
        <div>
          <p class="text-2xl font-bold text-foreground">{{ completedLessons }}</p>
          <p class="text-xs text-muted-foreground">Lessons Done</p>
        </div>
      </div>

      <div class="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
        <div class="p-2 rounded-lg bg-success-100 dark:bg-success-900/30">
          <CheckCircleIcon class="h-5 w-5 text-success-600 dark:text-success-400" />
        </div>
        <div>
          <p class="text-2xl font-bold text-foreground">{{ totalLessons - completedLessons }}</p>
          <p class="text-xs text-muted-foreground">Remaining</p>
        </div>
      </div>

      <div class="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
        <div class="p-2 rounded-lg bg-warning-100 dark:bg-warning-900/30">
          <FlameIcon class="h-5 w-5 text-warning-600 dark:text-warning-400" />
        </div>
        <div>
          <p class="text-2xl font-bold text-foreground">{{ currentStreak }}</p>
          <p class="text-xs text-muted-foreground">Day Streak</p>
        </div>
      </div>

      <div class="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
        <div class="p-2 rounded-lg bg-accent-100 dark:bg-accent-900/30">
          <TrophyIcon class="h-5 w-5 text-accent-600 dark:text-accent-400" />
        </div>
        <div>
          <p class="text-2xl font-bold text-foreground">{{ props.enrollment?.score || 0 }}%</p>
          <p class="text-xs text-muted-foreground">Avg Score</p>
        </div>
      </div>
    </div>

    <!-- Detailed Stats -->
    <div v-if="variant === 'detailed' && showStats" class="border-t border-border pt-4 space-y-3">
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <StatItem label="Time Spent" :value="formatDuration(props.enrollment?.total_time || 0)" icon="ClockIcon" />
        <StatItem label="Lessons Completed" :value="`${completedLessons}/${totalLessons}`" icon="CheckCircleIcon" />
        <StatItem label="Current Streak" :value="`${currentStreak} days`" icon="FlameIcon" />
        <StatItem label="Average Score" :value="`${props.enrollment?.score || 0}%`" icon="AwardIcon" />
      </div>

      <div class="space-y-2">
        <div class="flex items-center justify-between text-sm">
          <span class="text-muted-foreground">Estimated time to complete</span>
          <span class="font-medium">{{ formatTimeRemaining(estimatedTimeRemaining || 0) }}</span>
        </div>
        <Progress :value="progress" class="h-2" />
      </div>

      <!-- Milestones -->
      <div v-if="props.course.milestones?.length" class="border-t border-border pt-4 space-y-2">
        <h4 class="font-medium text-sm">Milestones</h4>
        <div v-for="milestone in props.course.milestones" :key="milestone.id" class="flex items-center gap-3 p-3 rounded-lg bg-muted/50">
          <div :class="cn('w-8 h-8 rounded-full flex items-center justify-center', milestone.completed ? 'bg-success-100 dark:bg-success-900/30 text-success-600' : 'bg-muted text-muted-foreground')">
            <CheckCircleIcon v-if="milestone.completed" class="h-4 w-4" />
            <span v-else class="text-xs font-medium">{{ milestone.order }}</span>
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium" :class="milestone.completed ? 'text-foreground' : 'text-muted-foreground'">{{ milestone.title }}</p>
            <p class="text-xs text-muted-foreground">{{ milestone.description }}</p>
          </div>
          <Badge v-if="milestone.completed" variant="success">Done</Badge>
          <Badge v-else variant="outline">Pending</Badge>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div v-if="showActions && !isCompleted" class="flex gap-2 pt-2">
      <Button class="flex-1 gap-2" size="lg">
        <PlayIcon class="h-4 w-4" />
        Continue Learning
      </Button>
      <Button variant="outline" class="px-4">
        <BookOpenIcon class="h-4 w-4" />
        <span class="hidden sm:inline">Curriculum</span>
      </Button>
    </div>

    <div v-if="isCompleted && showActions" class="flex gap-2 pt-2">
      <Button variant="success" class="flex-1 gap-2" size="lg">
        <AwardIcon class="h-4 w-4" />
        View Certificate
      </Button>
      <Button variant="outline" class="px-4">
        <TrophyIcon class="h-4 w-4" />
        <span class="hidden sm:inline">Achievements</span>
      </Button>
    </div>
  </div>
</template>

<script>
const StatItem = defineComponent({
  props: ['label', 'value', 'icon'],
  setup(props) {
    const icons = {
      ClockIcon: h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' })]),
      CheckCircleIcon: h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]),
      FlameIcon: h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 11a3 3 0 11-6 0 3 3 0 016 0z' })]),
      AwardIcon: h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]),
    }
    return () => h('div', { class: 'flex items-center gap-2 p-3 rounded-lg bg-muted/50' }, [
      h('div', { class: 'p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30' }, [
        h(icons[props.icon as keyof typeof icons] || icons.ClockIcon, { class: 'h-4 w-4 text-primary-600 dark:text-primary-400' })
      ]),
      h('div', [
        h('p', { class: 'text-sm font-bold text-foreground' }, props.value),
        h('p', { class: 'text-xs text-muted-foreground' }, props.label)
      ])
    ])
  }
})

const ClockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const CheckCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const FlameIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 11a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const AwardIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
</script>