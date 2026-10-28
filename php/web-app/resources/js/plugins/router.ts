import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import { useAuth } from '@/composables/useAuth'

// Public routes
const publicRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('@/pages/HomePage.vue'),
    meta: { title: 'Home', public: true },
  },
  {
    path: '/features',
    name: 'features',
    component: () => import('@/pages/FeaturesPage.vue'),
    meta: { title: 'Features', public: true },
  },
  {
    path: '/pricing',
    name: 'pricing',
    component: () => import('@/pages/PricingPage.vue'),
    meta: { title: 'Pricing', public: true },
  },
  {
    path: '/about',
    name: 'about',
    component: () => import('@/pages/AboutPage.vue'),
    meta: { title: 'About', public: true },
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('@/pages/auth/LoginPage.vue'),
    meta: { title: 'Sign In', public: true, layout: 'AuthLayout', guestOnly: true },
  },
  {
    path: '/register',
    name: 'register',
    component: () => import('@/pages/auth/RegisterPage.vue'),
    meta: { title: 'Sign Up', public: true, layout: 'AuthLayout', guestOnly: true },
  },
  {
    path: '/forgot-password',
    name: 'forgot-password',
    component: () => import('@/pages/auth/ForgotPasswordPage.vue'),
    meta: { title: 'Forgot Password', public: true, layout: 'AuthLayout', guestOnly: true },
  },
  {
    path: '/reset-password/:token',
    name: 'reset-password',
    component: () => import('@/pages/auth/ResetPasswordPage.vue'),
    meta: { title: 'Reset Password', public: true, layout: 'AuthLayout', guestOnly: true },
  },
  {
    path: '/verify-email/:token',
    name: 'verify-email',
    component: () => import('@/pages/auth/VerifyEmailPage.vue'),
    meta: { title: 'Verify Email', public: true, layout: 'AuthLayout' },
  },
  {
    path: '/courses',
    name: 'courses',
    component: () => import('@/pages/courses/CoursesPage.vue'),
    meta: { title: 'Courses', public: true },
  },
  {
    path: '/courses/:id',
    name: 'course-detail',
    component: () => import('@/pages/courses/CourseDetailPage.vue'),
    meta: { title: 'Course Details', public: true },
  },
  {
    path: '/learning-paths',
    name: 'learning-paths',
    component: () => import('@/pages/learning-paths/LearningPathsPage.vue'),
    meta: { title: 'Learning Paths', public: true },
  },
  {
    path: '/learning-paths/:id',
    name: 'learning-path-detail',
    component: () => import('@/pages/learning-paths/LearningPathDetailPage.vue'),
    meta: { title: 'Learning Path Details', public: true },
  },
]

