<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { 
  TargetIcon, TrendingUpIcon, AwardIcon, BookOpenIcon,
  ArrowRightIcon, PlusIcon, SearchIcon, FilterIcon
} from '@/components/ui/icons'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'

interface Props {
  skills: Array<{
    id: string
    name: string
    category: string
    level: number // 0-100
    targetLevel?: number
    trend?: 'up' | 'down' | 'stable'
    coursesCount?: number
    lastPracticed?: string
    color?: 'primary' | 'success' | 'warning' | 'info' | 'accent' | 'destructive'
  }>
  maxItems?: number
  showAllLink?: boolean
  onSelectSkill?: (skill: any) => void
}

const props = withDefaults(defineProps<Props>(), {
  maxItems: 6,
  showAllLink: true,
})

const activeTab = ref<'all' | 'improving' | 'needs_work' | 'mastered'>('all')

const filteredSkills = computed(() => {
  let skills = [...props.skills].sort((a, b) => b.level - a.level)
  
  switch (activeTab.value) {
    case 'improving':
      return skills.filter(s => s.trend === 'up')
    case 'needs_work':
      return skills.filter(s => s.level < 50)
    case 'mastered':
      return skills.filter(s => s.level >= 80)
    default:
      return skills
  }
})

const displayedSkills = computed(() => filteredSkills.value.slice(0, props.maxItems))

const colorClasses = {
  primary: 'bg-primary-600',
  success: 'bg-success-600',
  warning: 'bg-warning-600',
  info: 'bg-info-600',
  accent: 'bg-accent-600',
  destructive: 'bg-destructive-600',
}
</script>

<template>
  <Card>
    <CardHeader class="pb-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <TargetIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Skill Progress</CardTitle>
            <CardDescription>Track your competency development</CardDescription>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <Tabs default-value="all" class="hidden sm:flex">
            <TabsList class="grid grid-cols-4 bg-muted p-1 rounded-lg">
              <TabsTrigger value="all" class="text-xs py-1.5">All</TabsTrigger>
              <TabsTrigger value="improving" class="text-xs py-1.5">
                <TrendingUpIcon class="h-3 w-3 mr-1" /> Improving
              </TabsTrigger>
              <TabsTrigger value="needs_work" class="text-xs py-1.5">Needs Work</TabsTrigger>
              <TabsTrigger value="mastered" class="text-xs py-1.5">Mastered</TabsTrigger>
            </TabsList>
          </Tabs>
          <Button v-if="showAllLink" variant="ghost" size="sm" class="gap-1 ml-auto">
            View All
            <ArrowRightIcon class="h-3 w-3" />
          </Button>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <!-- Skills Grid -->
        <div class="space-y-4">
          <div v-for="skill in displayedSkills" :key="skill.id" class="group">
            <div class="flex items-center justify-between mb-2">
              <div class="flex items-center gap-3">
                <div :class="cn('p-2 rounded-lg', `bg-${skill.color}-100 dark:bg-${skill.color}-900/30`)" >
                  <component :is="skill.icon || BookOpenIcon" :class="cn('h-5 w-5', `text-${skill.color}-600 dark:text-${skill.color}-400`)" />
                </div>
                <div>
                  <h4 class="font-medium truncate">{{ skill.name }}</h4>
                  <div class="flex items-center gap-2 text-xs text-muted-foreground">
                    <Badge :variant="skill.level >= 80 ? 'success' : skill.level >= 50 ? 'default' : 'secondary'" class="gap-1">
                      <AwardIcon class="h-3 w-3" />
                      {{ skill.level >= 80 ? 'Mastered' : skill.level >= 50 ? 'Proficient' : 'Developing' }}
                    </Badge>
                    <span v-if="skill.category">{{ skill.category }}</span>
                    <span v-if="skill.coursesCount" class="flex items-center gap-1">
                      <BookOpenIcon class="h-3 w-3" />
                      {{ skill.coursesCount }}
                    </span>
                  </div>
                </div>
              </div>
              
              <div class="flex items-center gap-2">
                <Badge :variant="skill.trend === 'up' ? 'success' : skill.trend === 'down' ? 'destructive' : 'secondary'" class="gap-1">
                  <TrendingUpIcon v-if="skill.trend === 'up'" class="h-3 w-3" />
                  <TrendingUpIcon v-else-if="skill.trend === 'down'" class="h-3 w-3 rotate-180" />
                  <span v-else>Stable</span>
                </Badge>
              </div>
            </div>
            
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2 flex-1 min-w-0">
                <span class="text-sm font-medium tabular-nums">{{ skill.level }}%</span>
                <span v-if="skill.targetLevel" class="text-xs text-muted-foreground">Target: {{ skill.targetLevel }}%</span>
              </div>
              <Button 
                variant="ghost" 
                size="icon" 
                class="text-muted-foreground hover:text-foreground opacity-0 group-hover:opacity-100 transition-opacity"
                @click="$emit('select-skill', skill)"
              >
                <ChevronRightIcon class="h-4 w-4" />
              </Button>
            </div>
            
            <!-- Progress Bar -->
            <Progress 
              :value="skill.level" 
              class="h-2 mt-2" 
              :class="colorClasses[skill.color]"
            />
            
            <!-- Meta Info -->
            <div class="flex items-center justify-between text-xs text-muted-foreground mt-2 pt-2 border-t border-border">
              <span v-if="skill.lastPracticed">
                <CalendarIcon class="h-3 w-3 mr-1" />
                Last practiced {{ formatRelativeTime(skill.lastPracticed) }}
              </span>
              <span v-if="skill.targetLevel && skill.level < skill.targetLevel">
                {{ skill.targetLevel - skill.level }}% to target
              </span>
            </div>
          </div>
        </div>

        <!-- View All Link -->
        <div v-if="filteredSkills.length > maxItems && showAllLink" class="text-center pt-4 border-t border-border">
          <Button variant="outline" @click="$emit('view-all')">
            View all {{ filteredSkills.length }} skills
            <ArrowRightIcon class="h-4 w-4" />
          </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script>
const CalendarIcon = defineComponent({ render() { return h('svg', { class: 'h-3 w-3', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z' })]) }})
</script>