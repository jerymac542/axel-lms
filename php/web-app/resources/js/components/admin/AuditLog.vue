<script setup lang="ts">
import { ref, computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from '@/components/ui/table'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'
import { Pagination } from '@/components/ui/pagination'
import { 
  SearchIcon, FilterIcon, DownloadIcon, RefreshCwIcon,
  CalendarIcon, UserIcon, AlertCircleIcon, CheckCircleIcon,
  EyeIcon, ChevronDownIcon, ChevronUpIcon, ShieldIcon
} from '@/components/ui/icons'
import { formatRelativeTime, formatDate } from '@/utils/cn'

interface Props {
  logs?: any[]
  loading?: boolean
  onExport?: (format: string) => void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

const searchQuery = ref('')
const eventTypeFilter = ref('all')
const severityFilter = ref('all')
const userFilter = ref('')
const dateRange = ref({ start: null, end: null })
const sortBy = ref('timestamp')
const sortOrder = ref<'asc' | 'desc'>('desc')
const currentPage = ref(1)
const itemsPerPage = 50
const selectedLog = ref<any>(null)
const showDetail = ref(false)

const mockLogs = [
  { id: '1', timestamp: '2024-01-15T10:30:00Z', eventType: 'authentication', severity: 'info', userId: 'user_1', userName: 'Sarah Chen', ip: '192.168.1.100', userAgent: 'Chrome 120.0', action: 'login_success', resource: 'auth', details: { method: 'password', mfa: true }, tenantId: 'tenant_1' },
  { id: '2', timestamp: '2024-01-15T10:31:00Z', eventType: 'authorization', severity: 'warning', userId: 'user_2', userName: 'Marcus Johnson', ip: '192.168.1.101', userAgent: 'Firefox 121.0', action: 'access_denied', resource: 'admin/users', details: { reason: 'insufficient_permissions' }, tenantId: 'tenant_1' },
  { id: '3', timestamp: '2024-01-15T10:32:00Z', eventType: 'data_modification', severity: 'info', userId: 'user_1', userName: 'Sarah Chen', ip: '192.168.1.100', userAgent: 'Chrome 120.0', action: 'course_published', resource: 'course', resourceId: 'course_123', details: { courseTitle: 'Advanced React Patterns' }, tenantId: 'tenant_1' },
  { id: '4', timestamp: '2024-01-15T10:33:00Z', eventType: 'security', severity: 'critical', userId: 'user_3', userName: 'Unknown', ip: '45.77.123.45', userAgent: 'curl/7.68.0', action: 'brute_force_attempt', resource: 'auth/login', details: { attempts: 15, blocked: true }, tenantId: 'tenant_1' },
  { id: '5', timestamp: '2024-01-15T10:34:00Z', eventType: 'data_access', severity: 'info', userId: 'user_4', userName: 'David Park', ip: '192.168.1.102', userAgent: 'Safari 17.0', action: 'grade_viewed', resource: 'grade', resourceId: 'grade_456', details: { courseId: 'course_789' }, tenantId: 'tenant_1' },
  { id: '6', timestamp: '2024-01-15T10:35:00Z', eventType: 'configuration_change', severity: 'warning', userId: 'user_5', userName: 'Lisa Wong', ip: '192.168.1.103', userAgent: 'Chrome 120.0', action: 'role_assigned', resource: 'role', resourceId: 'role_instructor', details: { targetUserId: 'user_6', roleName: 'instructor' }, tenantId: 'tenant_1' },
  { id: '7', timestamp: '2024-01-15T10:36:00Z', eventType: 'data_modification', severity: 'info', userId: 'user_1', userName: 'Sarah Chen', ip: '192.168.1.100', userAgent: 'Chrome 120.0', action: 'enrollment_created', resource: 'enrollment', resourceId: 'enr_789', details: { courseId: 'course_123', studentId: 'user_7' }, tenantId: 'tenant_1' },
  { id: '8', timestamp: '2024-01-15T10:37:00Z', eventType: 'authentication', severity: 'info', userId: 'user_6', userName: 'Alex Kumar', ip: '192.168.1.104', userAgent: 'Edge 120.0', action: 'mfa_enabled', resource: 'auth', details: { method: 'totp' }, tenantId: 'tenant_1' },
  { id: '9', timestamp: '2024-01-15T10:38:00Z', eventType: 'security', severity: 'high', userId: 'user_7', userName: 'Bot/Script', ip: '185.220.101.1', userAgent: 'python-requests/2.28', action: 'scraping_attempt', resource: 'course/content', details: { blocked: true, rateLimited: true }, tenantId: 'tenant_1' },
  { id: '10', timestamp: '2024-01-15T10:39:00Z', eventType: 'data_export', severity: 'info', userId: 'user_5', userName: 'Lisa Wong', ip: '192.168.1.103', userAgent: 'Chrome 120.0', action: 'analytics_exported', resource: 'analytics', details: { format: 'csv', dateRange: '30d' }, tenantId: 'tenant_1' },
]

const logs = computed(() => {
  let result = props.logs || mockLogs
  
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(l => 
      l.userName.toLowerCase().includes(query) ||
      l.action.toLowerCase().includes(query) ||
      l.resource.toLowerCase().includes(query) ||
      l.ip.includes(query)
    )
  }
  
  if (eventTypeFilter.value !== 'all') {
    result = result.filter(l => l.eventType === eventTypeFilter.value)
  }
  
  if (severityFilter.value !== 'all') {
    result = result.filter(l => l.severity === severityFilter.value)
  }
  
  if (userFilter.value) {
    result = result.filter(l => l.userName.toLowerCase().includes(userFilter.value.toLowerCase()))
  }
  
  if (dateRange.value.start) {
    result = result.filter(l => new Date(l.timestamp) >= new Date(dateRange.value.start))
  }
  if (dateRange.value.end) {
    result = result.filter(l => new Date(l.timestamp) <= new Date(dateRange.value.end))
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

const paginatedLogs = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  return logs.value.slice(start, start + itemsPerPage)
}

const totalPages = computed(() => Math.ceil(logs.value.length / itemsPerPage))

const handleSort = (column: string) => {
  if (sortBy.value === column) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortBy.value = column
    sortOrder.value = 'asc'
  }
}

const getSeverityBadge = (severity: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'destructive' | 'info' | 'secondary'> = {
    info: 'info',
    low: 'success',
    medium: 'warning',
    high: 'destructive',
    critical: 'destructive',
  }
  return variants[severity] || 'default'
}

const getEventTypeIcon = (type: string) => {
  const icons: Record<string, any> = {
    authentication: 'LockIcon',
    authorization: 'ShieldIcon',
    data_access: 'EyeIcon',
    data_modification: 'EditIcon',
    data_export: 'DownloadIcon',
    security: 'AlertCircleIcon',
    configuration_change: 'SettingsIcon',
  }
  return icons[type] || 'FileTextIcon'
}

const openDetail = (log: any) => {
  selectedLog.value = log
  showDetail.value = true
}
</script>

<template>
  <Card class="h-full">
    <CardHeader class="pb-3">
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <ShieldIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Audit Log</CardTitle>
            <CardDescription>Track all system activities and security events</CardDescription>
          </div>
        </div>
        
        <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
          <div class="relative flex-1 max-w-sm">
            <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              v-model="searchQuery"
              placeholder="Search logs..."
              class="pl-10"
              @input="currentPage = 1"
            />
          </div>
          
          <Select v-model="eventTypeFilter" class="w-36">
            <SelectTrigger>
              <SelectValue placeholder="Event Type" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Types</SelectItem>
              <SelectItem value="authentication">Authentication</SelectItem>
              <SelectItem value="authorization">Authorization</SelectItem>
              <SelectItem value="data_access">Data Access</SelectItem>
              <SelectItem value="data_modification">Data Modification</SelectItem>
              <SelectItem value="data_export">Data Export</SelectItem>
              <SelectItem value="security">Security</SelectItem>
              <SelectItem value="configuration_change">Config Change</SelectItem>
            </SelectContent>
          </Select>
          
          <Select v-model="severityFilter" class="w-32">
            <SelectTrigger>
              <SelectValue placeholder="Severity" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All</SelectItem>
              <SelectItem value="info">Info</SelectItem>
              <SelectItem value="low">Low</SelectItem>
              <SelectItem value="medium">Medium</SelectItem>
              <SelectItem value="high">High</SelectItem>
              <SelectItem value="critical">Critical</SelectItem>
            </SelectContent>
          </Select>
          
          <div class="flex items-center gap-2">
            <Button variant="outline" size="icon" @click="refreshLogs" :disabled="loading" class="gap-1">
              <RefreshCwIcon class="h-4 w-4" :class="{ 'animate-spin': loading }" />
            </Button>
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button variant="outline" size="icon" class="gap-1">
                  <DownloadIcon class="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem @click="exportLogs('csv')">
                  <FileSpreadsheetIcon class="h-4 w-4" />
                  Export CSV
                </DropdownMenuItem>
                <DropdownMenuItem @click="exportLogs('json')">
                  <FileJsonIcon class="h-4 w-4" />
                  Export JSON
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <div class="overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead class="cursor-pointer" @click="sortBy = 'timestamp'; sortOrder = sortBy === 'timestamp' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    Timestamp
                    <ChevronDownIcon v-if="sortBy === 'timestamp' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'timestamp' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="sortBy = 'eventType'; sortOrder = sortBy === 'eventType' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    Event Type
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden md:table-cell" @click="sortBy = 'severity'; sortOrder = sortBy === 'severity' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    Severity
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="sortBy = 'userName'; sortOrder = sortBy === 'userName' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    User
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="sortBy = 'action'; sortOrder = sortBy === 'action' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    Action
                  </div>
                </TableHead>
                <TableHead class="hidden lg:table-cell">Resource</TableHead>
                <TableHead class="hidden lg:table-cell">IP Address</TableHead>
                <TableHead class="w-24">Actions</TableHead>
              </TableRow>
            </TableHeader>
            
            <TableBody>
              <TableRow v-for="log in paginatedLogs" :key="log.id" class="hover:bg-accent/50" @click="openDetail(log)">
                <TableCell class="font-mono text-sm">
                  {{ formatRelativeTime(log.timestamp) }}
                </TableCell>
                <TableCell>
                  <Badge :variant="getEventTypeBadge(log.eventType)" class="gap-1">
                    <component :is="getEventTypeIcon(log.eventType)" class="h-3 w-3" />
                    {{ log.eventType.replace('_', ' ') }}
                  </Badge>
                </TableCell>
                <TableCell class="hidden md:table-cell">
                  <Badge :variant="getSeverityBadge(log.severity)">{{ log.severity }}</Badge>
                </TableCell>
                <TableCell>
                  <div class="flex items-center gap-2">
                    <Avatar :size="sm" :fallback="log.userName">
                      <template #icon>
                        <UserIcon class="h-3 w-3" />
                      </template>
                    </Avatar>
                    <span class="font-medium">{{ log.userName }}</span>
                  </div>
                </TableCell>
                <TableCell class="font-mono text-sm">{{ log.action }}</TableCell>
                <TableCell class="hidden lg:table-cell text-sm text-muted-foreground">
                  {{ log.resource }}{{ log.resourceId ? ` / ${log.resourceId}` : '' }}
                </TableCell>
                <TableCell class="hidden lg:table-cell font-mono text-sm">{{ log.ip }}</TableCell>
                <TableCell>
                  <Button variant="ghost" size="icon" @click.stop="openDetail(log)" class="h-8 w-8">
                    <EyeIcon class="h-4 w-4" />
                  </Button>
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
        
        <div v-if="totalPages > 1" class="flex items-center justify-between mt-6">
          <div class="text-sm text-muted-foreground">
            Showing {{ (currentPage - 1) * itemsPerPage + 1 }} to {{ Math.min(currentPage * itemsPerPage, logs.length) }} of {{ logs.length }} events
          </div>
          <Pagination
            :current-page="currentPage"
            :total-pages="totalPages"
            @page-change="currentPage = $event"
          />
        </div>
      </CardContent>
    </Card>

    <!-- Detail Dialog -->
    <Dialog v-model:open="showDetail">
      <DialogContent class="max-w-4xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Audit Log Detail</DialogTitle>
          <DialogDescription>Event ID: {{ selectedLog?.id }}</DialogDescription>
        </DialogHeader>
        
        <div class="space-y-6 py-4">
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <DetailItem label="Event ID" :value="selectedLog?.id" copyable />
            <DetailItem label="Timestamp" :value="formatDate(selectedLog?.timestamp)" />
            <DetailItem label="Event Type" :value="selectedLog?.eventType" badge :badgeVariant="getEventTypeBadge(selectedLog?.eventType)" />
            <DetailItem label="Severity" :value="selectedLog?.severity" badge :badgeVariant="getSeverityBadge(selectedLog?.severity)" />
          </div>
          
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <DetailItem label="User" :value="selectedLog?.userName" />
            <DetailItem label="User ID" :value="selectedLog?.userId" copyable />
            <DetailItem label="Action" :value="selectedLog?.action" />
            <DetailItem label="Resource" :value="`${selectedLog?.resource}${selectedLog.resourceId ? ` / ${selectedLog.resourceId}` : ''}`" />
          </div>
          
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <DetailItem label="IP Address" :value="selectedLog?.ip" copyable />
            <DetailItem label="Tenant" :value="selectedLog?.tenantId" />
            <DetailItem label="User Agent" :value="selectedLog?.userAgent" />
          </div>
          
          <div class="border-t border-border pt-6">
            <h4 class="font-medium mb-3">Details</h4>
            <pre class="bg-muted p-4 rounded-lg overflow-x-auto text-sm"><code>{{ JSON.stringify(selectedLog?.details || {}, null, 2) }}</code></pre>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script>
const DetailItem = defineComponent({
  props: ['label', 'value', 'badge', 'badgeVariant', 'copyable'],
  setup(props) {
    return () => h('div', { class: 'space-y-1' }, [
      h('p', { class: 'text-xs font-medium text-muted-foreground' }, props.label),
      h('div', { class: 'flex items-center gap-2' }, [
        h('span', { class: 'font-mono text-sm break-all', style: { wordBreak: 'break-all' } }, props.value || '—'),
        props.badge && h('Badge', { variant: props.badgeVariant }, props.value),
        props.copyable && h('Button', { variant: 'ghost', size: 'icon', class: 'ml-auto h-6 w-6', onClick: () => navigator.clipboard.writeText(props.value) }, [
          h('component', { is: 'CopyIcon', class: 'h-3 w-3' })
        ]),
      ]),
    ])
  }
})

const FileSpreadsheetIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z' })]) }})
const FileJsonIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 17H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2.5M8 17v-2a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V17' })]) }})
const FileTextIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })]) }})
const CopyIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 012-2h10a2 2 0 012 2v12a2 2 0 01-2 2h-2' })]) }})
</script>