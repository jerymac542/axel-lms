<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import { Link } from '@inertiajs/vue3'
import { Button } from '@/components/ui/button'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { Collapsible, CollapsibleTrigger, CollapsibleContent } from '@/components/ui/collapsible'
import { useAuth } from '@/composables/useAuth'
import { cn } from '@/utils/cn'

const route = useRoute()
const auth = useAuth()
const isCollapsed = ref(false)
const openSections = ref<string[]>(['courses', 'learning'])

const navigation = [
  {
    section: 'learning',
    label: 'Learning',
    icon: BookOpenIcon,
    items: [
      { name: 'Dashboard', href: '/dashboard', icon: LayoutDashboardIcon, badge: null },
      { name: 'My Courses', href: '/my-courses', icon: GraduationCapIcon, badge: auth.user?.stats?.active_courses || 0 },
      { name: 'Learning Paths', href: '/learning-paths', icon: MapIcon, badge: null },
      { name: 'Schedule', href: '/schedule', icon: CalendarIcon, badge: null },
      { name: 'Achievements', href: '/achievements', icon: TrophyIcon, badge: auth.user?.stats?.badges_earned || 0 },
      { name: 'Certificates', href: '/certificates', icon: AwardIcon, badge: auth.user?.stats?.certificates_earned || 0 },
    ],
  },
  {
    section: 'community',
    label: 'Community',
    icon: UsersIcon,
    items: [
      { name: 'Discussions', href: '/discussions', icon: MessageSquareIcon, badge: auth.user?.stats?.unread_discussions || 0 },
      { name: 'Study Groups', href: '/study-groups', icon: UsersIcon, badge: null },
      { name: 'Messages', href: '/messages', icon: MailIcon, badge: auth.user?.stats?.unread_messages || 0 },
      { name: 'Peers', href: '/peers', icon: UserPlusIcon, badge: null },
    ],
  },
  {
    section: 'resources',
    label: 'Resources',
    icon: LibraryIcon,
    items: [
      { name: 'Library', href: '/library', icon: BookIcon, badge: null },
      { name: 'Notes', href: '/notes', icon: FileTextIcon, badge: null },
      { name: 'Bookmarks', href: '/bookmarks', icon: BookmarkIcon, badge: auth.user?.stats?.bookmarks_count || 0 },
      { name: 'Downloads', href: '/downloads', icon: DownloadIcon, badge: null },
    ],
  },
]

const adminNavigation = [
  {
    section: 'admin',
    label: 'Administration',
    icon: SettingsIcon,
    items: [
      { name: 'Overview', href: '/admin', icon: LayoutDashboardIcon, badge: null },
      { name: 'Users', href: '/admin/users', icon: UsersIcon, badge: null },
      { name: 'Courses', href: '/admin/courses', icon: BookOpenIcon, badge: null },
      { name: 'Enrollments', href: '/admin/enrollments', icon: GraduationCapIcon, badge: null },
      { name: 'Analytics', href: '/admin/analytics', icon: BarChartIcon, badge: null },
      { name: 'Settings', href: '/admin/settings', icon: SettingsIcon, badge: null },
    ],
  },
]

const user = computed(() => auth.user)

onMounted(() => {
  // Restore collapsed state from localStorage
  const saved = localStorage.getItem('sidebar-collapsed')
  if (saved !== null) {
    isCollapsed.value = JSON.parse(saved)
  }
})

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
  localStorage.setItem('sidebar-collapsed', JSON.stringify(isCollapsed.value))
}

const toggleSection = (section: string) => {
  if (openSections.value.includes(section)) {
    openSections.value = openSections.value.filter(s => s !== section)
  } else {
    openSections.value.push(section)
  }
}

const isSectionOpen = (section: string) => openSections.value.includes(section)

const isActive = (href: string) => route().current(href) || route().current(`${href}/*`)
</script>

