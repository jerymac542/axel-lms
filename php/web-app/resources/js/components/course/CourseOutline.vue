<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { cn } from '@/utils/cn'
import { 
  ChevronRightIcon, ChevronDownIcon, PlayIcon, CheckCircleIcon, 
  ClockIcon, LockIcon, BookOpenIcon, VideoIcon, FileTextIcon, 
  HelpCircleIcon, FlagIcon, MoreHorizontalIcon, PencilIcon, TrashIcon,
  GripVerticalIcon, ExternalLinkIcon
} from '@/components/ui/icons'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { Tooltip, TooltipTrigger, TooltipContent } from '@/components/ui/tooltip'
import { formatDuration } from '@/utils/cn'
import { useAuth } from '@/composables/useAuth'
import { useToast } from '@/composables/useToast'

interface Props {
  course: any
  enrollment: any
  currentModule?: any
  currentLesson?: any
  showProgress?: boolean
  editable?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showProgress: true,
  editable: false,
})

const auth = useAuth()
const toast = useToast()

const expandedModules = ref<Set<string>>(new Set())
const draggedItem = ref<{ type: 'module' | 'lesson', data: any } | null>(null)
const dragOverItem = ref<{ type: 'module' | 'lesson', data: any } | null>(null)

const modules = computed(() => props.course.modules || [])
const totalLessons = computed(() => 
  modules.value.reduce((sum: number, m: any) => sum + (m.lessons?.length || 0), 0)
)
const completedLessons = computed(() => 
  modules.value.reduce((sum: number, m: any) => 
    sum + (m.lessons?.filter((l: any) => l.completed).length || 0), 0)
)

const isLessonCompleted = (lesson: any) => {
  return lesson.completed || props.enrollment?.completed_lessons?.includes(lesson.id)
}

const isLessonLocked = (lesson: any, index: number, moduleIndex: number) => {
  if (!props.enrollment || props.enrollment.status !== 'active') return true
  if (isLessonCompleted(lesson)) return false
  
  // Check if previous lesson is completed
  let lessonCount = 0
  for (let i = 0; i <= moduleIndex; i++) {
    const mod = modules.value[i]
    for (let j = 0; j < (mod.lessons?.length || 0); j++) {
      lessonCount++
      if (i === moduleIndex && j === index) {
        // This is our lesson
        return lessonCount > 1 && !isPrevLessonCompleted(moduleIndex, index)
      }
    }
  }
  return false
}

const isPrevLessonCompleted = (moduleIndex: number, lessonIndex: number) => {
  if (lessonIndex > 0) {
    return isLessonCompleted(modules.value[moduleIndex].lessons[lessonIndex - 1])
  } else if (moduleIndex > 0) {
    const prevModule = modules.value[moduleIndex - 1]
    const lastLesson = prevModule.lessons[prevModule.lessons.length - 1]
    return isLessonCompleted(lastLesson)
  }
  return false
}

const isModuleExpanded = (moduleId: string) => expandedModules.value.has(moduleId)
const toggleModule = (moduleId: string) => {
  if (expandedModules.value.has(moduleId)) {
    expandedModules.value.delete(moduleId)
  } else {
    expandedModules.value.add(moduleId)
  }
}

const selectLesson = (lesson: any, module: any) => {
  if (isLessonLocked(lesson, module.lessons.indexOf(lesson), modules.value.indexOf(module))) {
    toast.warning('Complete previous lesson first')
    return
  }
  // Emit event to parent
  window.dispatchEvent(new CustomEvent('lesson-selected', { detail: { lesson, module } }))
}

const toggleLessonComplete = (lesson: any) => {
  // Emit event to parent
  window.dispatchEvent(new CustomEvent('lesson-toggle', { detail: lesson }))
}

const getLessonIcon = (type: string) => {
  switch (type) {
    case 'video': return VideoIcon
    case 'reading': return FileTextIcon
    case 'quiz': return HelpCircleIcon
    case 'assignment': return FlagIcon
    case 'discussion': return 'MessageSquareIcon'
    default: return BookOpenIcon
  }
}

const getLessonTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    video: 'Video',
    reading: 'Reading',
    quiz: 'Quiz',
    assignment: 'Assignment',
    discussion: 'Discussion',
    live: 'Live Session',
  }
  return labels[type] || type
}

