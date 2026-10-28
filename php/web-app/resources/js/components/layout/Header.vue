<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import { Link } from '@inertiajs/vue3'
import { Button } from '@/components/ui/button'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { ThemeToggle } from '@/components/layout/ThemeToggle'
import { LanguageSwitcher } from '@/components/layout/LanguageSwitcher'
import { NotificationBell } from '@/components/layout/NotificationBell'
import { SearchCommand } from '@/components/layout/SearchCommand'
import { useAuth } from '@/composables/useAuth'
import { BrandLogo } from '@/components/ui/BrandLogo'
import { 
  MenuIcon, XIcon, ChevronLeftIcon, ChevronRightIcon, 
  HomeIcon, BookOpenIcon, MapIcon, CalendarIcon, UsersIcon,
  LayoutDashboardIcon, GraduationCapIcon, SettingsIcon, BarChartIcon,
  ShieldIcon, LogOutIcon, LogInIcon, UserPlusIcon
} from '@/components/ui/icons'

const route = useRoute()
const auth = useAuth()
const mobileMenuOpen = ref(false)
const userMenuOpen = ref(false)
const scrolled = ref(false)

const navigation = [
  { name: 'Dashboard', href: '/dashboard', icon: HomeIcon },
  { name: 'Courses', href: '/courses', icon: BookOpenIcon },
  { name: 'Learning Paths', href: '/learning-paths', icon: MapIcon },
  { name: 'Calendar', href: '/calendar', icon: CalendarIcon },
  { name: 'Community', href: '/community', icon: UsersIcon },
]

const adminNavigation = [
  { name: 'Overview', href: '/admin', icon: LayoutDashboardIcon },
  { name: 'Users', href: '/admin/users', icon: UsersIcon },
  { name: 'Courses', href: '/admin/courses', icon: BookOpenIcon },
  { name: 'Enrollments', href: '/admin/enrollments', icon: GraduationCapIcon },
  { name: 'Analytics', href: '/admin/analytics', icon: BarChartIcon },
  { name: 'Settings', href: '/admin/settings', icon: SettingsIcon },
]

onMounted(() => {
  window.addEventListener('scroll', handleScroll)
  handleScroll()
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})

const handleScroll = () => {
  scrolled.value = window.scrollY > 10
}
</script>

