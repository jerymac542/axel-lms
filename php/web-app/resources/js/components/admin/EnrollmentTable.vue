<script setup lang="ts">
import { ref, computed } from 'vue'
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
  SearchIcon, FilterIcon, MoreHorizontalIcon, UserPlusIcon,
  UserCheckIcon, SettingsIcon, ShieldIcon, EyeIcon, EditIcon,
  TrashIcon, CopyIcon, DownloadIcon, UploadIcon,
  ChevronDownIcon, ChevronUpIcon, CalendarIcon, ClockIcon,
  UsersIcon, CheckCircleIcon, AlertCircleIcon, GraduationCapIcon,
  BookOpenIcon, ArrowRightIcon
} from '@/components/ui/icons'
import { formatRelativeTime, formatDate } from '@/utils/cn'

interface Props {
  enrollments?: any[]
  loading?: boolean
  onEnrollmentAction?: (action: string, enrollment: any) => void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

const searchQuery = ref('')
const statusFilter = ref('all')
const roleFilter = ref('all')
const courseFilter = ref('')
const sortBy = ref('enrolled_at')
const sortOrder = ref<'asc' | 'desc'>('desc')
const currentPage = ref(1)
const itemsPerPage = 20

const mockEnrollments = [
  { id: 'enr_1', courseId: 'course_1', courseTitle: 'Advanced React Patterns', userId: 'user_1', userName: 'Sarah Chen', userEmail: 'sarah@techcorp.com', userAvatar: '/avatars/sarah.jpg', organizationId: 'org_1', role: 'student', status: 'active', source: 'self', enrolledAt: '2024-01-10T10:30:00Z', startedAt: '2024-01-10T10:30:00Z', completedAt: null, droppedAt: null, progress: 65, score: 87, grade: 'A', enrolledBy: 'user_1' },
  { id: 'enr_2', courseId: 'course_2', courseTitle: 'Machine Learning Fundamentals', userId: 'user_2', userName: 'Marcus Johnson', userEmail: 'marcus@startup.io', userAvatar: '/avatars/marcus.jpg', organizationId: 'org_1', role: 'student', status: 'completed', source: 'self', enrolledAt: '2023-11-15T09:15:00Z', startedAt: '2023-11-15T09:15:00Z', completedAt: '2024-01-10T14:20:00Z', droppedAt: null, progress: 100, score: 92, grade: 'A+', enrolledBy: 'user_2' },
  { id: 'enr_3', courseId: 'course_3', courseTitle: 'System Design Interview Prep', userId: 'user_3', userName: 'Emily Rodriguez', userEmail: 'emily@university.edu', userAvatar: '/avatars/emily.jpg', organizationId: 'org_2', role: 'student', status: 'active', source: 'instructor', enrolledAt: '2024-01-05T16:45:00Z', startedAt: '2024-01-05T16:45:00Z', completedAt: null, droppedAt: null, progress: 45, score: 78, grade: null, enrolledBy: 'instructor_1' },
  { id: 'enr_4', courseId: 'course_1', courseTitle: 'Advanced React Patterns', userId: 'user_4', userName: 'David Park', userEmail: 'david@enterprise.com', userAvatar: '/avatars/david.jpg', organizationId: 'org_2', role: 'teaching_assistant', status: 'active', source: 'admin', enrolledAt: '2024-01-12T10:00:00Z', startedAt: '2024-01-12T10:00:00Z', completedAt: null, droppedAt: null, progress: 30, score: 85, grade: null, enrolledBy: 'admin_1' },
  { id: 'enr_5', courseId: 'course_4', courseTitle: 'GraphQL API Development', userId: 'user_5', userName: 'Lisa Wong', userEmail: 'lisa@corp.com', userAvatar: '/avatars/lisa.jpg', organizationId: 'org_3', role: 'instructor', status: 'active', source: 'self', enrolledAt: '2024-01-08T14:20:00Z', startedAt: '2024-01-08T14:20:00Z', completedAt: null, droppedAt: null, progress: 15, score: null, grade: null, enrolledBy: 'user_5' },
  { id: 'enr_6', courseId: 'course_2', courseTitle: 'Machine Learning Fundamentals', userId: 'user_6', userName: 'Alex Kumar', userEmail: 'alex@freelance.dev', userAvatar: '/avatars/alex.jpg', organizationId: 'org_1', role: 'student', status: 'dropped', source: 'self', enrolledAt: '2023-12-01T10:30:00Z', startedAt: '2023-12-01T10:30:00Z', completedAt: null, droppedAt: '2024-01-05T11:00:00Z', progress: 40, score: 65, grade: 'C', enrolledBy: 'user_6', dropReason: 'Time constraints' },
]

const enrollments = computed(() => {
  let result = props.enrollments || mockEnrollments
  
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(e => 
      e.userName.toLowerCase().includes(query) ||
      e.userEmail.toLowerCase().includes(query) ||
      e.courseTitle.toLowerCase().includes(query)
    )
  }
  
  if (statusFilter.value !== 'all') {
    result = result.filter(e => e.status === statusFilter.value)
  }
  
  if (roleFilter.value !== 'all') {
    result = result.filter(e => e.role === roleFilter.value)
  }
  
  if (courseFilter.value) {
    result = result.filter(e => e.courseTitle.toLowerCase().includes(courseFilter.value.toLowerCase()))
  }
  
  result.sort((a, b) => {
    let aVal = a[sortBy.value]
    let bVal = b[sortBy.value]
    if (typeof aVal === 'string') aVal = aVal.toLowerCase()
    if (typeof bVal === 'string') bVal = bVal.toLowerCase()
    return sortOrder.value === 'asc' ? (aVal > bVal ? 1 : -1) : (aVal < bVal ? 1 : -1)
  })
  
  return result
}

const paginatedEnrollments = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  return enrollments.value.slice(start, start + itemsPerPage)
}

