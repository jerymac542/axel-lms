<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import { Badge } from '@/components/ui/badge'
import { PlayIcon, ClockIcon, CheckCircleIcon, ArrowRightIcon, BookOpenIcon, LockIcon } from '@/components/ui/icons'
import { formatDuration } from '@/utils/cn'

interface Props {
  courses: any[]
  maxItems?: number
  showProgress?: boolean
  compact?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  maxItems: 3,
  showProgress: true,
  compact: false,
})

const inProgressCourses = computed(() => 
  props.courses
    .filter(c => c.enrollment?.status === 'active' || c.enrollment?.status === 'in_progress')
    .sort((a, b) => (b.enrollment?.last_accessed || 0) - (a.enrollment?.last_accessed || 0))
    .slice(0, props.maxItems)
)

const hasCourses = computed(() => inProgressCourses.value.length > 0)
</script>

<template>
  <Card v-if="hasCourses" :class="cn(compact && 'max-w-xs')">
    <CardHeader class="pb-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <PlayIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Continue Learning</CardTitle>
            <p class="text-sm text-muted-foreground">Pick up where you left off</p>
          </div>
        </div>
        <Button v-if="!compact" variant="ghost" size="sm" class="gap-1">
          <ArrowRightIcon class="h-3 w-3" />
          View All
        </Button>
      </CardHeader>
      
      <CardContent :class="cn(compact ? 'p-4 pt-0' : 'pt-0')">
        <div class="space-y-3" v-if="!compact">
          <div v-for="course in inProgressCourses" :key="course.id" class="group relative">
            <Button 
              variant="ghost" 
              class="w-full justify-start gap-4 p-3 rounded-xl hover:bg-accent/50 transition-colors text-left"
              @click="$emit('select-course', course)"
            >
              <!-- Thumbnail -->
              <div class="relative w-16 h-9 rounded-lg overflow-hidden bg-muted flex-shrink-0">
                <img 
                  v-if="course.thumbnail_url" 
                  :src="course.thumbnail_url" 
                  :alt="course.title"
                  class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                  loading="lazy"
                />
                <div v-else class="w-full h-full flex items-center justify-center">
                  <BookOpenIcon class="h-4 w-4 text-muted-foreground/50" />
                </div>
                <div class="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                  <PlayIcon class="h-5 w-5 text-white" />
                </div>
              </div>
              
              <!-- Info -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                  <h4 class="font-medium truncate">{{ course.title }}</h4>
                  <Badge v-if="course.enrollment?.status === 'completed'" variant="success" class="gap-1">
                    <CheckCircleIcon class="h-3 w-3" />
                    Completed
                  </Badge>
                  <Badge v-else-if="course.enrollment?.status === 'active'" variant="default" class="gap-1">
                    In Progress
                  </Badge>
                  <Badge v-else variant="outline" class="gap-1">
                    <LockIcon class="h-3 w-3" />
                    Locked
                  </Badge>
                </div>
                <p class="text-sm text-muted-foreground truncate mb-2">{{ course.instructor?.name }}</p>
                
                <!-- Progress -->
                <div v-if="showProgress && course.enrollment?.progress !== undefined" class="space-y-1">
                  <div class="flex items-center justify-between text-xs">
                    <span class="font-medium">{{ course.enrollment.progress }}% complete</span>
                    <span class="text-muted-foreground">{{ course.enrollment.completed_lessons || 0 }}/{{ course.lessons_count || 0 }} lessons</span>
                  </div>
                  <Progress :value="course.enrollment.progress" class="h-1.5" />
                </div>
                
                <div v-if="course.next_lesson" class="mt-2 flex items-center gap-2 text-xs text-muted-foreground">
                  <ClockIcon class="h-3 w-3" />
                  <span>Next: {{ course.next_lesson }}</span>
                </div>
              </div>
              
              <!-- Continue Button -->
              <Button 
                v-if="course.enrollment?.status === 'active'" 
                size="sm" 
                class="gap-2 ml-auto flex-shrink-0"
                @click.stop="$emit('select-course', course)"
              >
                <PlayIcon class="h-3 w-3" />
                <span class="hidden sm:inline">Continue</span>
              </Button>
            </Button>
          </div>
        </div>

        <!-- Compact Version -->
        <div v-else class="space-y-2">
          <div v-for="course in inProgressCourses" :key="course.id" class="flex items-center gap-3 p-3 rounded-lg hover:bg-accent/50 transition-colors">
            <div class="relative w-12 h-7 rounded-lg overflow-hidden bg-muted flex-shrink-0">
              <img 
                v-if="course.thumbnail_url" 
                :src="course.thumbnail_url" 
                :alt="course.title"
                class="w-full h-full object-cover"
                loading="lazy"
              />
              <div v-else class="w-full h-full flex items-center justify-center">
                <BookOpenIcon class="h-4 w-4 text-muted-foreground/50" />
              </div>
            </div>
            
            <div class="flex-1 min-w-0">
              <h4 class="font-medium truncate">{{ course.title }}</h4>
              <div class="flex items-center gap-2 text-xs text-muted-foreground mt-1">
                <span>{{ course.instructor?.name }}</span>
                <Badge v-if="course.enrollment?.progress !== undefined" variant="secondary">{{ course.enrollment.progress }}%</Badge>
              </div>
            </div>
            
            <Progress v-if="showProgress && course.enrollment?.progress !== undefined" :value="course.enrollment.progress" class="w-24 h-1.5" />
            
            <Button 
              v-if="course.enrollment?.status === 'active'" 
              size="icon" 
              variant="default"
              @click.stop="$emit('select-course', course)"
            >
              <PlayIcon class="h-4 w-4" />
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Empty State -->
    <Card v-else :class="cn(compact && 'max-w-xs')">
      <CardContent class="py-12 text-center">
        <div class="mx-auto mb-4 p-3 rounded-full bg-muted">
          <BookOpenIcon class="h-8 w-8 text-muted-foreground" />
        </div>
        <h3 class="font-semibold mb-1">No courses in progress</h2>
        <p class="text-sm text-muted-foreground mb-4">Start your learning journey today</p>
        <Button @click="$emit('browse-courses')">
          <BookOpenIcon class="h-4 w-4" />
          Browse Courses
        </Button>
      </CardContent>
    </Card>
  </div>
</template>