// Drag and drop handlers
const handleDragStart = (event: DragEvent, type: 'module' | 'lesson', data: any) => {
  if (!props.editable) return
  draggedItem.value = { type, data }
  event.dataTransfer!.effectAllowed = 'move'
  event.dataTransfer!.setData('text/plain', JSON.stringify({ type, id: data.id }))
}

const handleDragOver = (event: DragEvent, type: 'module' | 'lesson', data: any) => {
  if (!props.editable) return
  event.preventDefault()
  event.dataTransfer!.dropEffect = 'move'
  dragOverItem.value = { type, data }
}

const handleDragLeave = () => {
  dragOverItem.value = null
}

const handleDrop = (event: DragEvent, targetType: 'module' | 'lesson', targetData: any) => {
  if (!props.editable || !draggedItem.value) return
  event.preventDefault()
  
  // Emit reorder event
  window.dispatchEvent(new CustomEvent('outline-reorder', { 
    detail: { 
      dragged: draggedItem.value, 
      target: { type: targetType, data: targetData } 
    } 
  }))
  
  draggedItem.value = null
  dragOverItem.value = null
}

const handleDragEnd = () => {
  draggedItem.value = null
  dragOverItem.value = null
}
</script>

<template>
  <div class="space-y-2">
    <!-- Header -->
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center gap-3">
        <h3 class="font-semibold text-lg">Course Content</h3>
        <Badge variant="secondary">{{ totalLessons }} lessons</Badge>
        <Badge v-if="props.course.duration" variant="outline">
          <ClockIcon class="h-3 w-3" />
          {{ props.course.duration }}
        </Badge>
      </div>
      
      <div v-if="props.editable" class="flex items-center gap-2">
        <Button variant="outline" size="sm" class="gap-1">
          <PlusIcon class="h-3 w-3" />
          Add Module
        </Button>
        <Button variant="ghost" size="icon" @click="expandedModules.value = new Set(modules.value.map(m => m.id))">
          <ChevronDownIcon class="h-4 w-4" />
        </Button>
        <Button variant="ghost" size="icon" @click="expandedModules.value.clear()">
          <ChevronRightIcon class="h-4 w-4" />
        </Button>
      </div>
    </div>

    <!-- Modules List -->
    <div class="space-y-3" role="list" aria-label="Course modules">
      <div 
        v-for="(module, moduleIndex) in modules" 
        :key="module.id" 
        class="border border-border rounded-xl overflow-hidden bg-card"
        role="listitem"
      >
        <!-- Module Header -->
        <button
          @click="toggleModule(module.id)"
          class="w-full flex items-center gap-3 p-4 text-left transition-colors hover:bg-accent/50"
          :aria-expanded="isModuleExpanded(module.id)"
          :class="cn(
            draggedItem.value?.type === 'module' && draggedItem.value.data.id === module.id && 'opacity-50',
            dragOverItem.value?.type === 'module' && dragOverItem.value.data.id === module.id && 'ring-2 ring-primary-500'
          )"
          draggable="editable"
          @dragstart="handleDragStart($event, 'module', module)"
          @dragover="handleDragOver($event, 'module', module)"
          @dragleave="handleDragLeave"
          @drop="handleDrop($event, 'module', module)"
          @dragend="handleDragEnd"
        >
          <GripVerticalIcon v-if="editable" class="h-5 w-5 text-muted-foreground/50 cursor-grab" />
          
          <ChevronRightIcon 
            class="h-5 w-5 text-muted-foreground flex-shrink-0 transition-transform duration-200" 
            :class="{ 'rotate-90': isModuleExpanded(module.id) }"
          />
          
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <h4 class="font-medium truncate">{{ module.title }}</h4>
              <Badge v-if="module.type === 'project'" variant="secondary">{{ module.type }}</Badge>
              <Badge v-if="module.locked" variant="outline" class="gap-1">
                <LockIcon class="h-3 w-3" />
                Locked
              </Badge>
            </div>
            <p v-if="module.description" class="text-sm text-muted-foreground truncate mt-1">{{ module.description }}</p>
          </div>
          
          <div class="flex items-center gap-3 text-sm text-muted-foreground">
            <span class="flex items-center gap-1">
              <BookOpenIcon class="h-4 w-4" />
              {{ module.lessons?.length || 0 }} lessons
            </span>
            <span v-if="module.duration" class="flex items-center gap-1">
              <ClockIcon class="h-4 w-4" />
              {{ module.duration }}
            </span>
            <span class="flex items-center gap-1">
              <CheckCircleIcon class="h-4 w-4" />
              {{ getModuleCompletedCount(module) }}/{{ module.lessons?.length || 0 }}
            </span>
          </div>
          
          <div v-if="editable" class="flex items-center gap-1">
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button variant="ghost" size="icon" class="h-8 w-8">
                  <MoreHorizontalIcon class="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem>
                  <PencilIcon class="h-4 w-4" />
                  Edit Module
                </DropdownMenuItem>
                <DropdownMenuItem @click="duplicateModule(module)">
                  <CopyIcon class="h-4 w-4" />
                  Duplicate
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem class="text-destructive" @click="deleteModule(module)">
                  <TrashIcon class="h-4 w-4" />
                  Delete
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </button>

        <!-- Module Lessons -->
        <div 
          v-show="isModuleExpanded(module.id)" 
          class="border-t border-border bg-muted/30 animate-slide-down-2"
          role="list"
          aria-label="{{ module.title }} lessons"
        >
          <div v-for="(lesson, lessonIndex) in module.lessons" :key="lesson.id" 
            class="flex items-center gap-3 px-4 py-3 transition-colors hover:bg-accent/50 border-t border-border/50 last:border-b-0"
            role="listitem"
            :class="cn(
              isLessonCompleted(lesson) && 'bg-success-50 dark:bg-success-900/20',
              isLessonLocked(lesson, lessonIndex, moduleIndex) && 'opacity-50',
              draggedItem.value?.type === 'lesson' && draggedItem.value.data.id === lesson.id && 'opacity-50',
              dragOverItem.value?.type === 'lesson' && dragOverItem.value.data.id === lesson.id && 'ring-2 ring-primary-500'
            )"
            draggable="editable"
            @dragstart="handleDragStart($event, 'lesson', lesson)"
            @dragover="handleDragOver($event, 'lesson', lesson)"
            @dragleave="handleDragLeave"
            @drop="handleDrop($event, 'lesson', lesson)"
            @dragend="handleDragEnd"
          >
            <GripVerticalIcon v-if="editable" class="h-5 w-5 text-muted-foreground/50 cursor-grab" />
            
            <!-- Completion Checkbox -->
            <button
              @click.stop="toggleLessonComplete(lesson)"
              :disabled="isLessonLocked(lesson, lessonIndex, moduleIndex) || !isInProgress"
              :aria-pressed="isLessonCompleted(lesson)"
              class="flex-shrink-0 w-5 h-5 rounded border-2 flex items-center justify-center transition-colors"
              :class="cn(
                isLessonCompleted(lesson) ? 'bg-success-500 border-success-500 text-white' : 
                'border-border bg-transparent',
                isLessonLocked(lesson, lessonIndex, moduleIndex) && 'opacity-30 cursor-not-allowed'
              )"
            >
              <CheckCircleIcon v-if="isLessonCompleted(lesson)" class="h-3.5 w-3.5" />
              <LockIcon v-else-if="isLessonLocked(lesson, lessonIndex, moduleIndex)" class="h-3.5 w-3.5" />
            </button>
            
            <!-- Lesson Icon & Type -->
            <div class="flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center bg-muted"
              :class="isLessonCompleted(lesson) ? 'bg-success-100 dark:bg-success-900/30' : ''">
              <component 
                :is="getLessonIcon(lesson.type)" 
                class="h-4 w-4" 
                :class="isLessonCompleted(lesson) ? 'text-success-600' : 'text-muted-foreground'" 
              />
            </div>
            
            <!-- Lesson Info -->
            <div class="flex-1 min-w-0" @click="selectLesson(lesson, module)">
              <div class="flex items-center gap-2">
                <span class="font-medium truncate" :class="isLessonLocked(lesson, lessonIndex, moduleIndex) ? 'text-muted-foreground' : 'text-foreground'">
                  {{ lesson.title }}
                </span>
                <Badge v-if="lesson.type" variant="outline" class="text-[10px] gap-1">
                  <component :is="getLessonIcon(lesson.type)" class="h-2.5 w-2.5" />
                  {{ getLessonTypeLabel(lesson.type) }}
                </Badge>
                <Badge v-if="lesson.free" variant="secondary" class="text-[10px]">Free Preview</Badge>
                <Badge v-if="lesson.new" variant="warning" class="text-[10px]">New</Badge>
              </div>
              <p v-if="lesson.description" class="text-xs text-muted-foreground truncate mt-0.5">{{ lesson.description }}</p>
            </div>
            
            <!-- Duration & Progress -->
            <div class="flex items-center gap-3 text-xs text-muted-foreground">
              <span v-if="lesson.duration" class="flex items-center gap-1">
                <ClockIcon class="h-3 w-3" />
                {{ lesson.duration }}
              </span>
              <Tooltip v-if="showProgress && lesson.type === 'video' && lesson.progress !== undefined">
                <TooltipTrigger as-child>
                  <Progress :value="lesson.progress" class="w-24 h-1.5" />
                </TooltipTrigger>
                <TooltipContent>{{ lesson.progress }}% watched</TooltipContent>
              </Tooltip>
            </div>
            
            <!-- Actions -->
            <div v-if="editable" class="flex items-center gap-1">
              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button variant="ghost" size="icon" class="h-8 w-8">
                    <MoreHorizontalIcon class="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem>
                    <PencilIcon class="h-4 w-4" />
                    Edit Lesson
                  </DropdownMenuItem>
                  <DropdownMenuItem @click="duplicateLesson(lesson)">
                    <CopyIcon class="h-4 w-4" />
                    Duplicate
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem class="text-destructive" @click="deleteLesson(lesson)">
                    <TrashIcon class="h-4 w-4" />
                    Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
            
            <!-- Lock indicator for locked lessons -->
            <div v-if="isLessonLocked(lesson, lessonIndex, moduleIndex) && !editable" class="flex items-center gap-1 text-xs text-muted-foreground">
              <LockIcon class="h-3 w-3" />
              <span>Locked</span>
            </div>
          </div>
          
          <!-- Drop zone between lessons -->
          <div 
            v-if="editable && dragOverItem.value?.type === 'lesson' && dragOverItem.value.data.id === lesson.id"
            class="h-2 bg-primary-500/20 border-t-2 border-primary-500 border-dashed mx-4"
          />
        </div>
        
        <!-- Add lesson button -->
        <button 
          v-if="editable && isModuleExpanded(module.id)"
          @click="addLesson(module)"
          class="w-full flex items-center justify-center gap-2 px-4 py-3 text-muted-foreground hover:text-foreground hover:bg-accent/50 border-t border-border/50 transition-colors"
        >
          <PlusIcon class="h-4 w-4" />
          <span>Add Lesson</span>
        </button>
        
        <!-- Drop zone at end of module -->
        <div 
          v-if="editable && dragOverItem.value?.type === 'module' && dragOverItem.value.data.id === module.id"
          class="h-4 bg-primary-500/20 border-t-2 border-primary-500 border-dashed"
        />
      </div>
      
      <!-- Empty state -->
      <div v-if="modules.length === 0" class="text-center py-12 border-2 border-dashed border-border rounded-xl">
        <BookOpenIcon class="h-12 w-12 mx-auto text-muted-foreground/50 mb-4" />
        <p class="text-muted-foreground mb-4">No modules yet</p>
        <Button v-if="editable" @click="addModule" class="gap-2">
          <PlusIcon class="h-4 w-4" />
          Create First Module
        </Button>
      </div>
    </div>

    <!-- Floating add module button -->
    <button 
      v-if="editable" 
      @click="addModule"
      class="fixed bottom-6 right-6 z-50 rounded-full bg-primary-600 p-3 text-primary-foreground shadow-lg hover:bg-primary-700 transition-colors"
      aria-label="Add module"
    >
      <PlusIcon class="h-6 w-6" />
    </button>
  </div>
</template>

<script>
const PlusIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4v16m8-8H4' })]) }})
const CopyIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 012-2h10a2 2 0 012 2v12a2 2 0 01-2 2h-2' })]) }})
const MessageSquareIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 12h4M8 12l2-2m0 0l2 2m-2-2v8m-8 0v8m0-8h8m-8 0h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949M3 12c0 4.418 4.03 8 9 8a9.863 9.863 0 014.255-.949' })]) }})
const VideoIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('polygon', { points: '23 7 16 12 23 17 23 7' }), h('rect', { x: '1', y: '5', width: '15', height: '14', rx: '2', 'stroke-width': '2' })]) }})
</script>