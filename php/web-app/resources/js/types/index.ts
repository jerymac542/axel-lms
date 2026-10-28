// Shared type definitions for Axel LMS frontend

export interface User {
  id: string
  email: string
  name: string
  avatar_url?: string
  role: string
  role_display: string
  email_verified: boolean
  phone_verified: boolean
  locale: string
  timezone: string
  last_login_at?: string
  created_at: string
  updated_at: string
  stats?: UserStats
  permissions?: string[]
  roles?: string[]
}

export interface UserStats {
  active_courses: number
  completed_courses: number
  badges_earned: number
  certificates_earned: number
  bookmarks_count: number
  unread_messages: number
  unread_discussions: number
  learning_streak: number
  total_hours: number
}

export interface Course {
  id: string
  organization_id: string
  title: string
  description: string
  short_description?: string
  status: 'draft' | 'published' | 'archived'
  visibility: 'private' | 'organization' | 'tenant' | 'public'
  language: string
  thumbnail_url?: string
  banner_url?: string
  settings: CourseSettings
  metadata: CourseMetadata
  created_at: string
  updated_at: string
  published_at?: string
  created_by: string
  instructor?: User
  stats?: CourseStats
  enrollment?: Enrollment
  modules?: Module[]
}

export interface CourseSettings {
  self_enrollment: boolean
  require_approval: boolean
  show_progress: boolean
  allow_discussions: boolean
  allow_annotations: boolean
  certificate_enabled: boolean
  certificate_template_id?: string
  prerequisite_course_ids: string[]
  enrollment_start?: string
  enrollment_end?: string
  max_enrollment?: number
  grading: GradingSettings
  completion: CompletionSettings
  access: AccessSettings
}

export interface GradingSettings {
  enabled: boolean
  scheme: 'percentage' | 'letter' | 'pass_fail' | 'custom'
  show_grades_to_students: boolean
  show_rankings: boolean
  gradebook: GradebookSettings
}

export interface GradebookSettings {
  enabled: boolean
  auto_calculate: boolean
  categories: GradeCategory[]
}

export interface GradeCategory {
  id: string
  name: string
  weight: number
  drop_lowest: boolean
  drop_count: number
}

export interface CompletionSettings {
  enabled: boolean
  criteria: 'all_activities' | 'score_threshold' | 'manual' | 'custom'
  required_score?: number
  required_activities?: string[]
}

export interface AccessSettings {
  allowed_groups: string[]
  denied_groups: string[]
  allowed_roles: string[]
  denied_roles: string[]
  require_enrollment: boolean
  guest_access: boolean
}

export interface CourseMetadata {
  subject?: string
  level?: string
  tags: string[]
  estimated_duration?: string
  learning_objectives: string[]
  prerequisites: string[]
  custom: Record<string, string>
}

export interface CourseStats {
  total_enrollments: number
  active_students: number
  completed_students: number
  average_score: number
  completion_rate: number
  average_time_spent: number
}

export interface Module {
  id: string
  course_id: string
  parent_id?: string
  title: string
  type: 'section' | 'unit' | 'lesson' | 'activity'
  position: number
  published: boolean
  content?: ModuleContent
  children?: Module[]
  stats?: ModuleStats
}

export interface ModuleContent {
  content_type: string
  content_id: string
  url?: string
  settings: Record<string, string>
}

export interface ModuleStats {
  completions: number
  average_score: number
  average_time: number
}

export interface Enrollment {
  id: string
  course_id: string
  user_id: string
  organization_id: string
  role: 'student' | 'teaching_assistant' | 'instructor' | 'course_designer' | 'observer' | 'auditor'
  status: 'pending' | 'active' | 'completed' | 'dropped' | 'suspended' | 'expired' | 'waitlisted'
  source: 'self' | 'instructor' | 'admin' | 'sis' | 'lti' | 'api' | 'bulk'
  enrolled_by: string
  enrolled_at: string
  started_at?: string
  completed_at?: string
  dropped_at?: string
  drop_reason?: string
  progress: number
  score?: number
  grade?: string
  custom_fields: Record<string, string>
  created_at: string
  updated_at: string
}