<template>
  <header
    :class="[
      'fixed top-0 left-0 right-0 z-40 transition-all duration-200',
      'bg-background/80 backdrop-blur-lg border-b border-border',
      scrolled ? 'shadow-sm' : '',
    ]"
  >
    <div class="container mx-auto px-4">
      <div class="flex h-16 items-center justify-between">
        <!-- Logo & Mobile Menu -->
        <div class="flex items-center gap-4">
          <button
            v-show="mobileMenuOpen"
            @click="mobileMenuOpen = false"
            class="lg:hidden p-2 rounded-md text-muted-foreground hover:bg-accent"
            aria-label="Close menu"
          >
            <XIcon class="h-6 w-6" />
          </button>
          <button
            v-show="!mobileMenuOpen"
            @click="mobileMenuOpen = true"
            class="lg:hidden p-2 rounded-md text-muted-foreground hover:bg-accent"
            aria-label="Open menu"
          >
            <MenuIcon class="h-6 w-6" />
          </button>

          <Link
            href="/"
            class="flex items-center gap-2 font-display font-bold text-xl text-foreground"
            aria-label="Axel LMS Home"
          >
            <BrandLogo brand="axel" variant="icon" size="lg" class="h-8 w-8" />
            <span class="hidden sm:block">Axel LMS</span>
          </Link>
        </div>

        <!-- Desktop Navigation -->
        <nav class="hidden lg:flex lg:items-center lg:gap-1 lg:px-2" aria-label="Main navigation">
          <template v-for="item in navigation" :key="item.name">
            <Link
              :href="item.href"
              class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground hover:bg-accent"
              :class="{ 'text-foreground bg-accent': route().current(item.href) }"
            >
              <item.icon class="h-4 w-4" />
              <span>{{ item.name }}</span>
            </Link>
          </template>
        </nav>

        <!-- Search -->
        <div class="hidden lg:block lg:w-64">
          <SearchCommand />
        </div>

        <!-- Right Side Actions -->
        <div class="flex items-center gap-2">
          <!-- Theme Toggle -->
          <ThemeToggle />

          <!-- Language Switcher -->
          <LanguageSwitcher />

          <!-- Notifications -->
          <NotificationBell />

          <!-- User Menu -->
          <div v-if="auth.user" class="relative">
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button variant="ghost" class="relative h-10 w-10 rounded-full p-0">
                  <Avatar :size="size" :src="auth.user?.avatar_url" :fallback="auth.user?.name">
                    <template #icon>
                      <UserIcon class="h-5 w-5" />
                    </template>
                  </Avatar>
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent class="w-56" align="end" force-mount>
                <div class="flex items-center gap-3 p-2">
                  <Avatar :size="lg" :src="auth.user?.avatar_url" :fallback="auth.user?.name" />
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium truncate">{{ auth.user?.name }}</p>
                    <p class="text-xs text-muted-foreground truncate">{{ auth.user?.email }}</p>
                  </div>
                </div>
                <DropdownMenuSeparator />
                <DropdownMenuItem as-child>
                  <Link href="/profile" class="flex w-full items-center gap-2">
                    <UserIcon class="h-4 w-4" />
                    Profile
                  </Link>
                </DropdownMenuItem>
                <DropdownMenuItem as-child>
                  <Link href="/settings" class="flex w-full items-center gap-2">
                    <SettingsIcon class="h-4 w-4" />
                    Settings
                  </Link>
                </DropdownMenuItem>
                <DropdownMenuItem as-child v-if="auth.hasRole('admin')">
                  <Link href="/admin" class="flex w-full items-center gap-2">
                    <LayoutDashboardIcon class="h-4 w-4" />
                    Admin Panel
                  </Link>
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem @click="auth.logout()" class="text-destructive-600 focus:text-destructive-600">
                  <LogOutIcon class="h-4 w-4" />
                  Sign Out
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>

          <!-- Auth Buttons (when not logged in) -->
          <div v-else class="hidden lg:flex lg:items-center lg:gap-2">
            <Link href="/login">
              <Button variant="ghost" size="sm">Sign In</Button>
            </Link>
            <Link href="/register">
              <Button size="sm">Get Started</Button>
            </Link>
          </div>
        </div>
      </div>
    </div>

    <!-- Mobile Menu -->
    <div
      v-show="mobileMenuOpen"
      class="lg:hidden border-t border-border bg-background animate-in slide-in-from-top"
    >
      <nav class="px-4 py-4 space-y-2" aria-label="Mobile navigation">
        <template v-for="item in navigation" :key="item.name">
          <Link
            :href="item.href"
            @click="mobileMenuOpen = false"
            class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent"
            :class="{ 'text-foreground bg-accent': route().current(item.href) }"
          >
            <item.icon class="h-5 w-5" />
            <span>{{ item.name }}</span>
          </Link>
        </template>
        <div class="pt-4 border-t border-border">
          <Link
            href="/login"
            @click="mobileMenuOpen = false"
            class="flex w-full items-center justify-center gap-2 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent"
          >
            <LogInIcon class="h-4 w-4" />
            Sign In
          </Link>
          <Link
            href="/register"
            @click="mobileMenuOpen = false"
            class="flex w-full items-center justify-center gap-2 rounded-lg px-3 py-2 text-sm font-medium"
          >
            <UserPlusIcon class="h-4 w-4" />
            Get Started
          </Link>
        </div>
      </nav>
    </div>
  </header>
</template>

<script>
// Icon components (inline for performance)
const HomeIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 011-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6' })]) }})
const BookOpenIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const MapIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 11a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const CalendarIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z' })]) }})
const UsersIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })]) }})
const LayoutDashboardIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z' })]) }})
const GraduationCapIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l9-5-9-5-9 5 9 5z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 14l9-5-9-5-9 5 9 5zm0 0l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14zm-4 6v-7.5l4-2.222' })]) }})
const SettingsIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' })]) }})
const BarChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 012-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' })]) }})
const ShieldIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
const XIcon = defineComponent({ render() { return h('svg', { class: 'h-6 w-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M6 18L18 6M6 6l12 12' })]) }})
const MenuIcon = defineComponent({ render() { return h('svg', { class: 'h-6 w-6', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 6h16M4 12h16M4 18h16' })]) }})
const UserIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z' })]) }})
const LogOutIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1' })]) }})
const LogInIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1' })]) }})
const UserPlusIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z' })]) }})
</script>