const totalPages = computed(() => Math.ceil(enrollments.value.length / itemsPerPage))

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
    active: 'success',
    completed: 'success',
    dropped: 'destructive',
    suspended: 'destructive',
    pending: 'warning',
    waitlisted: 'warning',
  }
  return variants[status] || 'default'
}

const getRoleBadge = (role: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'destructive' | 'info' | 'secondary'> = {
    student: 'default',
    teaching_assistant: 'info',
    instructor: 'warning',
    course_designer: 'info',
    observer: 'secondary',
    auditor: 'secondary',
  }
  return variants[role] || 'default'
}

const getSourceBadge = (source: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'info' | 'secondary'> = {
    self: 'default',
    instructor: 'info',
    admin: 'warning',
    sis: 'secondary',
    lti: 'info',
    api: 'secondary',
    bulk: 'secondary',
  }
  return variants[source] || 'default'
}

const handleAction = (action: string, enrollment: any) => {
  props.onEnrollmentAction?.(action, enrollment)
}
</script>

<template>
  <Card class="h-full">
    <CardHeader class="pb-3">
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <GraduationCapIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Enrollment Management</CardTitle>
            <CardDescription>{{ enrollments.length }} total enrollments</CardDescription>
          </div>
        </div>
        
        <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
          <div class="relative flex-1 max-w-sm">
            <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              v-model="searchQuery"
              placeholder="Search enrollments..."
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
              <SelectItem value="active">Active</SelectItem>
              <SelectItem value="completed">Completed</SelectItem>
              <SelectItem value="dropped">Dropped</SelectItem>
              <SelectItem value="suspended">Suspended</SelectItem>
              <SelectItem value="pending">Pending</SelectItem>
              <SelectItem value="waitlisted">Waitlisted</SelectItem>
            </SelectContent>
          </Select>
          
          <Select v-model="roleFilter" class="w-36">
            <SelectTrigger>
              <SelectValue placeholder="Role" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Roles</SelectItem>
              <SelectItem value="student">Student</SelectItem>
              <SelectItem value="teaching_assistant">Teaching Assistant</SelectItem>
              <SelectItem value="instructor">Instructor</SelectItem>
              <SelectItem value="course_designer">Course Designer</SelectItem>
              <SelectItem value="observer">Observer</SelectItem>
              <SelectItem value="auditor">Auditor</SelectItem>
            </SelectContent>
          </Select>
          
          <Button @click="showBulkEnroll = true" class="gap-2">
            <UserPlusIcon class="h-4 w-4" />
            Bulk Enroll
          </Button>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <div class="overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead class="cursor-pointer" @click="handleSort('userName')">
                  <div class="flex items-center gap-1">
                    Student
                    <ChevronDownIcon v-if="sortBy === 'userName' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'userName' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden md:table-cell" @click="handleSort('courseTitle')">
                  <div class="flex items-center gap-1">
                    Course
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="handleSort('role')">
                  <div class="flex items-center gap-1">
                    Role
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="handleSort('status')">
                  <div class="flex items-center gap-1">
                    Status
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="handleSort('source')">
                  <div class="flex items-center gap-1">
                    Source
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden lg:table-cell" @click="handleSort('progress')">
                  <div class="flex items-center gap-1">
                    Progress
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden lg:table-cell" @click="handleSort('score')">
                  <div class="flex items-center gap-1">
                    Score
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="handleSort('enrolledAt')">
                  <div class="flex items-center gap-1">
                    Enrolled
                  </div>
                </TableHead>
                <TableHead class="w-56">Actions</TableHead>
              </TableRow>
            </TableHeader>
            
            <TableBody v-if="!loading">
              <TableRow v-for="enrollment in paginatedEnrollments" :key="enrollment.id" class="hover:bg-accent/50">
                <TableCell class="font-medium">
                  <div class="flex items-center gap-3">
                    <Avatar :size="sm" :src="enrollment.userAvatar" :fallback="enrollment.userName">
                      <template #icon>
                        <UserIcon class="h-3 w-3" />
                      </template>
                    </Avatar>
                    <div>
                      <p class="font-medium truncate max-w-[150px]">{{ enrollment.userName }}</p>
                      <p class="text-sm text-muted-foreground truncate max-w-[150px]">{{ enrollment.userEmail }}</p>
                    </div>
                  </div>
                </TableCell>
                <TableCell class="hidden md:table-cell">
                  <p class="font-medium truncate max-w-[200px]">{{ enrollment.courseTitle }}</p>
                </TableCell>
                <TableCell>
                  <Badge :variant="getRoleBadge(enrollment.role)">{{ enrollment.role.replace('_', ' ') }}</Badge>
                </TableCell>
                <TableCell>
                  <Badge :variant="getStatusBadge(enrollment.status)">{{ enrollment.status }}</Badge>
                </TableCell>
                <TableCell class="hidden md:table-cell">
                  <Badge :variant="getSourceBadge(enrollment.source)">{{ enrollment.source }}</Badge>
                </TableCell>
                <TableCell class="hidden lg:table-cell">
                  <div class="flex items-center gap-2">
                    <Progress :value="enrollment.progress" class="w-24 h-1.5" />
                    <span class="text-sm font-medium tabular-nums">{{ enrollment.progress }}%</span>
                  </div>
                </TableCell>
                <TableCell class="hidden lg:table-cell">
                  <span v-if="enrollment.score !== null" class="font-mono tabular-nums">{{ enrollment.score }}%</span>
                  <span v-else class="text-muted-foreground">—</span>
                </TableCell>
                <TableCell class="text-muted-foreground text-sm">
                  {{ formatRelativeTime(enrollment.enrolledAt) }}
                </TableCell>
                <TableCell>
                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Button variant="ghost" size="icon" class="h-8 w-8">
                        <MoreHorizontalIcon class="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem @click="handleAction('view', enrollment)">
                        <EyeIcon class="h-4 w-4" />
                        View Details
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('edit', enrollment)">
                        <EditIcon class="h-4 w-4" />
                        Edit Enrollment
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('progress', enrollment)">
                        <BarChartIcon class="h-4 w-4" />
                        View Progress
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem v-if="enrollment.status === 'active'" @click="handleAction('complete', enrollment)">
                        <CheckCircleIcon class="h-4 w-4" />
                        Mark Complete
                      </DropdownMenuItem>
                      <DropdownMenuItem v-if="enrollment.status === 'active'" @click="handleAction('suspend', enrollment)" class="text-warning">
                        <PauseIcon class="h-4 w-4" />
                        Suspend
                      </DropdownMenuItem>
                      <DropdownMenuItem v-if="enrollment.status !== 'dropped'" @click="handleAction('drop', enrollment)" class="text-destructive">
                        <UserXIcon class="h-4 w-4" />
                        Drop Student
                      </DropdownMenuItem>
                      <DropdownMenuItem v-if="enrollment.status === 'dropped'" @click="handleAction('reinstate', enrollment)" class="text-success">
                        <UserCheckIcon class="h-4 w-4" />
                        Reinstate
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem @click="handleAction('grade', enrollment)">
                        <EditIcon class="h-4 w-4" />
                        Manage Grades
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('message', enrollment)">
                        <MailIcon class="h-4 w-4" />
                        Message Student
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TableCell>
              </TableRow>
            </TableBody>
            
            <TableBody v-else>
              <TableRow v-for="i in 5" :key="i">
                <TableCell><Skeleton class="h-10 w-24" /></TableCell>
                <TableCell><Skeleton class="h-4 w-32" /></TableCell>
                <TableCell><Skeleton class="h-4 w-20" /></TableCell>
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
            Showing {{ (currentPage - 1) * itemsPerPage + 1 }} to {{ Math.min(currentPage * itemsPerPage, enrollments.length) }} of {{ enrollments.length }} enrollments
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
const UserIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z' })]) }})
const MailIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z' })]) }})
const UserXIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z' })]) }})
const PauseIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '6', y: '4', width: '4', height: '16' }), h('rect', { x: '14', y: '4', width: '4', height: '16' })]) }})
const BarChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 012-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' })]) }})
</script>