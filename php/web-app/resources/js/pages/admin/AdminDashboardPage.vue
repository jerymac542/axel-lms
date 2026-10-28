<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { AdminLayout } from '@/layouts/AdminLayout'
import { AdminSidebar } from '@/components/admin/AdminSidebar'
import { AdminHeader } from '@/components/admin/AdminHeader'
import { AdminStats } from '@/components/admin/AdminStats'
import { UserTable } from '@/components/admin/UserTable'
import { CourseTable } from '@/components/admin/CourseTable'
import { EnrollmentTable } from '@/components/admin/EnrollmentTable'
import { AnalyticsChart } from '@/components/admin/AnalyticsChart'
import { SettingsPanel } from '@/components/admin/SettingsPanel'
import { RoleManager } from '@/components/admin/RoleManager'
import { TenantSwitcher } from '@/components/admin/TenantSwitcher'
import { AuditLog } from '@/components/admin/AuditLog'
import { SystemHealth } from '@/components/admin/SystemHealth'
import { useAuth } from '@/composables/useAuth'

const auth = useAuth()
const activeSection = ref('dashboard')

const sections = [
  { id: 'dashboard', label: 'Dashboard', icon: 'LayoutDashboardIcon', roles: ['admin', 'org_admin'] },
  { id: 'users', label: 'Users', icon: 'UsersIcon', roles: ['admin', 'org_admin'] },
  { id: 'courses', label: 'Courses', icon: 'BookOpenIcon', roles: ['admin', 'org_admin', 'instructor'] },
  { id: 'enrollments', label: 'Enrollments', icon: 'GraduationCapIcon', roles: ['admin', 'org_admin', 'instructor'] },
  { id: 'analytics', label: 'Analytics', icon: 'BarChartIcon', roles: ['admin', 'org_admin'] },
  { id: 'settings', label: 'Settings', icon: 'SettingsIcon', roles: ['admin', 'org_admin'] },
  { id: 'roles', label: 'Roles & Permissions', icon: 'ShieldIcon', roles: ['admin'] },
  { id: 'tenants', label: 'Tenants', icon: 'BuildingIcon', roles: ['admin'] },
  { id: 'audit', label: 'Audit Log', icon: 'ShieldCheckIcon', roles: ['admin'] },
  { id: 'health', label: 'System Health', icon: 'ActivityIcon', roles: ['admin'] },
]

const filteredSections = computed(() => 
  sections.filter(s => s.roles.some(r => auth.hasRole(r)))
)

const sectionComponents: Record<string, any> = {
  dashboard: AdminStats,
  users: UserTable,
  courses: CourseTable,
  enrollments: EnrollmentTable,
  analytics: AnalyticsChart,
  settings: SettingsPanel,
  roles: RoleManager,
  tenants: TenantSwitcher,
  audit: AuditLog,
  health: SystemHealth,
}

const currentComponent = computed(() => sectionComponents[activeSection.value] || AdminStats)
</script>

<template>
  <AdminLayout 
    :title="currentSectionTitle" 
    :description="currentSectionDescription"
  >
    <template #header>
      <div class="flex items-center gap-2">
        <Button 
          v-if="activeSection !== 'dashboard'" 
          variant="ghost" 
          size="icon" 
          @click="activeSection = 'dashboard'"
          class="md:hidden"
        >
          <LayoutDashboardIcon class="h-5 w-5" />
        </Button>
        <h2 class="font-semibold">{{ currentSectionTitle }}</h2>
      </div>
      <div class="flex items-center gap-2">
        <Button variant="ghost" size="sm" @click="refreshData" :disabled="loading" class="gap-1">
          <RefreshCwIcon class="h-4 w-4" :class="{ 'animate-spin': loading }" />
          Refresh
        </Button>
      </div>
    </template>

    <component 
      :is="currentComponent" 
      :loading="loading"
      @user-action="handleUserAction"
      @course-action="handleCourseAction"
    />
  </AdminLayout>
</template>

<script>
const auth = useAuth()

const activeSection = ref('dashboard')
const loading = ref(false)

const sections = [
  { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboardIcon, roles: ['admin', 'org_admin'] },
  { id: 'users', label: 'Users', icon: UsersIcon, roles: ['admin', 'org_admin'] },
  { id: 'courses', label: 'Courses', icon: BookOpenIcon, roles: ['admin', 'org_admin', 'instructor'] },
  { id: 'enrollments', label: 'Enrollments', icon: GraduationCapIcon, roles: ['admin', 'org_admin', 'instructor'] },
  { id: 'analytics', label: 'Analytics', icon: BarChartIcon, roles: ['admin', 'org_admin'] },
  { id: 'settings', label: 'Settings', icon: SettingsIcon, roles: ['admin', 'org_admin'] },
  { id: 'roles', label: 'Roles & Permissions', icon: ShieldIcon, roles: ['admin'] },
  { id: 'tenants', label: 'Tenants', icon: BuildingIcon, roles: ['admin'] },
  { id: 'audit', label: 'Audit Log', icon: ShieldCheckIcon, roles: ['admin'] },
  { id: 'health', label: 'System Health', icon: ActivityIcon, roles: ['admin'] },
]