// Authenticated routes
const authRoutes: RouteRecordRaw[] = [
  {
    path: '/dashboard',
    name: 'dashboard',
    component: () => import('@/pages/dashboard/DashboardPage.vue'),
    meta: { title: 'Dashboard', layout: 'DashboardLayout', auth: true },
    children: [
      {
        path: '',
        name: 'dashboard-home',
        component: () => import('@/pages/dashboard/HomePage.vue'),
      },
      {
        path: 'courses',
        name: 'my-courses',
        component: () => import('@/pages/dashboard/MyCoursesPage.vue'),
        meta: { title: 'My Courses' },
      },
      {
        path: 'learning-paths',
        name: 'my-learning-paths',
        component: () => import('@/pages/dashboard/MyLearningPathsPage.vue'),
        meta: { title: 'My Learning Paths' },
      },
      {
        path: 'schedule',
        name: 'schedule',
        component: () => import('@/pages/dashboard/SchedulePage.vue'),
        meta: { title: 'Schedule' },
      },
      {
        path: 'achievements',
        name: 'achievements',
        component: () => import('@/pages/dashboard/AchievementsPage.vue'),
        meta: { title: 'Achievements' },
      },
      {
        path: 'certificates',
        name: 'certificates',
        component: () => import('@/pages/dashboard/CertificatesPage.vue'),
        meta: { title: 'Certificates' },
      },
      {
        path: 'notes',
        name: 'notes',
        component: () => import('@/pages/dashboard/NotesPage.vue'),
        meta: { title: 'Notes' },
      },
      {
        path: 'bookmarks',
        name: 'bookmarks',
        component: () => import('@/pages/dashboard/BookmarksPage.vue'),
        meta: { title: 'Bookmarks' },
      },
      {
        path: 'settings',
        name: 'settings',
        component: () => import('@/pages/dashboard/SettingsPage.vue'),
        meta: { title: 'Settings' },
      },
      {
        path: 'profile',
        name: 'profile',
        component: () => import('@/pages/dashboard/ProfilePage.vue'),
        meta: { title: 'Profile' },
      },
    ],
  },
  {
    path: '/courses/:courseId/learn',
    name: 'course-learn',
    component: () => import('@/pages/courses/CourseLearnPage.vue'),
    meta: { title: 'Learning', layout: 'CourseLayout', auth: true },
  },
  {
    path: '/learning-paths/:pathId/learn',
    name: 'learning-path-learn',
    component: () => import('@/pages/learning-paths/LearningPathLearnPage.vue'),
    meta: { title: 'Learning', layout: 'CourseLayout', auth: true },
  },
  {
    path: '/calendar',
    name: 'calendar',
    component: () => import('@/pages/calendar/CalendarPage.vue'),
    meta: { title: 'Calendar', auth: true },
  },
  {
    path: '/community',
    name: 'community',
    component: () => import('@/pages/community/CommunityPage.vue'),
    meta: { title: 'Community', auth: true },
  },
  {
    path: '/library',
    name: 'library',
    component: () => import('@/pages/library/LibraryPage.vue'),
    meta: { title: 'Library', auth: true },
  },
  {
    path: '/messages',
    name: 'messages',
    component: () => import('@/pages/messages/MessagesPage.vue'),
    meta: { title: 'Messages', auth: true },
  },
  {
    path: '/study-groups',
    name: 'study-groups',
    component: () => import('@/pages/study-groups/StudyGroupsPage.vue'),
    meta: { title: 'Study Groups', auth: true },
  },
]

// Admin routes
const adminRoutes: RouteRecordRaw[] = [
  {
    path: '/admin',
    name: 'admin',
    component: () => import('@/pages/admin/AdminDashboardPage.vue'),
    meta: { title: 'Admin Dashboard', layout: 'AdminLayout', auth: true, roles: ['admin'] },
    children: [
      {
        path: '',
        name: 'admin-home',
        component: () => import('@/pages/admin/HomePage.vue'),
      },
      {
        path: 'users',
        name: 'admin-users',
        component: () => import('@/pages/admin/users/UsersPage.vue'),
        meta: { title: 'Users' },
      },
      {
        path: 'users/:id',
        name: 'admin-user-detail',
        component: () => import('@/pages/admin/users/UserDetailPage.vue'),
        meta: { title: 'User Details' },
      },
      {
        path: 'courses',
        name: 'admin-courses',
        component: () => import('@/pages/admin/courses/CoursesPage.vue'),
        meta: { title: 'Courses' },
      },
      {
        path: 'courses/:id',
        name: 'admin-course-detail',
        component: () => import('@/pages/admin/courses/CourseDetailPage.vue'),
        meta: { title: 'Course Details' },
      },
      {
        path: 'courses/create',
        name: 'admin-course-create',
        component: () => import('@/pages/admin/courses/CreateCoursePage.vue'),
        meta: { title: 'Create Course' },
      },
      {
        path: 'enrollments',
        name: 'admin-enrollments',
        component: () => import('@/pages/admin/enrollments/EnrollmentsPage.vue'),
        meta: { title: 'Enrollments' },
      },
      {
        path: 'analytics',
        name: 'admin-analytics',
        component: () => import('@/pages/admin/analytics/AnalyticsPage.vue'),
        meta: { title: 'Analytics' },
      },
      {
        path: 'settings',
        name: 'admin-settings',
        component: () => import('@/pages/admin/settings/SettingsPage.vue'),
        meta: { title: 'Settings' },
      },
      {
        path: 'roles',
        name: 'admin-roles',
        component: () => import('@/pages/admin/roles/RolesPage.vue'),
        meta: { title: 'Roles & Permissions' },
      },
      {
        path: 'tenants',
        name: 'admin-tenants',
        component: () => import('@/pages/admin/tenants/TenantsPage.vue'),
        meta: { title: 'Tenants', roles: ['super_admin'] },
      },
      {
        path: 'audit-logs',
        name: 'admin-audit-logs',
        component: () => import('@/pages/admin/audit/AuditLogsPage.vue'),
        meta: { title: 'Audit Logs' },
      },
      {
        path: 'system-health',
        name: 'admin-system-health',
        component: () => import('@/pages/admin/system/SystemHealthPage.vue'),
        meta: { title: 'System Health' },
      },
    ],
  },
]

