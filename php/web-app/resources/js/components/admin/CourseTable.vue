<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from '@/components/ui/table'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'
import { Pagination } from '@/components/ui/pagination'
import { Skeleton } from '@/components/ui/skeleton'
import { 
  SearchIcon, FilterIcon, MoreHorizontalIcon, PlusIcon,
  BookOpenIcon, UserCheckIcon, SettingsIcon, ShieldIcon,
  EyeIcon, EditIcon, TrashIcon, DownloadIcon, CopyIcon,
  ChevronDownIcon, ChevronUpIcon, CalendarIcon, ClockIcon,
  UsersIcon, CheckCircleIcon, AlertCircleIcon
} from '@/components/ui/icons'
import { formatRelativeTime, formatDate } from '@/utils/cn'

interface Props {
  courses?: any[]
  loading?: boolean
  onCourseAction?: (action: string, course: any) => void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

const searchQuery = ref('')
const statusFilter = ref('all')
const visibilityFilter = ref('all')
const sortBy = ref('created_at')
const sortOrder = ref<'asc' | 'desc'>('desc')
const currentPage = ref(1)
const itemsPerPage = 20

const mockCourses = [
  { id: '1', title: 'Advanced React Patterns', description: 'Master advanced React patterns', thumbnail: '/courses/react.jpg', status: 'published', visibility: 'public', instructor: { name: 'Sarah Chen' }, enrollments: 1247, completionRate: 78, avgScore: 87, createdAt: '2023-06-15', updatedAt: '2024-01-10' },
  { id: '2', title: 'Machine Learning Fundamentals', description: 'Introduction to ML concepts', thumbnail: '/courses/ml.jpg', status: 'published', visibility: 'public', instructor: { name: 'Dr. Alex Kumar' }, enrollments: 892, completionRate: 65, avgScore: 82, createdAt: '2023-08-22', updatedAt: '2024-01-08' },
  { id: '3', title: 'System Design Interview Prep', description: 'Master system design concepts', thumbnail: '/courses/sysdesign.jpg', status: 'published', visibility: 'organization', instructor: { name: 'Marcus Johnson' }, enrollments: 567, completionRate: 72, avgScore: 85, createdAt: '2023-11-10', updatedAt: '2024-01-05' },
  { id: '4', title: 'GraphQL API Development', description: 'Build production-ready GraphQL APIs', thumbnail: '/courses/graphql.jpg', status: 'draft', visibility: 'private', instructor: { name: 'Emma Wilson' }, enrollments: 0, completionRate: 0, avgScore: 0, createdAt: '2024-01-05', updatedAt: '2024-01-12' },
  { id: '5', title: 'Advanced CSS Layouts', description: 'Modern CSS techniques', thumbnail: '/courses/css.jpg', status: 'published', visibility: 'public', instructor: { name: 'Maria Santos' }, enrollments: 342, completionRate: 81, avgScore: 89, createdAt: '2023-09-01', updatedAt: '2024-01-09' },
  { id: '6', title: 'Kubernetes for Developers', description: 'Deploy and manage containers', thumbnail: '/courses/k8s.jpg', status: 'archived', visibility: 'tenant', instructor: { name: 'Lisa Wong' }, enrollments: 156, completionRate: 58, avgScore: 76, createdAt: '2023-07-15', updatedAt: '2023-12-20' },
]

const courses = computed(() => {
  let result = props.courses || mockCourses
  
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(c => 
      c.title.toLowerCase().includes(query) ||
      c.instructor.name.toLowerCase().includes(query)
    )
  }
  
  if (statusFilter.value !== 'all') {
    result = result.filter(c => c.status === statusFilter.value)
  }
  
  if (visibilityFilter.value !== 'all') {
    result = result.filter(c => c.visibility === visibilityFilter.value)
  }
  
  result.sort((a, b) => {
    let aVal = a[sortBy.value]
    let bVal = b[sortBy.value]
    if (typeof aVal === 'string') aVal = aVal.toLowerCase()
    if (typeof bVal === 'string') bVal = bVal.toLowerCase()
    return sortOrder.value === 'asc' ? (aVal > bVal ? 1 : -1) : (aVal < bVal ? 1 : -1)
  })
  
  return result
})