const sectionComponents: Record<string, any> = {
  dashboard: () => import('@/components/admin/AdminStats.vue'),
  users: () => import('@/components/admin/UserTable.vue'),
  courses: () => import('@/components/admin/CourseTable.vue'),
  enrollments: () => import('@/components/admin/EnrollmentTable.vue'),
  analytics: () => import('@/components/admin/AnalyticsChart.vue'),
  settings: () => import('@/components/admin/SettingsPanel.vue'),
  roles: () => import('@/components/admin/RoleManager.vue'),
  tenants: () => import('@/components/admin/TenantSwitcher.vue'),
  audit: () => import('@/components/admin/AuditLog.vue'),
  health: () => import('@/components/admin/SystemHealth.vue'),
}

const filteredSections = computed(() => 
  sections.filter(s => s.roles.some(r => auth.hasRole(r)))
)

const currentComponent = computed(() => {
  const comp = sectionComponents[activeSection.value]
  return comp ? (typeof comp === 'function' ? comp() : comp) : sectionComponents.dashboard
})

const currentSectionTitle = computed(() => {
  const section = sections.find(s => s.id === activeSection.value)
  return section?.label || 'Dashboard'
})

const currentSectionDescription = computed(() => {
  const descriptions: Record<string, string> = {
    dashboard: 'Overview of platform metrics and activity',
    users: 'Manage user accounts and permissions',
    courses: 'Create and manage courses',
    enrollments: 'Manage course enrollments',
    analytics: 'View platform analytics and reports',
    settings: 'Configure platform settings',
    roles: 'Manage roles and permissions',
    tenants: 'Manage tenant organizations',
    audit: 'View audit logs and security events',
    health: 'Monitor system health and performance',
  }
  return descriptions[activeSection.value] || ''
}

const loading = ref(false)

const refreshData = async () => {
  loading.value = true
  await new Promise(r => setTimeout(r, 1000))
  loading.value = false
}

const handleUserAction = (action: string, user: any) => {
  console.log('User action:', action, user)
}

const handleCourseAction = (action: string, course: any) => {
  console.log('Course action:', action, course)
}

const refreshData = async () => {
  loading.value = true
  await new Promise(r => setTimeout(r, 1000))
  loading.value = false
}
</script>

<template>
  <AdminLayout 
    :title="currentSectionTitle" 
    :description="currentSectionDescription"
  >
    <template #header>
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 w-full">
        <div class="flex items-center gap-3">
          <Button 
            variant="ghost" 
            size="icon" 
            @click="activeSection = 'dashboard'"
            class="md:hidden"
          >
            <LayoutDashboardIcon class="h-5 w-5" />
          </Button>
          <h1 class="text-xl font-semibold">{{ currentSectionTitle }}</h1>
        </div>
        
        <div class="flex items-center gap-2">
          <div class="flex-1 max-w-xs hidden sm:block">
            <div class="relative">
              <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <Input 
                placeholder="Search..." 
                class="pl-10"
                size="sm"
              />
            </div>
          </div>
          
          <Button variant="ghost" size="sm" @click="refreshData" :disabled="loading" class="gap-1">
            <RefreshCwIcon class="h-4 w-4" :class="{ 'animate-spin': loading }" />
            Refresh
          </Button>
        </div>
      </div>
    </template>

    <component :is="currentComponent" :loading="loading" />
  </AdminLayout>
</template>

<script>
const LayoutDashboardIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z' })]) }})
const UsersIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })]) }})
const BookOpenIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const GraduationCapIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l9-5-9-5-9 5 9 5z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l9-5-9-5-9 5 9 5zm0 0l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14zm-4 6v-7.5l4-2.222' })]) }})
const BarChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 012-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' })]) }})
const SettingsIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const ShieldIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
const BuildingIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4' })]) }})
const ShieldCheckIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
const ActivityIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 2L3 14h9l-1 8 10-12h-9l11-4 10-12H6l-1 8 10-12' })]) }})
const SearchIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('circle', { cx: '11', cy: '11', r: '8', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 21l-4.35-4.35' })]) }})
const RefreshCwIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15' })]) }})
</script>