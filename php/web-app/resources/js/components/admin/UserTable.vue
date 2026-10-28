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
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableCaption } from '@/components/ui/table'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'
import { Pagination } from '@/components/ui/pagination'
import { Skeleton } from '@/components/ui/skeleton'
import { 
  SearchIcon, FilterIcon, MoreHorizontalIcon, UserPlusIcon,
  MailIcon, ShieldIcon, UserCheckIcon, UserXIcon, SettingsIcon,
  ChevronDownIcon, ChevronUpIcon, ArrowRightIcon
} from '@/components/ui/icons'
import { formatRelativeTime, formatDate } from '@/utils/cn'
import { useToast } from '@/composables/useToast'

interface Props {
  users?: any[]
  loading?: boolean
  onUserAction?: (action: string, user: any) => void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

const toast = useToast()

const searchQuery = ref('')
const statusFilter = ref('all')
const roleFilter = ref('all')
const sortBy = ref('created_at')
const sortOrder = ref<'asc' | 'desc'>('desc')
const currentPage = ref(1)
const itemsPerPage = 20

const mockUsers = [
  { id: '1', name: 'Sarah Chen', email: 'sarah@techcorp.com', avatar: '/avatars/sarah.jpg', role: 'admin', status: 'active', lastLogin: '2024-01-15T10:30:00Z', createdAt: '2023-06-15', enrollments: 12, coursesCompleted: 8 },
  { id: '2', name: 'Marcus Johnson', email: 'marcus@startup.io', avatar: '/avatars/marcus.jpg', role: 'instructor', status: 'active', lastLogin: '2024-01-14T16:45:00Z', createdAt: '2023-08-22', enrollments: 3, coursesCompleted: 2 },
  { id: '3', name: 'Emily Rodriguez', email: 'emily@university.edu', avatar: '/avatars/emily.jpg', role: 'student', status: 'active', lastLogin: '2024-01-15T09:15:00Z', createdAt: '2023-11-10', enrollments: 5, coursesCompleted: 3 },
  { id: '4', name: 'David Park', email: 'david@enterprise.com', avatar: '/avatars/david.jpg', role: 'student', status: 'pending', lastLogin: '2024-01-10T14:20:00Z', createdAt: '2024-01-05', enrollments: 0, coursesCompleted: 0 },
  { id: '5', name: 'Lisa Wong', email: 'lisa@corp.com', avatar: '/avatars/lisa.jpg', role: 'org_admin', status: 'active', lastLogin: '2024-01-15T11:00:00Z', createdAt: '2023-09-01', enrollments: 8, coursesCompleted: 6 },
  { id: '6', name: 'Alex Kumar', email: 'alex@freelance.dev', avatar: '/avatars/alex.jpg', role: 'student', status: 'suspended', lastLogin: '2023-12-20T08:30:00Z', createdAt: '2023-07-15', enrollments: 12, coursesCompleted: 9 },
]

const users = computed(() => {
  let result = props.users || mockUsers
  
  // Search
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(u => 
      u.name.toLowerCase().includes(query) ||
      u.email.toLowerCase().includes(query)
    )
  }
  
  // Status filter
  if (statusFilter.value !== 'all') {
    result = result.filter(u => u.status === statusFilter.value)
  }
  
  // Role filter
  if (roleFilter.value !== 'all') {
    result = result.filter(u => u.role === roleFilter.value)
  }
  
  // Sort
  result.sort((a, b) => {
    let aVal = a[sortBy.value]
    let bVal = b[sortBy.value]
    if (typeof aVal === 'string') aVal = aVal.toLowerCase()
    if (typeof bVal === 'string') bVal = bVal.toLowerCase()
    return sortOrder.value === 'asc' 
      ? (aVal > bVal ? 1 : -1) 
      : (aVal < bVal ? 1 : -1)
  })
  
  return result
})

const paginatedUsers = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  return users.value.slice(start, start + itemsPerPage)
})

const totalPages = computed(() => Math.ceil(users.value.length / itemsPerPage))

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
    pending: 'warning',
    suspended: 'destructive',
    inactive: 'secondary',
  }
  return variants[status] || 'default'
}

const getRoleBadge = (role: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'destructive' | 'info' | 'secondary'> = {
    admin: 'destructive',
    super_admin: 'destructive',
    org_admin: 'warning',
    instructor: 'info',
    student: 'default',
    ta: 'info',
  }
  return variants[role] || 'default'
}

const handleAction = (action: string, user: any) => {
  props.onUserAction?.(action, user)
  toast.success(`${action.charAt(0).toUpperCase() + action.slice(1)} action initiated for ${user.name}`)
}
</script>

