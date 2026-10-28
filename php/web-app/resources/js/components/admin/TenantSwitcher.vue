<script setup lang="ts">
import { ref, computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from '@/components/ui/table'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'
import { 
  SearchIcon, FilterIcon, PlusIcon, BuildingIcon, GlobeIcon,
  SettingsIcon, UsersIcon, ShieldIcon, EyeIcon, EditIcon,
  TrashIcon, CopyIcon, DownloadIcon, UploadIcon
} from '@/components/ui/icons'
import { formatRelativeTime } from '@/utils/cn'
import { useToast } from '@/composables/useToast'

interface Props {
  currentTenantId?: string
  onSwitchTenant?: (tenantId: string) => void
}

const props = withDefaults(defineProps<Props>(), {})
const toast = useToast()

const searchQuery = ref('')
const statusFilter = ref('all')
const planFilter = ref('all')
const sortBy = ref('name')
const sortOrder = ref<'asc' | 'desc'>('asc')
const currentPage = ref(1)
const itemsPerPage = 10

const mockTenants = [
  { id: 'tenant_1', name: 'acme-corp', displayName: 'Acme Corporation', domain: 'acme.edu', subdomain: 'acme', status: 'active', plan: 'enterprise', settings: { branding: { name: 'Acme Learning' }}, organizationIds: ['org_1', 'org_2'], createdAt: '2023-01-15', updatedAt: '2024-01-10', logo: null },
  { id: 'tenant_2', name: 'university-state', displayName: 'State University', domain: 'stateu.edu', subdomain: 'stateu', status: 'active', plan: 'professional', settings: { branding: { name: 'StateU Online' }}, organizationIds: ['org_3', 'org_4', 'org_5'], createdAt: '2023-03-22', updatedAt: '2024-01-08', logo: null },
  { id: 'tenant_3', name: 'startup-inc', displayName: 'Startup Inc', domain: null, subdomain: 'startup', status: 'pending', plan: 'starter', settings: { branding: { name: 'Startup Academy' }}, organizationIds: ['org_6'], createdAt: '2024-01-10', updatedAt: '2024-01-15', logo: null },
  { id: 'tenant_4', name: 'training-co', displayName: 'Training Company', domain: 'training.co', subdomain: 'training', status: 'suspended', plan: 'professional', settings: { branding: { name: 'Training Co' }}, organizationIds: ['org_7', 'org_8'], createdAt: '2023-08-15', updatedAt: '2024-01-05', logo: null },
]

const tenants = computed(() => {
  let result = mockTenants
  
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(t => 
      t.name.toLowerCase().includes(query) ||
      t.displayName.toLowerCase().includes(query) ||
      t.domain?.toLowerCase().includes(query)
    )
  }
  
  if (statusFilter.value !== 'all') {
    result = result.filter(t => t.status === statusFilter.value)
  }
  
  if (planFilter.value !== 'all') {
    result = result.filter(t => t.plan === planFilter.value)
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

const paginatedTenants = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  return tenants.value.slice(start, start + itemsPerPage)
})

const totalPages = computed(() => Math.ceil(tenants.value.length / itemsPerPage))

const getStatusBadge = (status: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'destructive' | 'info' | 'secondary'> = {
    active: 'success',
    pending: 'warning',
    suspended: 'destructive',
    deleted: 'secondary',
  }
  return variants[status] || 'default'
}

const getPlanBadge = (plan: string) => {
  const variants: Record<string, 'default' | 'success' | 'warning' | 'info' | 'secondary'> = {
    free: 'secondary',
    starter: 'info',
    professional: 'warning',
    enterprise: 'default',
    custom: 'secondary',
  }
  return variants[plan] || 'default'
}

const switchTenant = (tenantId: string) => {
  props.onSwitchTenant?.(tenantId)
  toast.success('Switched to tenant')
}

const copyTenantId = (tenantId: string) => {
  navigator.clipboard.writeText(tenantId)
  toast.success('Tenant ID copied')
}
</script>

<template>
  <Card class="h-full">
    <CardHeader class="pb-3">
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <BuildingIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Tenant Switcher</CardTitle>
            <CardDescription>Manage and switch between tenants</CardDescription>
          </div>
        </div>
        
        <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
          <div class="relative flex-1 max-w-sm">
            <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              v-model="searchQuery"
              placeholder="Search tenants..."
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
              <SelectItem value="pending">Pending</SelectItem>
              <SelectItem value="suspended">Suspended</SelectItem>
              <SelectItem value="deleted">Deleted</SelectItem>
            </SelectContent>
          </Select>
          
          <Select v-model="planFilter" class="w-32">
            <SelectTrigger>
              <SelectValue placeholder="Plan" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Plans</SelectItem>
              <SelectItem value="free">Free</SelectItem>
              <SelectItem value="starter">Starter</SelectItem>
              <SelectItem value="professional">Professional</SelectItem>
              <SelectItem value="enterprise">Enterprise</SelectItem>
              <SelectItem value="custom">Custom</SelectItem>
            </SelectContent>
          </Select>
          
          <Button @click="showCreateTenant = true" class="gap-2">
            <PlusIcon class="h-4 w-4" />
            Create Tenant
          </Button>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <div class="overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead class="cursor-pointer" @click="sortBy = 'displayName'; sortOrder = sortBy === 'displayName' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    Tenant
                    <ChevronDownIcon v-if="sortBy === 'displayName' && sortOrder === 'desc'" class="h-3 w-3" />
                    <ChevronUpIcon v-if="sortBy === 'displayName' && sortOrder === 'asc'" class="h-3 w-3" />
                  </div>
                </TableHead>
                <TableHead class="hidden md:table-cell">
                  <div class="flex items-center gap-1">
                    Domain / Subdomain
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer hidden lg:table-cell" @click="sortBy = 'plan'; sortOrder = sortBy === 'plan' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    Plan
                  </div>
                </TableHead>
                <TableHead class="cursor-pointer" @click="sortBy = 'status'; sortOrder = sortBy === 'status' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    Status
                  </div>
                </TableHead>
                <TableHead class="hidden md:table-cell">
                  <div class="flex items-center gap-1">
                    Organizations
                  </div>
                </TableHead>
                <TableHead class="hidden md:table-cell" @click="sortBy = 'updatedAt'; sortOrder = sortBy === 'updatedAt' && sortOrder === 'asc' ? 'desc' : 'asc'">
                  <div class="flex items-center gap-1">
                    Updated
                  </div>
                </TableHead>
                <TableHead class="w-56">Actions</TableHead>
              </TableRow>
            </TableHeader>
            
            <TableBody>
              <TableRow v-for="tenant in paginatedTenants" :key="tenant.id" class="hover:bg-accent/50">
                <TableCell class="font-medium">
                  <div class="flex items-center gap-3">
                    <Avatar :size="sm" :src="tenant.logo" :fallback="tenant.displayName">
                      <template #icon>
                        <BuildingIcon class="h-4 w-4" />
                      </template>
                    </Avatar>
                    <div>
                      <p class="font-medium">{{ tenant.displayName }}</p>
                      <p class="text-sm text-muted-foreground font-mono text-xs">{{ tenant.name }}</p>
                    </div>
                  </div>
                </TableCell>
                <TableCell class="hidden md:table-cell text-sm">
                  <div>
                    <p v-if="tenant.domain" class="font-mono text-sm">{{ tenant.domain }}</p>
                    <p class="text-xs text-muted-foreground">{{ tenant.subdomain }}.axel.local</p>
                  </div>
                </TableCell>
                <TableCell class="hidden lg:table-cell">
                  <Badge :variant="getPlanBadge(tenant.plan)">{{ tenant.plan }}</Badge>
                </TableCell>
                <TableCell>
                  <Badge :variant="getStatusBadge(tenant.status)">{{ tenant.status }}</Badge>
                </TableCell>
                <TableCell class="hidden md:table-cell text-sm text-muted-foreground">
                  {{ tenant.organizationIds.length }}
                </TableCell>
                <TableCell class="hidden md:table-cell text-sm text-muted-foreground">
                  {{ formatRelativeTime(tenant.updatedAt) }}
                </TableCell>
                <TableCell>
                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Button variant="ghost" size="icon" class="h-8 w-8">
                        <MoreHorizontalIcon class="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem @click="switchTenant(tenant.id)" :class="{ 'font-semibold': currentTenantId === tenant.id }">
                        <EyeIcon class="h-4 w-4" />
                        {{ currentTenantId === tenant.id ? 'Current' : 'Switch To' }}
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="showEditTenant = tenant">
                        <EditIcon class="h-4 w-4" />
                        Edit Tenant
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="copyTenantId(tenant.id)">
                        <CopyIcon class="h-4 w-4" />
                        Copy ID
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem @click="handleAction('impersonate', tenant)">
                        <ShieldIcon class="h-4 w-4" />
                        Impersonate Admin
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('manage_orgs', tenant)">
                        <UsersIcon class="h-4 w-4" />
                        Manage Organizations
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem @click="handleAction('suspend', tenant)" class="text-destructive" v-if="tenant.status === 'active'">
                        <PauseIcon class="h-4 w-4" />
                        Suspend
                      </DropdownMenuItem>
                      <DropdownMenuItem @click="handleAction('activate', tenant)" class="text-success" v-if="tenant.status === 'suspended'">
                        <PlayIcon class="h-4 w-4" />
                        Activate
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
        
        <div v-if="totalPages > 1" class="flex items-center justify-between mt-6">
          <div class="text-sm text-muted-foreground">
            Showing {{ (currentPage - 1) * itemsPerPage + 1 }} to {{ Math.min(currentPage * itemsPerPage, tenants.length) }} of {{ tenants.length }} tenants
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
const BuildingIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4' })]) }})
const PauseIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '6', y: '4', width: '4', height: '16' }), h('rect', { x: '14', y: '4', width: '4', height: '16' })]) }})
const PlayIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M8 5v14l11-7z' })]) }})
</script>