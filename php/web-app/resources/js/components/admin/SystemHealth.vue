<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Progress } from '@/components/ui/progress'
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from '@/components/ui/table'
import { 
  ServerIcon, DatabaseIcon, HardDriveIcon, WifiIcon,
  CpuIcon, MemoryStickIcon, ShieldIcon, CheckCircleIcon,
  AlertCircleIcon, AlertTriangleIcon, RefreshCwIcon,
  ChevronDownIcon, ChevronUpIcon, ActivityIcon, SettingsIcon
} from '@/components/ui/icons'
import { formatRelativeTime, formatBytes } from '@/utils/cn'

interface Props {
  health?: any
  loading?: boolean
  onRefresh?: () => void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

const lastRefresh = ref(new Date())
const autoRefresh = ref(false)
const refreshInterval = ref<number>()

const mockHealth = {
  overall: 'healthy',
  services: [
    { name: 'API Gateway', status: 'healthy', latency: 12, uptime: '99.99%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.2.3', requests: 125000 } },
    { name: 'Auth Service', status: 'healthy', latency: 8, uptime: '99.98%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.1.0', activeSessions: 12500 } },
    { name: 'Realtime Service', status: 'healthy', latency: 5, uptime: '99.95%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.0.5', connections: 8500 } },
    { name: 'Search Service', status: 'degraded', latency: 45, uptime: '99.90%', lastCheck: '2024-01-15T10:29:00Z', details: { version: '1.3.1', indexSize: '2.4GB', queue: 12 } },
    { name: 'Media Service', status: 'healthy', latency: 23, uptime: '99.97%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.2.0', storage: '45GB', processing: 3 } },
    { name: 'Scheduler Service', status: 'healthy', latency: 18, uptime: '99.99%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.1.2', jobsQueued: 45, jobsRunning: 2 } },
    { name: 'AI Service', status: 'healthy', latency: 120, uptime: '99.92%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.4.0', gpuUtilization: '65%', queue: 8 } },
    { name: 'Analytics Service', status: 'healthy', latency: 35, uptime: '99.94%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.3.0', eventsProcessed: '2.4M/hr' } },
    { name: 'Grading Service', status: 'healthy', latency: 85, uptime: '99.96%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.2.1', queue: 15 } },
    { name: 'Recommendation Service', status: 'healthy', latency: 42, uptime: '99.93%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '1.1.0', cacheHitRate: '94%' } },
    { name: 'PostgreSQL', status: 'healthy', latency: 3, uptime: '99.99%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '15.5', connections: 45, size: '120GB' } },
    { name: 'Redis', status: 'healthy', latency: 1, uptime: '99.99%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '7.2', memory: '8GB', hitRate: '98%' } },
    { name: 'NATS', status: 'healthy', latency: 2, uptime: '99.99%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '2.10', streams: 12, consumers: 45 } },
    { name: 'MinIO', status: 'healthy', latency: 8, uptime: '99.98%', lastCheck: '2024-01-15T10:30:00Z', details: { version: '2023-12-01', buckets: 8, objects: '2.4M' } },
  ],
  resources: {
    cpu: { usage: 34, limit: 80, trend: 'stable' },
    memory: { usage: 62, limit: 85, trend: 'up' },
    disk: { usage: 45, limit: 90, trend: 'stable' },
    network: { usage: 28, limit: 70, trend: 'down' },
    gpu: { usage: 65, limit: 90, trend: 'up' },
  },
  incidents: [
    { id: '1', title: 'Search Service High Latency', severity: 'warning', status: 'investigating', createdAt: '2024-01-15T10:25:00Z', affectedServices: ['Search Service'] },
    { id: '2', title: 'GPU Memory Pressure on AI Nodes', severity: 'warning', status: 'monitoring', createdAt: '2024-01-15T09:45:00Z', affectedServices: ['AI Service'] },
  ],
}

const health = computed(() => props.health || mockHealth)
const services = computed(() => health.value.services)
const resources = computed(() => health.value.resources)
const incidents = computed(() => health.value.incidents)

const healthyCount = computed(() => services.value.filter(s => s.status === 'healthy').length)
const degradedCount = computed(() => services.value.filter(s => s.status === 'degraded').length)
const downCount = computed(() => services.value.filter(s => s.status === 'down').length)

const overallStatus = computed(() => {
  if (downCount.value > 0) return 'down'
  if (degradedCount.value > 0) return 'degraded'
  return 'healthy'
})

const getStatusBadge = (status: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'destructive' | 'info' | 'secondary'> = {
    healthy: 'success',
    degraded: 'warning',
    down: 'destructive',
    unknown: 'secondary',
  }
  return variants[status] || 'default'
}

const getSeverityBadge = (severity: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'destructive' | 'info' | 'secondary'> = {
    critical: 'destructive',
    high: 'destructive',
    warning: 'warning',
    info: 'info',
    low: 'success',
  }
  return variants[severity] || 'default'
}

const refreshHealth = () => {
  props.onRefresh?.()
  lastRefresh.value = new Date()
}

const toggleAutoRefresh = () => {
  autoRefresh.value = !autoRefresh.value
  if (autoRefresh.value) {
    refreshInterval.value = window.setInterval(refreshHealth, 30000)
  } else {
    if (refreshInterval.value) {
      clearInterval(refreshInterval.value)
    }
  }
}

onMounted(() => {
  if (autoRefresh.value) {
    refreshInterval.value = window.setInterval(refreshHealth, 30000)
  }
})

onUnmounted(() => {
  if (refreshInterval.value) {
    clearInterval(refreshInterval.value)
  }
})
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
      <div class="flex items-center gap-3">
        <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
          <ActivityIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
        </div>
        <div>
          <h1 class="text-2xl font-bold">System Health</h1>
          <p class="text-muted-foreground">Monitor system performance and service status</p>
        </div>
        
        <div class="flex items-center gap-2">
          <Badge :variant="overallStatus === 'healthy' ? 'success' : overallStatus === 'degraded' ? 'warning' : 'destructive'" class="gap-2">
            <component :is="overallStatus === 'healthy' ? CheckCircleIcon : overallStatus === 'degraded' ? AlertTriangleIcon : AlertCircleIcon" class="h-3 w-3" />
            {{ overallStatus.value.toUpperCase() }}
          </Badge>
          
          <div class="flex items-center gap-2 ml-4">
            <label class="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" v-model="autoRefresh" class="h-4 w-4 rounded border-border" @change="toggleAutoRefresh" />
              <span class="text-sm text-muted-foreground">Auto-refresh (30s)</span>
            </label>
            <Button variant="outline" size="icon" @click="refreshHealth" :disabled="loading" class="gap-1">
              <RefreshCwIcon class="h-4 w-4" :class="{ 'animate-spin': loading }" />
            </Button>
          </div>
        </div>
      </div>

    <!-- Overview Stats -->
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-5">
      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-muted-foreground">Healthy Services</p>
              <p class="text-3xl font-display font-bold text-success-600">{{ healthyCount }}</p>
            </div>
            <div class="p-3 rounded-xl bg-success-100 dark:bg-success-900/30">
              <CheckCircleIcon class="h-6 w-6 text-success-600 dark:text-success-400" />
            </div>
          </div>
        </Card>
        
        <Card>
          <CardContent class="p-6">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm text-muted-foreground">Degraded</p>
                <p class="text-3xl font-display font-bold text-warning-600">{{ degradedCount }}</p>
              </div>
              <div class="p-3 rounded-xl bg-warning-100 dark:bg-warning-900/30">
                <AlertTriangleIcon class="h-6 w-6 text-warning-600 dark:text-warning-400" />
              </div>
            </div>
          </Card>
          
          <Card>
            <CardContent class="p-6">
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-sm text-muted-foreground">Down</p>
                  <p class="text-3xl font-display font-bold text-destructive-600">{{ downCount }}</p>
                </div>
                <div class="p-3 rounded-xl bg-destructive-100 dark:bg-destructive-900/30">
                  <AlertCircleIcon class="h-6 w-6 text-destructive-600 dark:text-destructive-400" />
                </div>
              </div>
            </Card>
            
            <Card>
              <CardContent class="p-6">
                <div class="flex items-center justify-between">
                  <div>
                    <p class="text-sm text-muted-foreground">Total Services</p>
                    <p class="text-3xl font-display font-bold">{{ services.length }}</p>
                  </div>
                  <div class="p-3 rounded-xl bg-primary-100 dark:bg-primary-900/30">
                    <ServerIcon class="h-6 w-6 text-primary-600 dark:text-primary-400" />
                  </div>
                </div>
              </Card>
              
              <Card>
                <CardContent class="p-6">
                  <div class="flex items-center justify-between">
                    <div>
                      <p class="text-sm text-muted-foreground">Open Incidents</p>
                      <p class="text-3xl font-display font-bold text-warning-600">{{ incidents.length }}</p>
                    </div>
                    <div class="p-3 rounded-xl bg-warning-100 dark:bg-warning-900/30">
                      <AlertTriangleIcon class="h-6 w-6 text-warning-600 dark:text-warning-400" />
                    </div>
                  </div>
                </Card>
              </div>
            </div>
          </Card>
        </div>

    <!-- Resources -->
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <Card v-for="(resource, key) in resources" :key="key" class="relative overflow-hidden">
        <div class="absolute inset-0 bg-gradient-to-br from-[{{ resource.usage > resource.limit * 0.8 ? 'destructive' : resource.usage > resource.limit * 0.6 ? 'warning' : 'primary' }}]-500/10 to-transparent" />
        <CardContent class="relative p-6">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-3">
              <div :class="cn('p-3 rounded-xl', resource.usage > resource.limit * 0.8 ? 'bg-destructive-100 dark:bg-destructive-900/30' : resource.usage > resource.limit * 0.6 ? 'bg-warning-100 dark:bg-warning-900/30' : 'bg-primary-100 dark:bg-primary-900/30')">
                <component :is="resourceIcons[key]" class="h-6 w-6" :class="cn(
                  resource.usage > resource.limit * 0.8 ? 'text-destructive-600 dark:text-destructive-400' :
                  resource.usage > resource.limit * 0.6 ? 'text-warning-600 dark:text-warning-400' :
                  'text-primary-600 dark:text-primary-400'
                )" />
              </div>
              <div>
                <p class="text-sm font-medium capitalize">{{ key }}</p>
                <p class="text-xs text-muted-foreground">{{ resource.trend === 'up' ? 'Increasing' : resource.trend === 'down' ? 'Decreasing' : 'Stable' }}</p>
              </div>
            </div>
            <Badge :variant="resource.usage > resource.limit * 0.9 ? 'destructive' : resource.usage > resource.limit * 0.75 ? 'warning' : 'success'">
              {{ resource.usage }}%
            </Badge>
          </div>
          
          <Progress :value="resource.usage" class="h-2 mb-2" :class="cn(
            resource.usage > resource.limit * 0.9 ? 'bg-destructive-500' :
            resource.usage > resource.limit * 0.75 ? 'bg-warning-500' :
            'bg-primary-500'
          )" />
          
          <div class="flex items-center justify-between text-xs text-muted-foreground">
            <span>{{ resource.usage }}% used</span>
            <span>Limit: {{ resource.limit }}%</span>
          </div>
          
          <div class="mt-4 flex items-center gap-2 text-xs text-muted-foreground">
            <TrendingUpIcon v-if="resource.trend === 'up'" class="h-3 w-3 text-success-500" />
            <TrendingUpIcon v-else-if="resource.trend === 'down'" class="h-3 w-3 rotate-180 text-destructive-500" />
            <MinusIcon v-else class="h-3 w-3" />
            <span>{{ resource.trend === 'up' ? 'Increasing' : resource.trend === 'down' ? 'Decreasing' : 'Stable' }}</span>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Services -->
    <Card class="overflow-hidden">
      <CardHeader class="pb-3">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
              <ServerIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
            </div>
            <div>
              <CardTitle class="text-lg">Service Status</CardTitle>
              <CardDescription>{{ healthyCount }} healthy · {{ degradedCount }} degraded · {{ downCount }} down</CardDescription>
            </div>
          </div>
        </CardHeader>
        
        <CardContent class="pt-0">
          <div class="overflow-x-auto">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Service</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead>Latency</TableHead>
                  <TableHead>Uptime</TableHead>
                  <TableHead class="hidden md:table-cell">Last Check</TableHead>
                  <TableHead class="hidden lg:table-cell">Details</TableHead>
                  <TableHead>Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                <TableRow v-for="service in services" :key="service.name" class="hover:bg-accent/50">
                  <TableCell class="font-medium flex items-center gap-2">
                    <div :class="cn('w-2 h-2 rounded-full', service.status === 'healthy' ? 'bg-success-500' : service.status === 'degraded' ? 'bg-warning-500' : 'bg-destructive-500')" />
                    <span class="font-medium">{{ service.name }}</span>
                  </TableCell>
                  <TableCell>
                    <Badge :variant="getStatusBadge(service.status)">{{ service.status }}</Badge>
                  </TableCell>
                  <TableCell>
                    <span class="font-mono tabular-nums">{{ service.latency }}ms</span>
                  </TableCell>
                  <TableCell>{{ service.uptime }}</TableCell>
                  <TableCell class="hidden md:table-cell text-sm text-muted-foreground">{{ formatRelativeTime(service.lastCheck) }}</TableCell>
                  <TableCell class="hidden lg:table-cell text-sm text-muted-foreground">
                    <div v-for="(value, key) in service.details" :key="key" class="text-xs">
                      <span class="text-muted-foreground">{{ key }}: </span>{{ value }}
                    </div>
                  </TableCell>
                  <TableCell>
                    <DropdownMenu>
                      <DropdownMenuTrigger as-child>
                        <Button variant="ghost" size="icon" class="h-8 w-8">
                          <MoreHorizontalIcon class="h-4 w-4" />
                        </Button>
                      </DropdownMenuTrigger>
                      <DropdownMenuContent align="end">
                        <DropdownMenuItem @click="viewServiceLogs(service.name)">
                          <FileTextIcon class="h-4 w-4" />
                          View Logs
                        </DropdownMenuItem>
                        <DropdownMenuItem @click="viewServiceMetrics(service.name)">
                          <BarChartIcon class="h-4 w-4" />
                          View Metrics
                        </DropdownMenuItem>
                        <DropdownMenuItem v-if="service.status !== 'healthy'" class="text-warning">
                          <AlertTriangleIcon class="h-4 w-4" />
                          Investigate
                        </DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem @click="restartService(service.name)" class="text-destructive">
                          <RefreshCwIcon class="h-4 w-4" />
                          Restart Service
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </div>
        </CardContent>
      </Card>

    <!-- Incidents -->
    <Card v-if="incidents.length > 0">
      <CardHeader class="pb-3">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 rounded-lg bg-warning-100 dark:bg-warning-900/30">
              <AlertTriangleIcon class="h-5 w-5 text-warning-600 dark:text-warning-400" />
            </div>
            <div>
              <CardTitle class="text-lg">Open Incidents</CardTitle>
              <CardDescription>{{ incidents.length }} active incidents requiring attention</CardDescription>
            </div>
          </div>
        </CardHeader>
        
        <CardContent class="pt-0">
          <div class="space-y-3">
            <div v-for="incident in incidents" :key="incident.id" class="border border-border rounded-lg p-4">
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center gap-3 mb-2">
                    <h4 class="font-semibold">{{ incident.title }}</h4>
                    <Badge :variant="getSeverityBadge(incident.severity)">{{ incident.severity }}</Badge>
                    <Badge :variant="incident.status === 'resolved' ? 'success' : incident.status === 'investigating' ? 'info' : 'warning'">{{ incident.status }}</Badge>
                  </div>
                  <p class="text-sm text-muted-foreground mb-2">Affects: {{ incident.affectedServices.join(', ') }}</p>
                  <p class="text-xs text-muted-foreground">Detected: {{ formatRelativeTime(incident.createdAt) }}</p>
                </div>
                <div class="flex gap-2">
                  <Button variant="outline" size="sm">Investigate</Button>
                  <Button variant="default" size="sm">Resolve</Button>
                </div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </Card>
  </div>