export interface Assignment {
  id: string
  course_id: string
  group_id?: string
  title: string
  description: string
  type: 'homework' | 'quiz' | 'exam' | 'essay' | 'project' | 'discussion' | 'peer_review' | 'external_tool' | 'survey' | 'attendance'
  status: 'draft' | 'published' | 'closed'
  settings: AssignmentSettings
  grading: GradingSettings
  attachment_ids: string[]
  available_at?: string
  due_at?: string
  lock_at?: string
  created_at: string
  updated_at: string
  created_by: string
  submissions?: Submission[]
}

export interface AssignmentSettings {
  max_attempts: number
  allow_late_submission: boolean
  late_penalty: number
  require_lock: boolean
  show_correct_answers: boolean
  show_peer_reviews: boolean
  anonymous_grading: boolean
  group_submission: boolean
  group_size: number
  allowed_file_types: string[]
  max_file_size: number
  max_files: number
  plagiarism_check: boolean
  plagiarism_tool?: string
  custom: Record<string, string>
}

export interface Submission {
  id: string
  assignment_id: string
  user_id: string
  group_id?: string
  status: 'draft' | 'submitted' | 'late' | 'graded' | 'returned' | 'excused' | 'missing'
  content: SubmissionContent
  attachments: SubmissionAttachment[]
  grading?: GradingResult
  attempt: number
  submitted_at?: string
  graded_at?: string
  graded_by?: string
  created_at: string
  updated_at: string
}

export interface SubmissionContent {
  text?: string
  html?: string
  file_ids: string[]
  data: Record<string, string>
}

export interface SubmissionAttachment {
  id: string
  filename: string
  mime_type: string
  size: number
  url: string
  thumbnail_url?: string
}

export interface GradingResult {
  score: number
  grade?: string
  feedback?: string
  criterion_scores: CriterionScore[]
  auto_graded: boolean
  auto_grade_confidence: number
  details: Record<string, string>
}

export interface CriterionScore {
  criterion_id: string
  criterion_name: string
  score: number
  max_score: number
  feedback?: string
}

export interface Rubric {
  id: string
  course_id: string
  title: string
  description: string
  type: 'analytic' | 'holistic' | 'single_point'
  criteria: Criterion[]
  total_points: number
  hide_scores: boolean
  created_at: string
  updated_at: string
  created_by: string
}

export interface Criterion {
  id: string
  name: string
  description: string
  points: number
  order: number
  ratings: Rating[]
}

export interface Rating {
  id: string
  name: string
  description: string
  points: number
}

export interface Media {
  id: string
  tenant_id: string
  organization_id: string
  uploaded_by: string
  filename: string
  original_filename: string
  mime_type: string
  size: number
  checksum: string
  status: 'uploading' | 'uploaded' | 'processing' | 'processed' | 'failed' | 'deleted'
  metadata: MediaMetadata
  variants: MediaVariant[]
  storage_key: string
  storage_bucket: string
  created_at: string
  updated_at: string
  processed_at?: string
  tags: Record<string, string>
}

export interface MediaMetadata {
  width?: number
  height?: number
  color_space?: string
  has_alpha?: boolean
  frames?: number
  duration?: number
  video_codec?: string
  audio_codec?: string
  video_bitrate?: number
  audio_bitrate?: number
  frame_rate?: number
  aspect_ratio?: string
  page_count?: number
  author?: string
  creator?: string
  producer?: string
  creation_date?: string
  modification_date?: string
  custom: Record<string, string>
}

export interface MediaVariant {
  id: string
  media_id: string
  name: string
  mime_type: string
  size: number
  storage_key: string
  url: string
  metadata: MediaMetadata
  created_at: string
}

export interface DiscussionThread {
  id: string
  course_id: string
  module_id?: string
  lesson_id?: string
  title: string
  content: string
  author_id: string
  author?: User
  type: 'question' | 'discussion' | 'announcement' | 'poll'
  status: 'open' | 'closed' | 'pinned' | 'locked'
  tags: string[]
  replies_count: number
  participants_count: number
  last_reply_at?: string
  last_reply_by?: string
  created_at: string
  updated_at: string
  replies?: DiscussionReply[]
}

export interface DiscussionReply {
  id: string
  thread_id: string
  parent_id?: string
  author_id: string
  author?: User
  content: string
  created_at: string
  updated_at: string
  children?: DiscussionReply[]
}

