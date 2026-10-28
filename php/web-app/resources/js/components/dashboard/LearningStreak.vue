<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import { Badge } from '@/components/ui/badge'
import { 
  FlameIcon, CalendarIcon, CheckCircleIcon, 
  TrophyIcon, TargetIcon, ArrowRightIcon,
  ChevronLeftIcon, ChevronRightIcon
} from '@/components/ui/icons'
import { formatRelativeTime } from '@/utils/cn'

interface Props {
  currentStreak: number
  longestStreak: number
  weeklyGoal: number
  weeklyProgress: number
  streakHistory: Array<{ date: string, completed: boolean, minutes: number }>
  showCalendar?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showCalendar: true,
})

const progressPercent = computed(() => Math.min(100, (props.weeklyProgress / props.weeklyGoal) * 100))
const isOnTrack = computed(() => props.weeklyProgress >= (props.weeklyGoal / 7) * new Date().getDay())

// Generate calendar grid for current month
const currentMonth = ref(new Date())
const calendarDays = computed(() => {
  const year = currentMonth.value.getFullYear()
  const month = currentMonth.value.getMonth()
  const firstDay = new Date(year, month, 1).getDay()
  const daysInMonth = new Date(year, month + 1, 0).getDate()
  const prevMonthDays = new Date(year, month, 0).getDate()
  
  const days: Array<{ date: Date, isCurrentMonth: boolean, isToday: boolean, completed: boolean, minutes: number }> = []
  
  // Previous month padding
  for (let i = firstDay - 1; i >= 0; i--) {
    const date = new Date(currentMonth.value.getFullYear(), currentMonth.value.getMonth(), -i)
    days.push({ date, isCurrentMonth: false, isToday: false, completed: false, minutes: 0 })
  }
  
  // Current month days
  for (let day = 1; day <= daysInMonth; day++) {
    const date = new Date(year, month, day)
    const dayStr = date.toISOString().split('T')[0]
    const history = props.streakHistory.find(h => h.date === dayStr)
    days.push({
      date,
      isCurrentMonth: true,
      isToday: date.toDateString() === new Date().toDateString(),
      completed: history?.completed || false,
      minutes: history?.minutes || 0,
    })
  }
  
  // Next month padding
  const remaining = 42 - days.length
  for (let day = 1; day <= remaining; day++) {
    const date = new Date(year, month + 1, day)
    days.push({ date, isCurrentMonth: false, isToday: false, completed: false, minutes: 0 })
  }
  
  return days
})

const previousMonth = () => {
  currentMonth.value = new Date(currentMonth.value.getFullYear(), currentMonth.value.getMonth() - 1, 1)
}

const nextMonth = () => {
  currentMonth.value = new Date(currentMonth.value.getFullYear(), currentMonth.value.getMonth() + 1, 1)
}
</script>