<template>
  <Card>
    <CardHeader class="pb-3">
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <UsersIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">User Management</CardTitle>
            <CardDescription>{{ users.value.length }} total users</CardDescription>
          </div>
        </div>
        
        <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
          <div class="relative flex-1 max-w-sm">
            <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              v-model="searchQuery"
              placeholder="Search users..."
              class="pl-10"
              @input="currentPage = 1"
            />
          </div>
          
          <Select v-model="statusFilter" class="w-36">
            <SelectTrigger>
              <SelectValue placeholder="Status" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Statuses</SelectItem>
              <SelectItem value="active">Active</SelectItem>
              <SelectItem value="pending">Pending</SelectItem>
              <SelectItem value="suspended">Suspended</SelectItem>
              <SelectItem value="inactive">Inactive</SelectItem>
            </SelectContent>
          </Select>
          
          <Select v-model="roleFilter" class="w-36">
            <SelectTrigger>
              <SelectValue placeholder="Role" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Roles</SelectItem>
              <SelectItem value="super_admin">Super Admin</SelectItem>
              <SelectItem value="admin">Admin</SelectItem>
              <SelectItem value="org_admin">Org Admin</SelectItem>
              <SelectItem value="instructor">Instructor</SelectItem>
              <SelectItem value="ta">Teaching Assistant</SelectItem>
              <SelectItem value="student">Student</SelectItem>
            </SelectContent>
          </Select>
          
          <Button @click="$emit('create-user')" class="gap-2">
            <UserPlusIcon class="h-4 w-4" />
            Add User
          </Button>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <div class="overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead class="cursor-pointer" @click="handleSort('name')">
                  <div class="flex items-center gap-1">
                    User
                    <ChevronDownIcon v-if="sortBy === 'name' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'name' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden md:table-cell" @click="handleSort('email')">
                  <div class="flex items-center gap-1">
                    Email
                    <ChevronDownIcon v-if="sortBy === 'email' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'email' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden lg:table-cell" @click="handleSort('role')">
                  <div class="flex items-center gap-1">
                    Role
                    <ChevronDownIcon v-if="sortBy === 'role' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'role' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="handleSort('status')">
                  <div class="flex items-center gap-1">
                    Status
                    <ChevronDownIcon v-if="sortBy === 'status' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'status' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden md:table-cell" @click="handleSort('lastLogin')">
                  <div class="flex items-center gap-1">
                    Last Login
                    <ChevronDownIcon v-if="sortBy === 'lastLogin' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'lastLogin' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="handleSort('createdAt')">
                  <div class="flex items-center gap-1">
                    Joined
                    <ChevronDownIcon v-if="sortBy === 'createdAt' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'createdAt' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="w-48">Actions</TableHead>
              </TableRow>
            </TableHeader>
            
            <TableBody v-if="!loading">
              <TableRow v-for="user in paginatedUsers" :key="user.id" class="hover:bg-accent/50">
                <TableCell class="font-medium">
                  <div class="flex items-center gap-3">
                    <Avatar :size="sm" :src="user.avatar" :fallback="user.name" />
                    <div>
                      <p class="font-medium">{{ user.name }}</p>
                      <p class="text-sm text-muted-foreground truncate max-w-[200px]">{{ user.email }}</p>
                    </div>
                  </div>
                </TableCell>
                <TableCell class="hidden md:table-cell text-muted-foreground">{{ user.email }}</TableCell>
                <TableCell class="hidden lg:table-cell">
                  <Badge :variant="getRoleBadge(user.role)">{{ user.role.replace('_', ' ') }}</Badge>
                </TableCell>
                <TableCell>
                  <Badge :variant="getStatusBadge(user.status)">{{ user.status }}</Badge>
                </TableCell>
                <TableCell class="hidden md:table-cell text-muted-foreground">
                  {{ formatRelativeTime(user.lastLogin) }}
                </TableCell>
                <TableCell class="text-muted-foreground">
                  {{ formatDate(user.createdAt) }}
                </TableCell>
                <TableCell>
                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Button variant="ghost" size="icon" class="h-8 w-8">
                        <MoreHorizontalIcon class="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem @click="handleAction('view', user)">
                        <UserCheckIcon class="h-4 w-4" />
                        View Profile
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('edit', user)">
                        <SettingsIcon class="h-4 w-4" />
                        Edit
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem @click="handleAction('impersonate', user)">
                        <ShieldIcon class="h-4 w-4" />
                        Impersonate
                      </DropdownMenuItem>
                      <DropdownMenuItem v-if="user.status !== 'suspended'" @click="handleAction('suspend', user)" class="text-destructive">
                        <UserXIcon class="h-4 w-4" />
                        Suspend
                      </DropdownMenuItem>
                      <DropdownMenuItem v-if="user.status === 'suspended'" @click="handleAction('activate', user)" class="text-success">
                        <UserCheckIcon class="h-4 w-4" />
                        Activate
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem @click="handleAction('delete', user)" class="text-destructive">
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
                <TableCell><Skeleton class="h-4 w-40" /></TableCell>
                <TableCell><Skeleton class="h-4 w-24" /></TableCell>
                <TableCell><Skeleton class="h-4 w-20" /></TableCell>
                <TableCell><Skeleton class="h-4 w-24" /></TableCell>
                <TableCell><Skeleton class="h-4 w-24" /></TableCell>
                <TableCell><Skeleton class="h-4 w-24" /></TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
        
        <!-- Pagination -->
        <div v-if="totalPages > 1" class="flex items-center justify-between mt-6">
          <div class="text-sm text-muted-foreground">
            Showing {{ (currentPage - 1) * itemsPerPage + 1 }} to {{ Math.min(currentPage * itemsPerPage, users.value.length) }} of {{ users.value.length }} users
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
const UsersIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })]) }})
const UserCheckIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z' })]) }})
const SettingsIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const ShieldIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
const UserXIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z' })]) }})
const TrashIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v10m4-10h4m-4 0v10M4 7h16' })]) }})
</script>