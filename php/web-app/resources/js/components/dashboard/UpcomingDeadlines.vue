<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { 
  CalendarIcon, ClockIcon, AlertCircleIcon, CheckCircleIcon, 
  BookOpenIcon, FileTextIcon, HelpCircleIcon, FlagIcon,
  ArrowRightIcon, AlertTriangleIcon
} from '@/components/ui/icons'
import { formatRelativeTime, formatDate } from '@/utils/cn'

interface Props {
  deadlines: Array<{
    id: string
    title: string
    course: string
    due: string
    type: 'assignment' | 'quiz' | 'exam' | 'peer_review' | 'project'
    status: 'pending' | 'submitted' | 'overdue' | 'graded'
    courseId: string
    score?: number
  }>
  maxItems?: number
  showAllLink?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  maxItems: 5,
  showAllLink: true,
})

const now = new Date()

const sortedDeadlines = computed(() => {
  return [...props.deadlines].sort((a, b) => new Date(a.due).getTime() - new Date(b.due).getTime())
})

const overdue = computed(() => 
  sortedDeadlines.value.filter(d => d.status === 'overdue' || (new Date(d.due) < now && d.status !== 'submitted' && d.status !== 'graded'))
)

const upcoming = computed(() => 
  sortedDeadlines.value.filter(d => d.status !== 'overdue' && new Date(d.due) >= now && d.status !== 'submitted' && d.status !== 'graded')
)

const completed = computed(() => 
  sortedDeadlines.value.filter(d => d.status === 'submitted' || d.status === 'graded')
)

const getTypeIcon = (type: string) => {
  switch (type) {
    case 'assignment': return FileTextIcon
    case 'quiz': return HelpCircleIcon
    case 'exam': return AlertTriangleIcon
    case 'peer_review': return 'UsersIcon'
    case 'project': return FlagIcon
    default: return CalendarIcon
  }
}

const getStatusColor = (deadline: any) => {
  if (deadline.status === 'overdue' || (new Date(deadline.due) < now && deadline.status !== 'submitted' && deadline.status !== 'graded')) {
    return 'destructive'
  }
  if (deadline.status === 'graded') return 'success'
  if (deadline.status === 'submitted') return 'info'
  const daysUntil = Math.ceil((new Date(deadline.due).getTime() - now.getTime()) / (1000 * 60 * 60 * 24))
  if (daysUntil <= 1) return 'destructive'
  if (daysUntil <= 3) return 'warning'
  return 'default'
}

const getStatusLabel = (deadline: any) => {
  if (deadline.status === 'graded') return 'Graded'
  if (deadline.status === 'submitted') return 'Submitted'
  if (deadline.status === 'overdue') return 'Overdue'
  const daysUntil = Math.ceil((new Date(deadline.due).getTime() - now.getTime()) / (1000 * 60 * 60 * 24))
  if (daysUntil < 0) return 'Overdue'
  if (daysUntil === 0) return 'Due today'
  if (daysUntil === 1) return 'Due tomorrow'
  return `Due in ${daysUntil} days`
}
</script>

<template>
  <Card>
    <CardHeader class="pb-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div class="p-2 rounded-lg bg-destructive-100 dark:bg-destructive-900/30">
            <CalendarIcon class="h-5 w-5 text-destructive-600 dark:text-destructive-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Upcoming Deadlines</CardTitle>
            <CardDescription>Stay on top of your assignments</CardDescription>
          </div>
        </div>
        <div v-if="props.showAllLink" class="flex items-center gap-2">
          <Badge variant="destructive">{{ overdue.length }}</Badge>
          <Badge variant="warning">{{ upcoming.filter(d => {
            const days = Math.ceil((new Date(d.due).getTime() - Date.now()) / (1000 * 60 * 60 * 24))
            return days <= 3
          }).length }}</Badge>
          <Badge variant="default">{{ upcoming.filter(d => {
            const days = Math.ceil((new Date(d.due).getTime() - Date.now()) / (1000 * 60 * 60 * 24))
            return days > 3
          }).length }}</Badge>
          <Button variant="ghost" size="sm" class="gap-1 ml-auto">
            View All
            <ArrowRightIcon class="h-3 w-3" />
          </Button>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <!-- Overdue Section -->
        <div v-if="overdue.length > 0" class="mb-6">
          <div class="flex items-center gap-2 mb-3">
            <div class="p-1.5 rounded bg-destructive-100 dark:bg-destructive-900/30">
              <AlertCircleIcon class="h-4 w-4 text-destructive-600 dark:text-destructive-400" />
            </div>
            <h4 class="font-medium text-destructive-600 dark:text-destructive-400">Overdue ({{ overdue.length }})</h4>
          </div>
          <div class="space-y-2">
            <DeadlineItem v-for="deadline in overdue.slice(0, 3)" :key="deadline.id" :deadline="deadline" :showCourse="true" />
          </div>
          <Button v-if="overdue.length > 3" variant="ghost" size="sm" class="w-full mt-2 gap-1">
            View all {{ overdue.length }} overdue items
            <ArrowRightIcon class="h-3 w-3" />
          </Button>
        </div>

        <!-- Due Soon Section -->
        <div v-if="upcoming.length > 0" class="mb-6">
          <div class="flex items-center gap-2 mb-3">
            <div class="p-1.5 rounded bg-warning-100 dark:bg-warning-900/30">
              <ClockIcon class="h-4 w-4 text-warning-600 dark:text-warning-400" />
            </div>
            <h4 class="font-medium">Due Soon ({{ upcoming.length }})</h4>
          </div>
          <div class="space-y-2">
            <DeadlineItem v-for="deadline in upcoming.slice(0, 5)" :key="deadline.id" :deadline="deadline" :showCourse="true" />
          </div>
        </div>

        <!-- Completed Section -->
        <div v-if="completed.length > 0" class="border-t border-border pt-4">
          <h4 class="font-medium mb-3">Recently Completed ({{ completed.length }})</h4>
          <div class="space-y-2 max-h-60 overflow-y-auto">
            <DeadlineItem v-for="deadline in completed.slice(0, 3)" :key="deadline.id" :deadline="deadline" :showCourse="true" :compact="true" />
          </div>
        </div>

        <!-- Empty State -->
        <div v-if="overdue.length === 0 && upcoming.length === 0 && completed.length === 0" class="text-center py-8">
          <div class="mx-auto mb-3 p-3 rounded-full bg-muted">
            <CalendarIcon class="h-8 w-8 text-muted-foreground" />
          </div>
          <h4 class="font-medium mb-1">All caught up!</h4>
          <p class="text-sm text-muted-foreground">No upcoming deadlines. Enjoy your free time!</p>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script>