// Error routes
const errorRoutes: RouteRecordRaw[] = [
  {
    path: '/403',
    name: 'forbidden',
    component: () => import('@/pages/errors/ForbiddenPage.vue'),
    meta: { title: 'Forbidden', public: true, layout: 'AuthLayout' },
  },
  {
    path: '/404',
    name: 'not-found',
    component: () => import('@/pages/errors/NotFoundPage.vue'),
    meta: { title: 'Not Found', public: true, layout: 'AuthLayout' },
  },
  {
    path: '/500',
    name: 'server-error',
    component: () => import('@/pages/errors/ServerErrorPage.vue'),
    meta: { title: 'Server Error', public: true, layout: 'AuthLayout' },
  },
  {
    path: '/maintenance',
    name: 'maintenance',
    component: () => import('@/pages/errors/MaintenancePage.vue'),
    meta: { title: 'Maintenance', public: true, layout: 'AuthLayout' },
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/404',
  },
]

const routes: RouteRecordRaw[] = [
  ...publicRoutes,
  ...authRoutes,
  ...adminRoutes,
  ...errorRoutes,
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    } else {
      return { top: 0 }
    }
  },
})

// Navigation guards
router.beforeEach(async (to, from, next) => {
  const auth = useAuth()
  
  // Initialize auth if not loaded
  if (auth.isLoading.value) {
    await auth.initAuth()
  }

  // Check if route requires authentication
  if (to.meta.auth && !auth.isAuthenticated.value) {
    return next({ name: 'login', query: { redirect: to.fullPath } })
  }

  // Check if route is for guests only
  if (to.meta.guestOnly && auth.isAuthenticated.value) {
    return next({ name: 'dashboard' })
  }

  // Check roles
  if (to.meta.roles) {
    const requiredRoles = Array.isArray(to.meta.roles) ? to.meta.roles : [to.meta.roles]
    const hasRole = requiredRoles.some((role: string) => auth.hasRole(role))
    if (!hasRole) {
      return next({ name: 'forbidden' })
    }
  }

  // Check permissions
  if (to.meta.permissions) {
    const requiredPermissions = Array.isArray(to.meta.permissions) ? to.meta.permissions : [to.meta.permissions]
    const hasPermission = requiredPermissions.every((perm: string) => auth.hasPermission(perm))
    if (!hasPermission) {
      return next({ name: 'forbidden' })
    }
  }

  // Set document title
  if (to.meta.title) {
    document.title = `${to.meta.title} | Axel LMS`
  }

  next()
})

router.afterEach((to) => {
  // Track page views for analytics
  if (typeof window !== 'undefined' && window.gtag) {
    window.gtag('event', 'page_view', {
      page_path: to.fullPath,
      page_title: to.meta.title || document.title,
    })
  }
})

export default router