<template>
  <Card>
    <CardHeader class="pb-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div class="p-2 rounded-lg bg-warning-100 dark:bg-warning-900/30">
            <FlameIcon class="h-5 w-5 text-warning-600 dark:text-warning-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Learning Streak</CardTitle>
            <CardDescription>{{ currentStreak }} day streak • {{ longestStreak }} day best</CardDescription>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <Badge :variant="currentStreak > 0 ? 'success' : 'default'" class="gap-1">
            <FlameIcon class="h-3 w-3" />
            {{ currentStreak }} days
          </Badge>
          <Badge variant="outline">
            <TrophyIcon class="h-3 w-3" />
            Best: {{ longestStreak }}
          </Badge>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0 space-y-6">
        <!-- Weekly Goal Progress -->
        <div>
          <div class="flex items-center justify-between mb-3">
            <div>
              <h4 class="font-medium">Weekly Goal</h4>
              <p class="text-sm text-muted-foreground">{{ weeklyProgress }} / {{ weeklyGoal }} hours</p>
            </div>
            <Badge :variant="weeklyProgress >= weeklyGoal ? 'success' : 'default'" class="gap-1">
              <TargetIcon class="h-3 w-3" />
              {{ Math.min(100, Math.round((weeklyProgress / weeklyGoal) * 100)) }}%
            </Badge>
          </div>
          <Progress :value="Math.min(100, (weeklyProgress / weeklyGoal) * 100)" class="h-2" />
          <p class="text-xs text-muted-foreground mt-1">
            {{ weeklyProgress }}h of {{ weeklyGoal }}h goal • 
            {{ isOnTrack ? 'On track! 🎯' : 'Need ' + ((weeklyGoal / 7) * new Date().getDay() - weeklyProgress).toFixed(1) + 'h more today' }}
          </p>
        </div>

        <!-- Current Streak Visualization -->
        <div>
          <h4 class="font-medium mb-3">Current Streak</h4>
          <div class="flex items-center justify-center gap-1 flex-wrap">
            <div v-for="i in Math.min(currentStreak, 30)" :key="i" 
              class="w-8 h-8 rounded-lg flex items-center justify-center bg-gradient-to-br from-warning-400 to-warning-600 text-white font-bold text-sm shadow-sm"
            >
              {{ i }}
            </div>
            <div v-if="currentStreak > 30" class="flex items-center justify-center w-8 h-8 rounded-lg bg-muted text-muted-foreground text-sm">
              +{{ currentStreak - 30 }}
            </div>
          </div>
          <p class="text-sm text-muted-foreground text-center mt-2">
            {{ currentStreak }} days in a row! 
            <span v-if="currentStreak > 0" class="text-warning-600 dark:text-warning-400 font-medium"> 🔥</span>
          </p>
        </div>

        <!-- Calendar Heatmap -->
        <div v-if="showCalendar" class="border-t border-border pt-6">
          <div class="flex items-center justify-between mb-4">
            <h4 class="font-medium">Activity Calendar</h4>
            <div class="flex items-center gap-2">
              <Button variant="ghost" size="icon" @click="previousMonth" class="h-8 w-8">
                <ChevronLeftIcon class="h-4 w-4" />
              </Button>
              <span class="font-medium min-w-[150px] text-center">
                {{ currentMonth.toLocaleDateString('en-US', { month: 'long', year: 'numeric' }) }}
              </span>
              <Button variant="ghost" size="icon" @click="nextMonth" class="h-8 w-8">
                <ChevronRightIcon class="h-4 w-4" />
              </Button>
            </div>
          </div>
          
          <!-- Day Labels -->
          <div class="grid grid-cols-7 gap-1 mb-2 text-center text-xs text-muted-foreground font-medium">
            <span v-for="day in ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']" :key="day">{{ day }}</span>
          </div>
          
          <!-- Calendar Grid -->
          <div class="grid grid-cols-7 gap-1">
            <div v-for="(day, index) in calendarDays" :key="index" 
              :class="cn(
                'aspect-square rounded flex items-center justify-center text-xs font-medium transition-colors',
                !day.isCurrentMonth && 'text-muted-foreground/30',
                day.isToday && 'ring-2 ring-primary-500',
                day.completed && 'bg-warning-500 text-white font-bold',
                !day.completed && day.isCurrentMonth && 'bg-muted text-muted-foreground hover:bg-accent',
                !day.isCurrentMonth && 'cursor-default'
              )"
              :title="day.completed ? `${day.minutes} min` : 'No activity'"
            >
              {{ day.date.getDate() }}
            </div>
          </div>
          
          <!-- Legend -->
          <div class="flex items-center justify-center gap-4 mt-4 text-xs text-muted-foreground">
            <div class="flex items-center gap-1">
              <div class="w-4 h-4 rounded bg-warning-500" />
              <span>Completed</span>
            </div>
            <div class="flex items-center gap-1">
              <div class="w-4 h-4 rounded bg-muted" />
              <span>No activity</span>
            </div>
            <div class="flex items-center gap-1">
              <div class="w-4 h-4 rounded border border-border" />
              <span>Future</span>
            </div>
          </div>
        </div>

        <!-- Streak Stats -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 border-t border-border pt-6">
          <div class="text-center p-3 rounded-lg bg-muted/50">
            <p class="text-2xl font-display font-bold text-warning-600">{{ currentStreak }}</p>
            <p class="text-xs text-muted-foreground">Current Streak</p>
          </div>
          <div class="text-center p-3 rounded-lg bg-muted/50">
            <p class="text-2xl font-display font-bold text-muted-foreground">{{ longestStreak }}</p>
            <p class="text-xs text-muted-foreground">Longest Streak</p>
          </div>
          <div class="text-center p-3 rounded-lg bg-muted/50">
            <p class="text-2xl font-display font-bold text-primary-600">{{ streakHistory.filter(d => d.completed).length }}</p>
            <p class="text-xs text-muted-foreground">Total Active Days</p>
          </div>
          <div class="text-center p-3 rounded-lg bg-muted/50">
            <p class="text-2xl font-display font-bold text-success-600">{{ streakHistory.reduce((sum, d) => sum + d.minutes, 0) }}m</p>
            <p class="text-xs text-muted-foreground">Total Minutes</p>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script>
import { ref } from 'vue'
const ChevronLeftIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 19l-7-7 7-7' })]) }})
const ChevronRightIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 5l7 7-7 7' })]) }})
</script>