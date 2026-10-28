import './bootstrap'
import '../css/app.css'

import { createApp, h, defineComponent } from 'vue'
import { createInertiaApp, Link, Head } from '@inertiajs/vue3'
import { createPinia } from 'pinia'
import { VueQueryPlugin } from '@tanstack/vue-query'
import { Toaster } from 'vue-sonner'
import { createHead } from '@vueuse/head'

// Import UI components
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem } from '@/components/ui/dropdown-menu'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableCaption } from '@/components/ui/table'
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from '@/components/ui/dialog'
import { Sheet, SheetTrigger, SheetContent, SheetHeader, SheetTitle, SheetDescription, SheetFooter } from '@/components/ui/sheet'
import { Tooltip, TooltipTrigger, TooltipContent } from '@/components/ui/tooltip'
import { Alert, AlertTitle, AlertDescription } from '@/components/ui/alert'
import { Separator } from '@/components/ui/separator'
import { Skeleton } from '@/components/ui/skeleton'
import { Progress } from '@/components/ui/progress'
import { Toggle } from '@/components/ui/toggle'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from '@/components/ui/accordion'
import { Popover, PopoverTrigger, PopoverContent } from '@/components/ui/popover'
import { HoverCard, HoverCardTrigger, HoverCardContent } from '@/components/ui/hover-card'
import { Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem, MenubarSeparator } from '@/components/ui/menubar'
import { Command, CommandInput, CommandList, CommandItem, CommandGroup, CommandSeparator } from '@/components/ui/command'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem, SelectGroup, SelectLabel } from '@/components/ui/select'
import { Checkbox } from '@/components/ui/checkbox'
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group'
import { Switch } from '@/components/ui/switch'
import { Slider } from '@/components/ui/slider'
import { Textarea } from '@/components/ui/textarea'
import { Form, FormField, FormItem, FormLabel, FormControl, FormDescription, FormMessage } from '@/components/ui/form'
import { Calendar } from '@/components/ui/calendar'
import { DatePicker } from '@/components/ui/date-picker'
import { Pagination } from '@/components/ui/pagination'
import { Breadcrumbs } from '@/components/ui/breadcrumbs'
import { Stepper } from '@/components/ui/stepper'
import { Rating } from '@/components/ui/rating'
import { FileUpload } from '@/components/ui/file-upload'
import { ColorPicker } from '@/components/ui/color-picker'
import { TimePicker } from '@/components/ui/time-picker'
import { TagInput } from '@/components/ui/tag-input'
import { MarkdownEditor } from '@/components/ui/markdown-editor'
import { CodeEditor } from '@/components/ui/code-editor'
import { ImageCropper } from '@/components/ui/image-cropper'
import { DataTable } from '@/components/ui/data-table'
import { Chart } from '@/components/ui/chart'
import { StatCard } from '@/components/ui/stat-card'
import { EmptyState } from '@/components/ui/empty-state'
import { LoadingState } from '@/components/ui/loading-state'
import { ErrorState } from '@/components/ui/error-state'

// Layout components
import { AppLayout } from '@/layouts/AppLayout'
import { AuthLayout } from '@/layouts/AuthLayout'
import { CourseLayout } from '@/layouts/CourseLayout'
import { DashboardLayout } from '@/layouts/DashboardLayout'
import { AdminLayout } from '@/layouts/AdminLayout'
import { Header } from '@/components/layout/Header'
import { Sidebar } from '@/components/layout/Sidebar'
import { Footer } from '@/components/layout/Footer'
import { MobileNav } from '@/components/layout/MobileNav'
import { UserMenu } from '@/components/layout/UserMenu'
import { NotificationBell } from '@/components/layout/NotificationBell'
import { SearchCommand } from '@/components/layout/SearchCommand'
import { LanguageSwitcher } from '@/components/layout/LanguageSwitcher'
import { ThemeToggle } from '@/components/layout/ThemeToggle'