</template>

<script>
const ServerIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '2', y: '2', width: '20', height: '20', rx: '2', ry: '2', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M6 10h12M6 14h12M6 18h12' })]) }})
const DatabaseIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('ellipse', { cx: '12', cy: '5', rx: '9', ry: '3', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 5v14a2 2 0 002 2h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v9' })]) }})
const HardDriveIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17 14V3a2 2 0 00-2-2H9a2 2 0 00-2 2v11' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 14h-6a2 2 0 00-2 2v4a2 2 0 002 2h6a2 2 0 002-2v-4' })]) }})
const WifiIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 13l6-6M12 4V1M5 19h14' })]) }})
const CpuIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '4', y: '4', width: '16', height: '16', rx: '2', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 8h6M9 12h6M9 16h6' })]) }})
const MemoryStickIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M16 3H5a1 1 0 00-1 1v16a1 1 0 001 1h14a1 1 0 001-1V8a1 1 0 00-1-1h-5' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 17h8M8 13h8M8 9h8' })]) }})
const MinusIcon = defineComponent({ render() { return h('svg', { class: 'h-3 w-3', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M20 12H4' })]) }})
const ActivityIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 2L3 14h9l-1 8 10-12h-9l11-4 10-12H6l-1 8 10-12' })]) }})
const SettingsIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
</script>