const DeadlineItem = defineComponent({
  props: {
    deadline: Object,
    showCourse: { type: Boolean, default: true },
    compact: { type: Boolean, default: false },
  },
  setup(props) {
    const now = new Date()
    const dueDate = new Date(props.deadline.due)
    const daysUntil = Math.ceil((dueDate.getTime() - Date.now()) / (1000 * 60 * 60 * 24))
    
    const isOverdue = props.deadline.status === 'overdue' || (dueDate < now && props.deadline.status !== 'submitted' && props.deadline.status !== 'graded')
    const statusColor = isOverdue ? 'destructive' : 
      props.deadline.status === 'graded' ? 'success' :
      props.deadline.status === 'submitted' ? 'info' :
      daysUntil <= 1 ? 'destructive' :
      daysUntil <= 3 ? 'warning' : 'default'
    
    const statusLabel = props.deadline.status === 'graded' ? 'Graded' :
      props.deadline.status === 'submitted' ? 'Submitted' :
      props.deadline.status === 'overdue' ? 'Overdue' :
      daysUntil < 0 ? 'Overdue' :
      daysUntil === 0 ? 'Due today' :
      daysUntil === 1 ? 'Due tomorrow' : `Due in ${daysUntil} days`

    const typeIcons = {
      assignment: 'FileTextIcon',
      quiz: 'HelpCircleIcon',
      exam: 'AlertTriangleIcon',
      peer_review: 'UsersIcon',
      project: 'FlagIcon',
    }

    return () => h('div', { 
      class: `flex items-start gap-3 p-3 rounded-lg border transition-colors hover:bg-accent/50 ${props.compact ? 'p-2' : ''}` 
    }, [
      // Type Icon
      h('div', { class: 'flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center bg-muted' }, [
        h('component', { is: typeIcons[props.deadline.type as keyof typeof typeIcons] || 'CalendarIcon', class: 'h-4 w-4 text-muted-foreground' })
      ]),
      
      // Content
      h('div', { class: 'flex-1 min-w-0' }, [
        h('div', { class: 'flex items-center gap-2' }, [
          h('h5', { class: 'font-medium truncate' }, props.deadline.title),
          h('Badge', { variant: statusColor, class: 'text-xs gap-1' }, [
            h('component', { is: statusColor === 'destructive' ? 'AlertCircleIcon' : statusColor === 'success' ? 'CheckCircleIcon' : statusColor === 'info' ? 'CheckCircleIcon' : 'ClockIcon', class: 'h-3 w-3' }),
            statusLabel
          ]),
        ]),
        
        h('p', { class: 'text-sm text-muted-foreground truncate mt-1' }, props.deadline.course),
        
        h('div', { class: 'flex items-center gap-3 mt-2 text-xs text-muted-foreground' }, [
          h('span', { class: 'flex items-center gap-1' }, [
            h('component', { is: 'ClockIcon', class: 'h-3 w-3' }),
            formatDate(props.deadline.due)
          ]),
          h('span', { class: 'flex items-center gap-1' }, [
            h('component', { is: typeIcons[props.deadline.type as keyof typeof typeIcons] || 'CalendarIcon', class: 'h-3 w-3' }),
            props.deadline.type
          ]),
          h('span', { class: 'flex items-center gap-1' }, [
            h('StarIcon', { class: 'h-3 w-3 fill-current text-warning-500' }),
            props.deadline.score !== undefined ? `${props.deadline.score}%` : 'Not graded'
          ]),
        ]),
      ]),
      
      // Actions
      h('div', { class: 'flex-shrink-0 flex items-center gap-1' }, [
        h('Button', { 
          variant: 'ghost', 
          size: 'icon', 
          class: 'text-muted-foreground hover:text-foreground',
          onClick: () => window.location.href = `/courses/${props.deadline.courseId}/assignments/${props.deadline.id}`
        }, [
          h('component', { is: 'ArrowRightIcon', class: 'h-4 w-4' })
        ]),
      ]),
    )
  }
})

// Icon imports
const ClockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const AlertCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z' })]) }})
const CheckCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const FileTextIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })]) }})
const AlertTriangleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z' })]) }})
const UsersIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })]) }})
const CalendarIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z' })]) }})
const FlagIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1v12z' })]) }})
</script>