// Course components
import { CourseCard } from '@/components/course/CourseCard'
import { CourseGrid } from '@/components/course/CourseGrid'
import { CourseHero } from '@/components/course/CourseHero'
import { CourseProgress } from '@/components/course/CourseProgress'
import { CourseOutline } from '@/components/course/CourseOutline'
import { CourseReviews } from '@/components/course/CourseReviews'
import { CourseInstructor } from '@/components/course/CourseInstructor'
import { CoursePricing } from '@/components/course/CoursePricing'
import { CourseMetadata } from '@/components/course/CourseMetadata'
import { EnrollmentButton } from '@/components/course/EnrollmentButton'
import { LearningPath } from '@/components/course/LearningPath'
import { ModuleProgress } from '@/components/course/ModuleProgress'
import { LessonPlayer } from '@/components/course/LessonPlayer'
import { DiscussionThread } from '@/components/course/DiscussionThread'
import { NotesPanel } from '@/components/course/NotesPanel'
import { BookmarkButton } from '@/components/course/BookmarkButton'
import { CertificatePreview } from '@/components/course/CertificatePreview'

// Dashboard components
import { DashboardStats } from '@/components/dashboard/DashboardStats'
import { RecentActivity } from '@/components/dashboard/RecentActivity'
import { UpcomingDeadlines } from '@/components/dashboard/UpcomingDeadlines'
import { ContinueLearning } from '@/components/dashboard/ContinueLearning'
import { RecommendedCourses } from '@/components/dashboard/RecommendedCourses'
import { LearningStreak } from '@/components/dashboard/LearningStreak'
import { SkillProgress } from '@/components/dashboard/SkillProgress'
import { CalendarWidget } from '@/components/dashboard/CalendarWidget'
import { QuickActions } from '@/components/dashboard/QuickActions'

// Admin components
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

// Composables
import { useTheme } from '@/composables/useTheme'
import { useAuth } from '@/composables/useAuth'
import { useToast } from '@/composables/useToast'
import { useModal } from '@/composables/useModal'
import { useClipboard } from '@/composables/useClipboard'
import { useDebounce } from '@/composables/useDebounce'
import { useLocalStorage } from '@/composables/useLocalStorage'
import { useMediaQuery } from '@/composables/useMediaQuery'
import { useIntersectionObserver } from '@/composables/useIntersectionObserver'
import { useKeyboardShortcuts } from '@/composables/useKeyboardShortcuts'
import { useInertia } from '@/composables/useInertia'
import { usePermissions } from '@/composables/usePermissions'
import { useFeatureFlags } from '@/composables/useFeatureFlags'

// Plugins
import { i18n } from '@/plugins/i18n'
import { router } from '@/plugins/router'
import { vPermission } from '@/plugins/permission'
import { vFeature } from '@/plugins/feature'
import { vTooltip } from '@/plugins/tooltip'
import { vClickOutside } from '@/plugins/click-outside'
import { vFocusTrap } from '@/plugins/focus-trap'
import { vAutoFocus } from '@/plugins/auto-focus'
import { vRipple } from '@/plugins/ripple'

// Styles
import 'vue-sonner/style.css'
import '@vueuse/head/dist/style.css'