export interface Notification {
  id: string
  user_id: string
  type: 'info' | 'success' | 'warning' | 'error' | 'assignment' | 'grade' | 'enrollment' | 'discussion' | 'mention' | 'system' | 'security' | 'marketing'
  title: string
  body: string
  action_url?: string
  action_text?: string
  icon_url?: string
  data: Record<string, string>
  priority: 'low' | 'normal' | 'high' | 'urgent'
  read: boolean
  created_at: string
  read_at?: string
  expires_at?: string
}

export interface Organization {
  id: string
  tenant_id: string
  name: string
  display_name: string
  description?: string
  logo_url?: string
  domain?: string
  settings: OrganizationSettings
  member_count: number
  course_count: number
  created_at: string
  updated_at: string
  deleted_at?: string
}

export interface OrganizationSettings {
  default_role: string
  allow_self_enrollment: boolean
  require_approval: boolean
  visibility: 'private' | 'members' | 'tenant' | 'public'
  branding?: any
  integrations: any
}

export interface Tenant {
  id: string
  name: string
  display_name: string
  domain?: string
  subdomain?: string
  status: 'active' | 'suspended' | 'pending' | 'deleted'
  plan: 'free' | 'starter' | 'professional' | 'enterprise' | 'custom'
  settings: TenantSettings
  features: FeatureFlags
  organization_ids: string[]
  created_at: string
  updated_at: string
  deleted_at?: string
}

export interface TenantSettings {
  default_locale: string
  supported_locales: string[]
  timezone: string
  theme: ThemeSettings
  branding: BrandingSettings
  security: SecuritySettings
  integrations: IntegrationSettings
  limits: TenantLimits
}

export interface ThemeSettings {
  primary_color: string
  secondary_color: string
  logo_url?: string
  favicon_url?: string
  custom_css?: string
}

export interface BrandingSettings {
  name: string
  description?: string
  support_email?: string
  support_url?: string
  terms_url?: string
  privacy_url?: string
}

export interface SecuritySettings {
  mfa_required: boolean
  password_policy: PasswordPolicySettings
  session_timeout: number
  allowed_domains: string[]
  ip_allowlist: string[]
  ip_blocklist: string[]
}

export interface PasswordPolicySettings {
  min_length: number
  require_uppercase: boolean
  require_lowercase: boolean
  require_numbers: boolean
  require_symbols: boolean
  max_age_days?: number
  history_count: number
}

export interface IntegrationSettings {
  lti_enabled: boolean
  scorm_enabled: boolean
  xapi_enabled: boolean
  caliper_enabled: boolean
  oneroster_enabled: boolean
  sso_providers: SSOProviderConfig[]
}

export interface SSOProviderConfig {
  provider: string
  enabled: boolean
  config: Record<string, string>
}

export interface TenantLimits {
  max_users?: number
  max_courses?: number
  max_storage_gb?: number
  max_bandwidth_gb?: number
  max_api_calls_per_hour?: number
  max_concurrent_sessions?: number
}

export interface FeatureFlags {
  flags: Record<string, boolean>
  rollout: Record<string, RolloutConfig>
}

export interface RolloutConfig {
  percentage: number
  groups: string[]
  tenants: string[]
  users: string[]
}

export interface PaginationRequest {
  page: number
  per_page: number
}

export interface PaginationResponse {
  current_page: number
  per_page: number
  total_items: number
  total_pages: number
  has_next: boolean
  has_prev: boolean
}

export interface SortRequest {
  field: string
  direction: 'asc' | 'desc'
}

export interface FilterRequest {
  field: string
  operator: string
  value: any
}

export interface ListRequest {
  pagination?: PaginationRequest
  sort?: SortRequest
  filters?: FilterRequest[]
  search?: string
  include?: string[]
}

export interface ListResponse<T> {
  items: T[]
  pagination: PaginationResponse
}

export interface ErrorResponse {
  error: {
    code: string
    message: string
    correlation_id?: string
    event_id?: string
    context?: Record<string, string>
  }
}

export interface HealthCheckResponse {
  status: string
  version: string
  timestamp: string
  checks: Record<string, { status: string; message?: string }>
}

// Auth types
export interface AuthState {
  user: User | null
  isAuthenticated: boolean
  isLoading: boolean
  permissions: string[]
  roles: string[]
}