<template>
  <aside
    :class="[
      'fixed left-0 top-16 bottom-0 z-30 transition-all duration-300 ease-in-out',
      'bg-card border-r border-border overflow-y-auto',
      isCollapsed ? 'w-16' : 'w-64',
    ]"
    :aria-label="isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'"
  >
    <!-- Collapse Toggle -->
    <button
      @click="toggleCollapse"
      class="absolute -right-3 top-4 z-10 flex h-10 w-10 items-center justify-center rounded-full bg-background border border-border shadow-lg hover:bg-accent transition-all duration-200 lg:hidden"
      :aria-label="isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'"
      :aria-expanded="!isCollapsed"
    >
      <ChevronLeftIcon v-if="!isCollapsed" class="h-5 w-5" />
      <ChevronRightIcon v-else class="h-5 w-5" />
    </button>

    <!-- Sidebar Content -->
    <div class="flex flex-col h-full p-3">
      <!-- User Profile (when not collapsed) -->
      <div v-show="!isCollapsed" class="mb-4 p-2">
        <Link href="/profile" class="flex items-center gap-3 p-2 rounded-lg hover:bg-accent transition-colors">
          <Avatar :size="lg" :src="user?.avatar_url" :fallback="user?.name" class="flex-shrink-0" />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium truncate">{{ user?.name }}</p>
            <p class="text-xs text-muted-foreground truncate">{{ user?.email }}</p>
          </div>
          <Badge variant="secondary" class="ml-auto">{{ user?.role_display }}</Badge>
        </Link>
      </div>

      <!-- Navigation -->
      <nav class="flex-1 space-y-1" aria-label="Sidebar navigation">
        <template v-for="group in navigation" :key="group.section">
          <Collapsible :open="isSectionOpen(group.section)">
            <CollapsibleTrigger
              @click="toggleSection(group.section)"
              class="flex w-full items-center gap-2 rounded-lg px-2 py-1.5 text-xs font-semibold text-muted-foreground uppercase tracking-wider hover:text-foreground transition-colors"
            >
              <component :is="group.icon" class="h-4 w-4 flex-shrink-0" />
              <span v-show="!isCollapsed" class="flex-1 text-left">{{ group.label }}</span>
              <ChevronDownIcon
                class="h-4 w-4 flex-shrink-0 transition-transform duration-200"
                :class="{ 'rotate-180': isSectionOpen(group.section) }"
              />
            </CollapsibleTrigger>

            <CollapsibleContent class="overflow-hidden data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:slide-up-2 data-[state=open]:slide-down-2">
              <ul class="space-y-0.5 px-1" role="list">
                <li v-for="item in group.items" :key="item.name">
                  <Link
                    :href="item.href"
                    class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200"
                    :class="[
                      'text-muted-foreground hover:text-foreground hover:bg-accent',
                      isActive(item.href) ? 'text-primary-600 dark:text-primary-400 bg-primary-50 dark:bg-primary-900/20' : '',
                    ]"
                    @click="$event.stopPropagation()"
                  >
                    <component :is="item.icon" class="h-5 w-5 flex-shrink-0" />
                    <span v-show="!isCollapsed" class="flex-1 truncate">{{ item.name }}</span>
                    <Badge
                      v-if="item.badge && item.badge > 0"
                      variant="default"
                      class="ml-auto flex-shrink-0"
                      :class="{ 'hidden': isCollapsed }"
                    >
                      {{ item.badge > 99 ? '99+' : item.badge }}
                    </Badge>
                  </Link>
                </li>
              </ul>
            </CollapsibleContent>
          </Collapsible>
        </template>

        <!-- Admin Navigation -->
        <template v-if="auth.hasRole('admin')">
          <div class="pt-4 mt-4 border-t border-border" v-show="!isCollapsed">
            <p class="px-3 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
              Administration
            </p>
          </div>
          <template v-for="group in adminNavigation" :key="group.section">
            <Collapsible :open="isSectionOpen(group.section)">
              <CollapsibleTrigger
                @click="toggleSection(group.section)"
                class="flex w-full items-center gap-2 rounded-lg px-2 py-1.5 text-xs font-semibold text-muted-foreground uppercase tracking-wider hover:text-foreground transition-colors"
              >
                <component :is="group.icon" class="h-4 w-4 flex-shrink-0" />
                <span v-show="!isCollapsed" class="flex-1 text-left">{{ group.label }}</span>
                <ChevronDownIcon
                  class="h-4 w-4 flex-shrink-0 transition-transform duration-200"
                  :class="{ 'rotate-180': isSectionOpen(group.section) }"
                />
              </CollapsibleTrigger>

              <CollapsibleContent class="overflow-hidden data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:slide-up-2 data-[state=open]:slide-down-2">
                <ul class="space-y-0.5 px-1" role="list">
                  <li v-for="item in group.items" :key="item.name">
                    <Link
                      :href="item.href"
                      class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200"
                      :class="[
                        'text-muted-foreground hover:text-foreground hover:bg-accent',
                        isActive(item.href) ? 'text-primary-600 dark:text-primary-400 bg-primary-50 dark:bg-primary-900/20' : '',
                      ]"
                      @click="$event.stopPropagation()"
                    >
                      <component :is="item.icon" class="h-5 w-5 flex-shrink-0" />
                      <span v-show="!isCollapsed" class="flex-1 truncate">{{ item.name }}</span>
                    </Link>
                  </li>
                </ul>
              </CollapsibleContent>
            </Collapsible>
          </template>
        </template>
      </nav>

      <!-- Bottom Actions -->
      <div v-show="!isCollapsed" class="border-t border-border pt-3 space-y-2">
        <Link
          href="/settings"
          class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
        >
          <SettingsIcon class="h-5 w-5 flex-shrink-0" />
          <span>Settings</span>
        </Link>
        <Link
          href="/help"
          class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
        >
          <HelpCircleIcon class="h-5 w-5 flex-shrink-0" />
          <span>Help & Support</span>
        </Link>
        <div class="pt-2">
          <button
            @click="auth.logout()"
            class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-destructive-600 hover:bg-destructive-50 dark:hover:bg-destructive-900/20 transition-colors"
          >
            <LogOutIcon class="h-5 w-5 flex-shrink-0" />
            <span>Sign Out</span>
          </button>
        </div>
      </div>

      <!-- Collapsed Bottom Icons -->
      <div v-show="isCollapsed" class="absolute bottom-0 left-0 right-0 p-3 space-y-1">
        <Link
          href="/settings"
          class="flex items-center justify-center rounded-lg p-2 text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
          title="Settings"
        >
          <SettingsIcon class="h-5 w-5" />
        </Link>
        <Link
          href="/help"
          class="flex items-center justify-center rounded-lg p-2 text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
          title="Help & Support"
        >
          <HelpCircleIcon class="h-5 w-5" />
        </Link>
        <button
          @click="auth.logout()"
          class="flex w-full items-center justify-center rounded-lg p-2 text-destructive-600 hover:bg-destructive-50 dark:hover:bg-destructive-900/20 transition-colors"
          title="Sign Out"
        >
          <LogOutIcon class="h-5 w-5" />
        </button>
      </div>
    </div>
  </aside>
</template>

<script>
// Icon components
const BookOpenIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const LayoutDashboardIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z' })]) }})
const GraduationCapIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l9-5-9-5-9 5 9 5z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l9-5-9-5-9 5 9 5zm0 0l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14zm-4 6v-7.5l4-2.222' })]) }})
const MapIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 11a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const CalendarIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z' })]) }})
const TrophyIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
const AwardIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const UsersIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })]) }})
const MessageSquareIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 12h4M8 12l2-2m0 0l2 2m-2-2v8m-8 0v8m0-8h8m-8 0h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949M3 12c0 4.418 4.03 8 9 8a9.863 9.863 0 014.255-.949' })]) }})
const MailIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z' })]) }})
const UserPlusIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z' })]) }})
const LibraryIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const FileTextIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })]) }})
const BookmarkIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z' })]) }})
const DownloadIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M7 10l5 5 5-5M17 10v10' })]) }})
const BookIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const SettingsIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const BarChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 012-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' })]) }})
const HelpCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const ChevronLeftIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 19l-7-7 7-7' })]) }})
const ChevronRightIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 5l7 7-7 7' })]) }})
const ChevronDownIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 9l-7 7-7-7' })]) }})
const LogOutIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1' })]) }})
</script>