// Types
declare module '@vue/runtime-core' {
  interface GlobalComponents {
    // UI
    Button: typeof Button
    Input: typeof Input
    Label: typeof Label
    Card: typeof Card
    CardHeader: typeof CardHeader
    CardTitle: typeof CardTitle
    CardDescription: typeof CardDescription
    CardContent: typeof CardContent
    CardFooter: typeof CardFooter
    Badge: typeof Badge
    Avatar: typeof Avatar
    AvatarImage: typeof AvatarImage
    AvatarFallback: typeof AvatarFallback
    DropdownMenu: typeof DropdownMenu
    DropdownMenuTrigger: typeof DropdownMenuTrigger
    DropdownMenuContent: typeof DropdownMenuContent
    DropdownMenuItem: typeof DropdownMenuItem
    Tabs: typeof Tabs
    TabsList: typeof TabsList
    TabsTrigger: typeof TabsTrigger
    TabsContent: typeof TabsContent
    Table: typeof Table
    TableHeader: typeof TableHeader
    TableBody: typeof TableBody
    TableRow: typeof TableRow
    TableHead: typeof TableHead
    TableCell: typeof TableCell
    TableCaption: typeof TableCaption
    Dialog: typeof Dialog
    DialogTrigger: typeof DialogTrigger
    DialogContent: typeof DialogContent
    DialogHeader: typeof DialogHeader
    DialogTitle: typeof DialogTitle
    DialogDescription: typeof DialogDescription
    DialogFooter: typeof DialogFooter
    Sheet: typeof Sheet
    SheetTrigger: typeof SheetTrigger
    SheetContent: typeof SheetContent
    SheetHeader: typeof SheetHeader
    SheetTitle: typeof SheetTitle
    SheetDescription: typeof SheetDescription
    SheetFooter: typeof SheetFooter
    Tooltip: typeof Tooltip
    TooltipTrigger: typeof TooltipTrigger
    TooltipContent: typeof TooltipContent
    Alert: typeof Alert
    AlertTitle: typeof AlertTitle
    AlertDescription: typeof AlertDescription
    Separator: typeof Separator
    Skeleton: typeof Skeleton
    Progress: typeof Progress
    Toggle: typeof Toggle
    ScrollArea: typeof ScrollArea
    Accordion: typeof Accordion
    AccordionItem: typeof AccordionItem
    AccordionTrigger: typeof AccordionTrigger
    AccordionContent: typeof AccordionContent
    Popover: typeof Popover
    PopoverTrigger: typeof PopoverTrigger
    PopoverContent: typeof PopoverContent
    HoverCard: typeof HoverCard
    HoverCardTrigger: typeof HoverCardTrigger
    HoverCardContent: typeof HoverCardContent
    Menubar: typeof Menubar
    MenubarMenu: typeof MenubarMenu
    MenubarTrigger: typeof MenubarTrigger
    MenubarContent: typeof MenubarContent
    MenubarItem: typeof MenubarItem
    MenubarSeparator: typeof MenubarSeparator
    Command: typeof Command
    CommandInput: typeof CommandInput
    CommandList: typeof CommandList
    CommandItem: typeof CommandItem
    CommandGroup: typeof CommandGroup
    CommandSeparator: typeof CommandSeparator
    Select: typeof Select
    SelectTrigger: typeof SelectTrigger
    SelectValue: typeof SelectValue
    SelectContent: typeof SelectContent
    SelectItem: typeof SelectItem
    SelectGroup: typeof SelectGroup
    SelectLabel: typeof SelectLabel
    Checkbox: typeof Checkbox
    RadioGroup: typeof RadioGroup
    RadioGroupItem: typeof RadioGroupItem
    Switch: typeof Switch
    Slider: typeof Slider
    Textarea: typeof Textarea
    Form: typeof Form
    FormField: typeof FormField
    FormItem: typeof FormItem
    FormLabel: typeof FormLabel
    FormControl: typeof FormControl
    FormDescription: typeof FormDescription
    FormMessage: typeof FormMessage
    Calendar: typeof Calendar
    DatePicker: typeof DatePicker
    Pagination: typeof Pagination
    Breadcrumbs: typeof Breadcrumbs
    Stepper: typeof Stepper
    Rating: typeof Rating
    FileUpload: typeof FileUpload
    ColorPicker: typeof ColorPicker
    TimePicker: typeof TimePicker
    TagInput: typeof TagInput
    MarkdownEditor: typeof MarkdownEditor
    CodeEditor: typeof CodeEditor
    ImageCropper: typeof ImageCropper
    DataTable: typeof DataTable
    Chart: typeof Chart
    StatCard: typeof StatCard
    EmptyState: typeof EmptyState
    LoadingState: typeof LoadingState
    ErrorState: typeof ErrorState

    // Layout
    AppLayout: typeof AppLayout
    AuthLayout: typeof AuthLayout
    CourseLayout: typeof CourseLayout
    DashboardLayout: typeof DashboardLayout
    AdminLayout: typeof AdminLayout
    Header: typeof Header
    Sidebar: typeof Sidebar
    Footer: typeof Footer
    MobileNav: typeof MobileNav
    UserMenu: typeof UserMenu
    NotificationBell: typeof NotificationBell
    SearchCommand: typeof SearchCommand
    LanguageSwitcher: typeof LanguageSwitcher
    ThemeToggle: typeof ThemeToggle