const paginatedCourses = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  return courses.value.slice(start, start + itemsPerPage)
})

const totalPages = computed(() => Math.ceil(courses.value.length / itemsPerPage))

const handleSort = (column: string) => {
  if (sortBy.value === column) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortBy.value = column
    sortOrder.value = 'asc'
  }
}

const getStatusBadge = (status: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'destructive' | 'info' | 'secondary'> = {
    published: 'success',
    draft: 'secondary',
    archived: 'destructive',
    review: 'warning',
  }
  return variants[status] || 'default'
}

const getVisibilityBadge = (visibility: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'info' | 'secondary'> = {
    public: 'success',
    organization: 'info',
    tenant: 'warning',
    private: 'secondary',
  }
  return variants[visibility] || 'default'
}

const handleAction = (action: string, course: any) => {
  props.onCourseAction?.(action, course)
}
</script>

<template>
  <Card>
    <CardHeader class="pb-3">
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <BookOpenIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Course Management</CardTitle>
            <CardDescription>{{ courses.length }} total courses</CardDescription>
          </div>
        </div>
        
        <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
          <div class="relative flex-1 max-w-sm">
            <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              v-model="searchQuery"
              placeholder="Search courses..."
              class="pl-10"
              @input="currentPage = 1"
            />
          </div>
          
          <Select v-model="statusFilter" class="w-32">
            <SelectTrigger>
              <SelectValue placeholder="Status" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Statuses</SelectItem>
              <SelectItem value="published">Published</SelectItem>
              <SelectItem value="draft">Draft</SelectItem>
              <SelectItem value="archived">Archived</SelectItem>
              <SelectItem value="review">In Review</SelectItem>
            </SelectContent>
          </Select>
          
          <Select v-model="visibilityFilter" class="w-32">
            <SelectTrigger>
              <SelectValue placeholder="Visibility" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All</SelectItem>
              <SelectItem value="public">Public</SelectItem>
              <SelectItem value="organization">Organization</SelectItem>
              <SelectItem value="tenant">Tenant</SelectItem>
              <SelectItem value="private">Private</SelectItem>
            </SelectContent>
          </Select>
          
          <Button @click="$emit('create-course')" class="gap-2">
            <PlusIcon class="h-4 w-4" />
            Create Course
          </Button>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <div class="overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead class="cursor-pointer" @click="handleSort('title')">
                  <div class="flex items-center gap-1">
                    Course
                    <ChevronDownIcon v-if="sortBy === 'title' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'title' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden md:table-cell" @click="handleSort('instructor')">
                  <div class="flex items-center gap-1">
                    Instructor
                    <ChevronDownIcon v-if="sortBy === 'instructor' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'instructor' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="handleSort('status')">
                  <div class="flex items-center gap-1">
                    Status
                    <ChevronDownIcon v-if="sortBy === 'status' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'status' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden lg:table-cell" @click="handleSort('visibility')">
                  <div class="flex items-center gap-1">
                    Visibility
                    <ChevronDownIcon v-if="sortBy === 'visibility' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'visibility' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden md:table-cell" @click="handleSort('enrollments')">
                  <div class="flex items-center gap-1">
                    Students
                    <ChevronDownIcon v-if="sortBy === 'enrollments' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'enrollments' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden lg:table-cell" @click="handleSort('completionRate')">
                  <div class="flex items-center gap-1">
                    Completion
                    <ChevronDownIcon v-if="sortBy === 'completionRate' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'completionRate' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="handleSort('updatedAt')">
                  <div class="flex items-center gap-1">
                    Updated
                    <ChevronDownIcon v-if="sortBy === 'updatedAt' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'updatedAt' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="w-56">Actions</TableHead>
              </TableRow>
            </TableHeader>
            
            <TableBody v-if="!loading">
              <TableRow v-for="course in paginatedCourses" :key="course.id" class="hover:bg-accent/50">
                <TableCell class="font-medium">
                  <div class="flex items-center gap-3">
                    <div class="w-12 h-7 rounded-lg bg-muted overflow-hidden flex-shrink-0">
                      <img v-if="course.thumbnail" :src="course.thumbnail" :alt="course.title" class="w-full h-full object-cover" />
                      <div v-else class="w-full h-full flex items-center justify-center">
                        <BookOpenIcon class="h-4 w-4 text-muted-foreground/50" />
                      </div>
                    </div>
                    <div>
                      <p class="font-medium truncate max-w-[200px]">{{ course.title }}</p>
                      <p class="text-sm text-muted-foreground truncate max-w-[200px]">{{ course.instructor?.name }}</p>
                    </div>
                  </div>
                </TableCell>
                <TableCell class="hidden md:table-cell">{{ course.instructor?.name }}</TableCell>
                <TableCell>
                  <Badge :variant="getStatusBadge(course.status)">{{ course.status }}</Badge>
                </TableCell>
                <TableCell class="hidden lg:table-cell">
                  <Badge :variant="getVisibilityBadge(course.visibility)">{{ course.visibility }}</Badge>
                </TableCell>
                <TableCell class="hidden md:table-cell">
                  <div class="flex items-center gap-1 text-sm">
                    <UsersIcon class="h-4 w-4 text-muted-foreground" />
                    {{ course.enrollments.toLocaleString() }}
                  </div>
                </TableCell>
                <TableCell class="hidden lg:table-cell">
                  <div class="flex items-center gap-1">
                    <div class="w-20 h-2 bg-muted rounded-full overflow-hidden">
                      <div class="h-full bg-primary-600 rounded-full" :style="{ width: course.completionRate + '%' }" />
                    </div>
                    <span class="text-sm font-medium ml-2">{{ course.completionRate }}%</span>
                  </div>
                </TableCell>
                <TableCell class="text-muted-foreground">{{ formatRelativeTime(course.updatedAt) }}</TableCell>
                <TableCell>
                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Button variant="ghost" size="icon" class="h-8 w-8">
                        <MoreHorizontalIcon class="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem @click="handleAction('view', course)">
                        <EyeIcon class="h-4 w-4" />
                        View
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('edit', course)">
                        <EditIcon class="h-4 w-4" />
                        Edit
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('duplicate', course)">
                        <CopyIcon class="h-4 w-4" />
                        Duplicate
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('analytics', course)">
                        <BarChartIcon class="h-4 w-4" />
                        Analytics
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem v-if="course.status === 'draft'" @click="handleAction('publish', course)" class="text-success">
                        <CheckCircleIcon class="h-4 w-4" />
                        Publish
                      </DropdownMenuItem>
                      <DropdownMenuItem v-if="course.status === 'published'" @click="handleAction('unpublish', course)" class="text-warning">
                        <PauseIcon class="h-4 w-4" />
                        Unpublish
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('export', course)">
                        <DownloadIcon class="h-4 w-4" />
                        Export
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem @click="handleAction('delete', course)" class="text-destructive">
                        <TrashIcon class="h-4 w-4" />
                        Delete
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TableCell>
              </TableRow>
            </TableBody>
            
            <TableBody v-else>
              <TableRow v-for="i in 5" :key="i">
                <TableCell><Skeleton class="h-10 w-24" /></TableCell>
                <TableCell><Skeleton class="h-4 w-24" /></TableCell>
                <TableCell><Skeleton class="h-4 w-20" /></TableCell>
                <TableCell><Skeleton class="h-4 w-20" /></TableCell>
                <TableCell><Skeleton class="h-4 w-20" /></TableCell>
                <TableCell><Skeleton class="h-4 w-20" /></TableCell>
                <TableCell><Skeleton class="h-4 w-20" /></TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
        
        <div v-if="totalPages > 1" class="flex items-center justify-between mt-6">
          <div class="text-sm text-muted-foreground">
            Showing {{ (currentPage - 1) * itemsPerPage + 1 }} to {{ Math.min(currentPage * itemsPerPage, courses.length) }} of {{ courses.length }} courses
          </div>
          <Pagination
            :current-page="currentPage"
            :total-pages="totalPages"
            @page-change="currentPage = $event"
          />
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script>
const BookOpenIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const PauseIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '6', y: '4', width: '4', height: '16' }), h('rect', { x: '14', y: '4', width: '4', height: '16' })]) }})
const BarChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 012-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' })]) }})
</script>