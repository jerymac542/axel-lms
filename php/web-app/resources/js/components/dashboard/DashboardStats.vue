<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardContent } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import { 
  BookOpenIcon, CheckCircleIcon, ClockIcon, TrophyIcon, 
  FlameIcon, TrendingUpIcon, TargetIcon, AwardIcon,
  UsersIcon, BookIcon, CalendarIcon, StarIcon
} from '@/components/ui/icons'
import { formatDuration, formatRelativeTime } from '@/utils/cn'

interface Props {
  stats: {
    coursesInProgress: number
    completedCourses: number
    learningStreak: number
    totalHours: number
    certificatesEarned: number
    badgesEarned: number
    weeklyGoal?: number
    weeklyProgress?: number
    upcomingDeadlines?: number
    avgScore?: number
  }
  variant?: 'default' | 'compact' | 'detailed'
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
})

const statsCards = computed(() => [
  {
    key: 'inProgress',
    label: 'In Progress',
    value: props.stats.coursesInProgress,
    icon: BookOpenIcon,
    color: 'primary',
    trend: '+1 this week',
    trendColor: 'success',
  },
  {
    key: 'completed',
    label: 'Completed',
    value: props.stats.completedCourses,
    icon: CheckCircleIcon,
    color: 'success',
    trend: '+2 this month',
    trendColor: 'success',
  },
  {
    key: 'streak',
    label: 'Learning Streak',
    value: `${props.stats.learningStreak} days`,
    icon: FlameIcon,
    color: 'warning',
    trend: props.stats.learningStreak > 7 ? 'Personal best!' : 'Keep it up!',
    trendColor: 'warning',
  },
  {
    key: 'hours',
    label: 'Total Hours',
    value: `${props.stats.totalHours}h`,
    icon: ClockIcon,
    color: 'info',
    trend: '+3.2h this week',
    trendColor: 'success',
  },
])

const extraStats = computed(() => [
  {
    key: 'certificates',
    label: 'Certificates',
    value: props.stats.certificatesEarned,
    icon: AwardIcon,
    color: 'warning',
  },
  {
    key: 'badges',
    label: 'Badges',
    value: props.stats.badgesEarned,
    icon: TrophyIcon,
    color: 'accent',
  },
  {
    key: 'score',
    label: 'Avg Score',
    value: `${props.stats.avgScore || 0}%`,
    icon: TargetIcon,
    color: 'primary',
  },
  {
    key: 'deadlines',
    label: 'Upcoming',
    value: props.stats.upcomingDeadlines || 0,
    icon: CalendarIcon,
    color: props.stats.upcomingDeadlines && props.stats.upcomingDeadlines > 3 ? 'destructive' : 'success',
  },
]
</script>

<template>
  <div :class="cn('space-y-6', variant === 'compact' && 'grid grid-cols-2 lg:grid-cols-4 gap-4')">
    <!-- Main Stats -->
    <div :class="cn('grid gap-4', variant === 'compact' ? 'grid-cols-2' : 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4')">
      <Card v-for="stat in statsCards" :key="stat.key" class="relative overflow-hidden transition-all hover:shadow-lg">
        <div class="absolute inset-0 bg-gradient-to-br from-[{{stat.color}}]-500/10 to-transparent" />
        <CardContent class="relative p-6">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">{{ stat.label }}</p>
              <p class="text-3xl font-display font-bold text-foreground mt-1">{{ stat.value }}</p>
              <p class="text-xs text-muted-foreground mt-1">{{ stat.trend }}</p>
            </div>
            <div :class="cn('p-3 rounded-xl', `bg-${stat.color}-100 dark:bg-${stat.color}-900/30`)" >
              <component :is="stat.icon" :class="cn('h-6 w-6', `text-${stat.color}-600 dark:text-${stat.color}-400`)" />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Extra Stats -->
    <div v-if="variant === 'detailed'" class="grid gap-4 grid-cols-2 md:grid-cols-4">
      <Card v-for="stat in extraStats" :key="stat.key" class="relative overflow-hidden">
        <div class="absolute inset-0 bg-gradient-to-br from-[{{stat.color}}]-500/10 to-transparent" />
        <CardContent class="relative p-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-xs font-medium text-muted-foreground">{{ stat.label }}</p>
              <p class="text-2xl font-display font-bold text-foreground mt-1">{{ stat.value }}</p>
            </div>
            <div :class="cn('p-2 rounded-lg', `bg-${stat.color}-100 dark:bg-${stat.color}-900/30`)" >
              <component :is="stat.icon" :class="cn('h-5 w-5', `text-${stat.color}-600 dark:text-${stat.color}-400`)" />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Weekly Goal Progress -->
    <div v-if="props.stats.weeklyGoal && props.stats.weeklyProgress !== undefined" class="border-t border-border pt-6">
      <div class="flex items-center justify-between mb-4">
        <div>
          <h3 class="font-semibold">Weekly Learning Goal</h3>
          <p class="text-sm text-muted-foreground">{{ props.stats.weeklyProgress }} / {{ props.stats.weeklyGoal }} hours</p>
        </div>
        <Badge :variant="props.stats.weeklyProgress >= props.stats.weeklyGoal ? 'success' : 'default'" class="gap-1">
          <TargetIcon class="h-3 w-3" />
          {{ Math.round((props.stats.weeklyProgress / props.stats.weeklyGoal) * 100) }}%
        </Badge>
      </div>
      <Progress :value="(props.stats.weeklyProgress / props.stats.weeklyGoal) * 100" class="h-3" />
    </div>
  </div>
</template>