    // Course
    CourseCard: typeof CourseCard
    CourseGrid: typeof CourseGrid
    CourseHero: typeof CourseHero
    CourseProgress: typeof CourseProgress
    CourseOutline: typeof CourseOutline
    CourseReviews: typeof CourseReviews
    CourseInstructor: typeof CourseInstructor
    CoursePricing: typeof CoursePricing
    CourseMetadata: typeof CourseMetadata
    EnrollmentButton: typeof EnrollmentButton
    LearningPath: typeof LearningPath
    ModuleProgress: typeof ModuleProgress
    LessonPlayer: typeof LessonPlayer
    DiscussionThread: typeof DiscussionThread
    NotesPanel: typeof NotesPanel
    BookmarkButton: typeof BookmarkButton
    CertificatePreview: typeof CertificatePreview

    // Dashboard
    DashboardStats: typeof DashboardStats
    RecentActivity: typeof RecentActivity
    UpcomingDeadlines: typeof UpcomingDeadlines
    ContinueLearning: typeof ContinueLearning
    RecommendedCourses: typeof RecommendedCourses
    LearningStreak: typeof LearningStreak
    SkillProgress: typeof SkillProgress
    CalendarWidget: typeof CalendarWidget
    QuickActions: typeof QuickActions

    // Admin
    AdminSidebar: typeof AdminSidebar
    AdminHeader: typeof AdminHeader
    AdminStats: typeof AdminStats
    UserTable: typeof UserTable
    CourseTable: typeof CourseTable
    EnrollmentTable: typeof EnrollmentTable
    AnalyticsChart: typeof AnalyticsChart
    SettingsPanel: typeof SettingsPanel
    RoleManager: typeof RoleManager
    TenantSwitcher: typeof TenantSwitcher
    AuditLog: typeof AuditLog
    SystemHealth: typeof SystemHealth

    // Inertia
    Link: typeof Link
    Head: typeof Head
  }
}