export interface LoginCredentials {
  email: string
  password: string
  mfa_code?: string
  backup_code?: string
  remember_me?: boolean
}

export interface RegisterData {
  name: string
  email: string
  password: string
  locale?: string
  timezone?: string
}

export interface TokenResponse {
  access_token: string
  refresh_token: string
  id_token?: string
  token_type: string
  expires_in: number
  refresh_expires_in: number
  scope: string[]
}

export interface MfaChallenge {
  method: 'totp' | 'webauthn' | 'backup_code'
  challenge: string
}

// API types
export interface ApiResponse<T> {
  data: T
  meta?: {
    timestamp: string
    request_id: string
    version: string
  }
}

export interface ApiError {
  code: string
  message: string
  correlation_id?: string
  event_id?: string
  context?: Record<string, string>
}

// UI types
export interface Toast {
  id: string
  type: 'success' | 'error' | 'warning' | 'info' | 'loading'
  title: string
  description?: string
  duration?: number
  dismissible?: boolean
  action?: {
    label: string
    onClick: () => void
  }
}

export interface Modal {
  id: string
  title: string
  content: string
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full'
  closable?: boolean
  onClose?: () => void
  onConfirm?: () => void
  confirmText?: string
  cancelText?: string
}

export interface DropdownItem {
  label: string
  icon?: string
  onClick: () => void
  disabled?: boolean
  divider?: boolean
  danger?: boolean
}

export interface BreadcrumbItem {
  label: string
  href?: string
  icon?: string
}

export interface TableColumn<T> {
  key: string
  label: string
  sortable?: boolean
  width?: string
  align?: 'left' | 'center' | 'right'
  render?: (item: T, value: any) => string | HTMLElement
}

export interface TableAction<T> {
  label: string
  icon?: string
  onClick: (item: T) => void
  disabled?: (item: T) => boolean
  danger?: boolean
}

export interface FormField {
  name: string
  label: string
  type: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url' | 'textarea' | 'select' | 'radio' | 'checkbox' | 'switch' | 'date' | 'datetime' | 'file' | 'hidden'
  placeholder?: string
  required?: boolean
  disabled?: boolean
  readonly?: boolean
  options?: { value: string; label: string }[]
  validation?: Record<string, any>
  helpText?: string
  dependsOn?: string
  showWhen?: (values: Record<string, any>) => boolean
}

export interface NavItem {
  name: string
  href: string
  icon?: string
  badge?: number | string
  children?: NavItem[]
  roles?: string[]
  permissions?: string[]
  exact?: boolean
}

// WebSocket types
export interface RealtimeEvent {
  type: string
  payload: any
  timestamp: string
}

export interface Presence {
  user_id: string
  status: 'online' | 'away' | 'busy' | 'offline' | 'invisible'
  status_message?: string
  activity?: string
  activity_id?: string
  last_seen: string
}

export interface ChannelMessage {
  id: string
  channel_id: string
  user_id: string
  content: string
  type: 'text' | 'html' | 'markdown' | 'system' | 'file' | 'image' | 'video' | 'audio' | 'embed' | 'poll' | 'code' | 'announcement'
  metadata: Record<string, string>
  reactions: Reaction[]
  reference?: MessageReference
  created_at: string
  updated_at: string
  deleted_at?: string
  edited: boolean
  system: boolean
}

export interface Reaction {
  emoji: string
  user_ids: string[]
  count: number
}

export interface MessageReference {
  message_id: string
  channel_id: string
  content_preview: string
}

// Feature flags
export interface FeatureFlag {
  name: string
  enabled: boolean
  rollout?: RolloutConfig
}

// Theme
export type Theme = 'light' | 'dark' | 'system'

// Locale
export interface Locale {
  code: string
  name: string
  nativeName: string
  rtl: boolean
}

// Analytics
export interface AnalyticsEvent {
  event: string
  properties: Record<string, any>
  userId?: string
  timestamp: string
}

export interface PageViewEvent extends AnalyticsEvent {
  event: 'page_view'
  properties: {
    path: string
    title: string
    referrer?: string
  }
}

export interface FeatureUsedEvent extends AnalyticsEvent {
  event: 'feature_used'
  properties: {
    feature: string
    properties?: Record<string, any>
  }
}