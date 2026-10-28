import './bootstrap'
import '../css/app.css'

import { createSSRApp, h, defineComponent } from 'vue'
import { createInertiaApp, Link, Head } from '@inertiajs/vue3'
import { createPinia } from 'pinia'
import { VueQueryPlugin } from '@tanstack/vue-query'
import { createHead } from '@vueuse/head'

// Import all global components (same as client)
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

    const app = createSSRApp({ render: () => h(App, props) })
      .use(plugin)
      .use(pinia)
      .use(head)
      .use(VueQueryPlugin, {
        queryClientConfig: {
          defaultOptions: {
            queries: {
              staleTime: 1000 * 60 * 5,
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

      DashboardStats,
      RecentActivity,
      UpcomingDeadlines,
      ContinueLearning,
      RecommendedCourses,
      LearningStreak,
      SkillProgress,
      CalendarWidget,
      QuickActions,

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

    return { app, head }
  },
  progress: {
    color: '#0c87f2',
    showSpinner: false,
  },
})