// Create Inertia app
createInertiaApp({
  title: (title) => `${title} | Axel LMS`,
  resolve: (name) => {
    const pages = import.meta.glob('./pages/**/*.vue', { eager: true })
    const page = pages[`./pages/${name}.vue`]
    if (!page) {
      throw new Error(`Page not found: ${name}`)
    }
    return page.default
  },
  setup({ el, App, props, plugin }) {
    const pinia = createPinia()
    const head = createHead()

    const app = createApp({ render: () => h(App, props) })
      .use(plugin)
      .use(pinia)
      .use(head)
      .use(VueQueryPlugin, {
        queryClientConfig: {
          defaultOptions: {
            queries: {
              staleTime: 1000 * 60 * 5, // 5 minutes
              retry: 1,
              refetchOnWindowFocus: false,
            },
          },
        },
      })
      .use(i18n)
      .use(router)

    // Global directives
    app.directive('permission', vPermission)
    app.directive('feature', vFeature)
    app.directive('tooltip', vTooltip)
    app.directive('click-outside', vClickOutside)
    app.directive('focus-trap', vFocusTrap)
    app.directive('auto-focus', vAutoFocus)
    app.directive('ripple', vRipple)

    // Global components
    Object.entries({
      // UI
      Button,
      Input,
      Label,
      Card,
      CardHeader,
      CardTitle,
      CardDescription,
      CardContent,
      CardFooter,
      Badge,
      Avatar,
      AvatarImage,
      AvatarFallback,
      DropdownMenu,
      DropdownMenuTrigger,
      DropdownMenuContent,
      DropdownMenuItem,
      Tabs,
      TabsList,
      TabsTrigger,
      TabsContent,
      Table,
      TableHeader,
      TableBody,
      TableRow,
      TableHead,
      TableCell,
      TableCaption,
      Dialog,
      DialogTrigger,
      DialogContent,
      DialogHeader,
      DialogTitle,
      DialogDescription,
      DialogFooter,
      Sheet,
      SheetTrigger,
      SheetContent,
      SheetHeader,
      SheetTitle,
      SheetDescription,
      SheetFooter,
      Tooltip,
      TooltipTrigger,
      TooltipContent,
      Alert,
      AlertTitle,
      AlertDescription,
      Separator,
      Skeleton,
      Progress,
      Toggle,
      ScrollArea,
      Accordion,
      AccordionItem,
      AccordionTrigger,
      AccordionContent,
      Popover,
      PopoverTrigger,
      PopoverContent,
      HoverCard,
      HoverCardTrigger,
      HoverCardContent,
      Menubar,
      MenubarMenu,
      MenubarTrigger,
      MenubarContent,
      MenubarItem,
      MenubarSeparator,
      Command,
      CommandInput,
      CommandList,
      CommandItem,
      CommandGroup,
      CommandSeparator,
      Select,
      SelectTrigger,
      SelectValue,
      SelectContent,
      SelectItem,
      SelectGroup,
      SelectLabel,
      Checkbox,
      RadioGroup,
      RadioGroupItem,
      Switch,
      Slider,
      Textarea,
      Form,
      FormField,
      FormItem,
      FormLabel,
      FormControl,
      FormDescription,
      FormMessage,
      Calendar,
      DatePicker,
      Pagination,
      Breadcrumbs,
      Stepper,
      Rating,
      FileUpload,
      ColorPicker,
      TimePicker,
      TagInput,
      MarkdownEditor,
      CodeEditor,
      ImageCropper,
      DataTable,
      Chart,
      StatCard,
      EmptyState,
      LoadingState,
      ErrorState,

      // Layout
      AppLayout,
      AuthLayout,
      CourseLayout,
      DashboardLayout,
      AdminLayout,
      Header,
      Sidebar,
      Footer,
      MobileNav,
      UserMenu,
      NotificationBell,
      SearchCommand,
      LanguageSwitcher,
      ThemeToggle,

      // Course
      CourseCard,
      CourseGrid,
      CourseHero,
      CourseProgress,
      CourseOutline,
      CourseReviews,
      CourseInstructor,
      CoursePricing,
      CourseMetadata,
      EnrollmentButton,
      LearningPath,
      ModuleProgress,
      LessonPlayer,
      DiscussionThread,
      NotesPanel,
      BookmarkButton,
      CertificatePreview,

      // Dashboard
      DashboardStats,
      RecentActivity,
      UpcomingDeadlines,
      ContinueLearning,
      RecommendedCourses,
      LearningStreak,
      SkillProgress,
      CalendarWidget,
      QuickActions,

      // Admin
      AdminSidebar,
      AdminHeader,
      AdminStats,
      UserTable,
      CourseTable,
      EnrollmentTable,
      AnalyticsChart,
      SettingsPanel,
      RoleManager,
      TenantSwitcher,
      AuditLog,
      SystemHealth,
    }).forEach(([name, component]) => {
      app.component(name, component)
    })

    // Provide global composables
    app.provide('theme', useTheme())
    app.provide('auth', useAuth())
    app.provide('toast', useToast())
    app.provide('modal', useModal())
    app.provide('clipboard', useClipboard())
    app.provide('debounce', useDebounce())
    app.provide('localStorage', useLocalStorage())
    app.provide('mediaQuery', useMediaQuery())
    app.provide('intersectionObserver', useIntersectionObserver())
    app.provide('keyboardShortcuts', useKeyboardShortcuts())
    app.provide('inertia', useInertia())
    app.provide('permissions', usePermissions())
    app.provide('featureFlags', useFeatureFlags())

    // Mount app
    app.mount(el as HTMLElement)

    // Initialize theme
    useTheme().init()

    // Register service worker for PWA
    if ('serviceWorker' in navigator && import.meta.env.PROD) {
      navigator.serviceWorker.register('/sw.js').catch(() => {
        // SW registration failed
      })
    }
  },
  progress: {
    color: '#0c87f2',
    showSpinner: false,
  },
})

// Global error handler
window.addEventListener('error', (event) => {
  console.error('Global error:', event.error)
  // Send to error tracking service
})

window.addEventListener('unhandledrejection', (event) => {
  console.error('Unhandled rejection:', event.reason)
  // Send to error tracking service
})

// Performance monitoring
if (import.meta.env.PROD) {
  // Web Vitals
  import('web-vitals').then(({ onCLS, onFID, onFCP, onLCP, onTTFB }) => {
    onCLS(console.log)
    onFID(console.log)
    onFCP(console.log)
    onLCP(console.log)
    onTTFB(console.log)
  })
}