<script setup lang="ts">
import { ref, computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'
import { Switch } from '@/components/ui/switch'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Textarea } from '@/components/ui/textarea'
import { Separator } from '@/components/ui/separator'
import { 
  SaveIcon, SettingsIcon, GlobeIcon, LockIcon, MailIcon,
  BellIcon, ShieldIcon, PaletteIcon, DatabaseIcon,
  UsersIcon, KeyIcon, CloudIcon, RefreshCwIcon,
  CheckCircleIcon, AlertCircleIcon
} from '@/components/ui/icons'
import { useToast } from '@/composables/useToast'

const toast = useToast()

const activeTab = ref('general')

const settings = ref({
  general: {
    siteName: 'Axel LMS',
    siteDescription: 'The enterprise learning platform for modern organizations',
    siteUrl: 'https://axel.example.com',
    adminEmail: 'admin@axel.example.com',
    defaultLanguage: 'en',
    defaultTimezone: 'UTC',
    maintenanceMode: false,
    registrationEnabled: true,
    emailVerificationRequired: true,
  },
  branding: {
    logoUrl: '/images/logos/axel-lms-logo.png',
    faviconUrl: '/images/logos/axel-favicon.ico',
    primaryColor: '#0c87f2',
    secondaryColor: '#1e40af',
    customCss: '',
    favicon: null,
  },
  security: {
    mfaRequired: true,
    passwordMinLength: 12,
    passwordRequireUppercase: true,
    passwordRequireLowercase: true,
    passwordRequireNumbers: true,
    passwordRequireSymbols: true,
    passwordMaxAge: 90,
    passwordHistoryCount: 5,
    sessionTimeout: 2592000,
    maxLoginAttempts: 5,
    lockoutDuration: 900,
    allowedDomains: '',
    ipAllowlist: '',
    ipBlocklist: '',
  },
  email: {
    provider: 'smtp',
    fromAddress: 'noreply@axel.example.com',
    fromName: 'Axel LMS',
    replyTo: 'support@axel.example.com',
    smtpHost: 'smtp.example.com',
    smtpPort: 587,
    smtpUsername: 'smtp_user',
    smtpPassword: '********',
    smtpEncryption: 'tls',
    templatesPath: 'templates/email',
  },
  integrations: {
    ltiEnabled: true,
    scormEnabled: true,
    xapiEnabled: true,
    caliperEnabled: true,
    onerosterEnabled: false,
    ssoProviders: [
      { provider: 'google', enabled: true, config: {} },
      { provider: 'microsoft', enabled: true, config: {} },
      { provider: 'saml', enabled: false, config: {} },
      { provider: 'ldap', enabled: false, config: {} },
    ],
  },
  storage: {
    provider: 's3',
    region: 'us-east-1',
    bucket: 'axel-lms',
    endpoint: 'https://s3.amazonaws.com',
    accessKey: 'AKIA...',
    secretKey: '********',
    cdnUrl: 'https://cdn.axel.example.com',
    maxUploadSize: 5368709120,
  },
  advanced: {
    maintenanceMode: false,
    maintenanceMessage: 'System is under maintenance. Please check back later.',
    debugMode: false,
    logLevel: 'info',
    cacheEnabled: true,
    cacheTtl: 3600,
    rateLimitEnabled: true,
    rateLimitRequests: 1000,
    rateLimitWindow: 60,
    apiEnabled: true,
    apiRateLimit: 1000,
    webhooksEnabled: true,
    webhookSecret: '********',
  },
})

const activeTab = ref('general')

const tabs = [
  { id: 'general', label: 'General', icon: SettingsIcon },
  { id: 'branding', label: 'Branding', icon: PaletteIcon },
  { id: 'security', label: 'Security', icon: ShieldIcon },
  { id: 'email', label: 'Email', icon: MailIcon },
  { id: 'integrations', label: 'Integrations', icon: GlobeIcon },
  { id: 'storage', label: 'Storage', icon: CloudIcon },
  { id: 'advanced', label: 'Advanced', icon: SettingsIcon },
]

const saveSettings = async (tab: string) => {
  toast.success(`${tab.charAt(0).toUpperCase() + tab.slice(1)} settings saved`)
}

const resetSettings = (tab: string) => {
  if (confirm(`Reset ${tab} settings to defaults?`)) {
    toast.success(`${tab.charAt(0).toUpperCase() + tab.slice(1)} settings reset`)
  }
}

const testEmail = async () => {
  toast.success('Test email sent')
}

const testStorage = async () => {
  toast.success('Storage connection successful')
}
</script>

<template>
  <Card class="h-full">
    <CardHeader class="pb-3">
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <SettingsIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Platform Settings</CardTitle>
            <CardDescription>Configure platform-wide settings and preferences</CardDescription>
          </div>
        </div>
        
        <div class="flex items-center gap-2">
          <Button variant="outline" @click="resetSettings(activeTab)" class="gap-1">
            <RefreshCwIcon class="h-4 w-4" />
            Reset
          </Button>
          <Button @click="saveSettings(activeTab)" class="gap-1">
            <SaveIcon class="h-4 w-4" />
            Save Changes
          </Button>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <Tabs default-value="general" class="w-full">
          <TabsList class="grid w-full grid-cols-7">
            <TabsTrigger v-for="tab in tabs" :key="tab.id" :value="tab.id">
              <component :is="tab.icon" class="h-4 w-4 mr-2" />
              {{ tab.label }}
            </TabsTrigger>
          </TabsList>

          <!-- General Settings -->
          <TabsContent value="general" class="mt-6 space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Basic Information</CardTitle>
                <CardDescription>Basic platform information and settings</CardDescription>
              </CardHeader>
              <CardContent class="space-y-6">
                <div class="grid gap-4 sm:grid-cols-2">
                  <div class="sm:col-span-2">
                    <Label for="siteName">Site Name</Label>
                    <Input id="siteName" v-model="settings.general.siteName" placeholder="Axel LMS" class="mt-1" />
                  </div>
                  <div class="sm:col-span-2">
                    <Label for="siteDescription">Site Description</Label>
                    <Textarea id="siteDescription" v-model="settings.general.siteDescription" placeholder="The enterprise learning platform..." class="mt-1 min-h-[80px]" />
                  </div>
                  <div>
                    <Label for="siteUrl">Site URL</Label>
                    <Input id="siteUrl" v-model="settings.general.siteUrl" type="url" placeholder="https://example.com" class="mt-1" />
                  </div>
                  <div>
                    <Label for="adminEmail">Admin Email</Label>
                    <Input id="adminEmail" v-model="settings.general.adminEmail" type="email" placeholder="admin@example.com" class="mt-1" />
                  </div>
                  <div>
                    <Label for="defaultLanguage">Default Language</Label>
                    <Select v-model="settings.general.defaultLanguage" class="mt-1">
                      <SelectTrigger>
                        <SelectValue placeholder="Select language" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="en">English</SelectItem>
                        <SelectItem value="es">Spanish</SelectItem>
                        <SelectItem value="fr">French</SelectItem>
                        <SelectItem value="de">German</SelectItem>
                        <SelectItem value="zh">Chinese</SelectItem>
                        <SelectItem value="ja">Japanese</SelectItem>
                        <SelectItem value="pt">Portuguese</SelectItem>
                        <SelectItem value="ar">Arabic</SelectItem>
                        <SelectItem value="hi">Hindi</SelectItem>
                        <SelectItem value="ru">Russian</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <Label for="defaultTimezone">Default Timezone</Label>
                    <Select v-model="settings.general.defaultTimezone" class="mt-1">
                      <SelectTrigger>
                        <SelectValue placeholder="Select timezone" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="UTC">UTC</SelectItem>
                        <SelectItem value="America/New_York">Eastern Time</SelectItem>
                        <SelectItem value="America/Chicago">Central Time</SelectItem>
                        <SelectItem value="America/Denver">Mountain Time</SelectItem>
                        <SelectItem value="America/Los_Angeles">Pacific Time</SelectItem>
                        <SelectItem value="Europe/London">London</SelectItem>
                        <SelectItem value="Europe/Paris">Paris</SelectItem>
                        <SelectItem value="Asia/Tokyo">Tokyo</SelectItem>
                        <SelectItem value="Asia/Shanghai">Shanghai</SelectItem>
                        <SelectItem value="Asia/Dubai">Dubai</SelectItem>
                        <SelectItem value="Asia/Singapore">Singapore</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>
                
                <Separator class="my-4" />
                
                <div class="space-y-4">
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Maintenance Mode</Label>
                      <p class="text-sm text-muted-foreground">Put the platform in maintenance mode</p>
                    </div>
                    <Switch v-model="settings.general.maintenanceMode" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>User Registration</Label>
                      <p class="text-sm text-muted-foreground">Allow new user registrations</p>
                    </div>
                    <Switch v-model="settings.general.registrationEnabled" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Email Verification Required</Label>
                      <p class="text-sm text-muted-foreground">Require email verification for new accounts</p>
                    </div>
                    <Switch v-model="settings.general.emailVerificationRequired" />
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <!-- Branding Settings -->
          <TabsContent value="branding" class="mt-6 space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Brand Identity</CardTitle>
                <CardDescription>Customize the look and feel of your platform</CardDescription>
              </CardHeader>
              <CardContent class="space-y-6">
                <div class="grid gap-4 sm:grid-cols-2">
                  <div>
                    <Label for="logoUrl">Logo URL</Label>
                    <Input id="logoUrl" v-model="settings.branding.logoUrl" placeholder="/images/logos/logo.png" class="mt-1" />
                  </div>
                  <div>
                    <Label for="faviconUrl">Favicon URL</Label>
                    <Input id="faviconUrl" v-model="settings.branding.faviconUrl" placeholder="/images/logos/favicon.ico" class="mt-1" />
                  </div>
                </div>
                
                <div class="grid gap-4 sm:grid-cols-2">
                  <div>
                    <Label for="primaryColor">Primary Color</Label>
                    <Input id="primaryColor" v-model="settings.branding.primaryColor" type="color" class="mt-1 h-10 w-full cursor-pointer" />
                  </div>
                  <div>
                    <Label for="secondaryColor">Secondary Color</Label>
                    <Input id="secondaryColor" v-model="settings.branding.secondaryColor" type="color" class="mt-1 h-10 w-full cursor-pointer" />
                  </div>
                </div>
                
                <div>
                  <Label for="customCss">Custom CSS</Label>
                  <Textarea id="customCss" v-model="settings.branding.customCss" placeholder="/* Custom CSS here */" class="mt-1 min-h-[200px] font-mono text-sm" />
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <!-- Security Settings -->
          <TabsContent value="security" class="mt-6 space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Authentication & Access</CardTitle>
                <CardDescription>Configure authentication and access control settings</CardDescription>
              </CardHeader>
              <CardContent class="space-y-6">
                <div class="space-y-4">
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Multi-Factor Authentication Required</Label>
                      <p class="text-sm text-muted-foreground">Require MFA for all users</p>
                    </div>
                    <Switch v-model="settings.security.mfaRequired" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Max Login Attempts</Label>
                      <p class="text-sm text-muted-foreground">Number of failed attempts before lockout</p>
                    </div>
                    <Input type="number" v-model.number="settings.security.maxLoginAttempts" min="1" max="10" class="w-24 mt-1" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Lockout Duration (seconds)</Label>
                      <p class="text-sm text-muted-foreground">Time before locked out users can try again</p>
                    </div>
                    <Input type="number" v-model.number="settings.security.lockoutDuration" min="60" max="86400" class="w-32 mt-1" />
                  </div>
                </div>
                
                <Separator class="my-4" />
                
                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                  <div>
                    <Label for="passwordMinLength">Min Password Length</Label>
                    <Input id="passwordMinLength" type="number" v-model.number="settings.security.passwordMinLength" min="8" max="128" class="mt-1" />
                  </div>
                  <div>
                    <Label for="passwordMaxAge">Password Max Age (days)</Label>
                    <Input id="passwordMaxAge" type="number" v-model.number="settings.security.passwordMaxAge" min="0" max="365" class="mt-1" />
                  </div>
                  <div>
                    <Label for="passwordHistoryCount">Password History Count</Label>
                    <Input id="passwordHistoryCount" type="number" v-model.number="settings.security.passwordHistoryCount" min="0" max="20" class="mt-1" />
                  </div>
                  <div>
                    <Label for="sessionTimeout">Session Timeout (seconds)</Label>
                    <Input id="sessionTimeout" type="number" v-model.number="settings.security.sessionTimeout" min="300" max="2592000" class="mt-1" />
                  </div>
                </div>
                
                <div class="space-y-4">
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Require Uppercase</Label>
                    </div>
                    <Switch v-model="settings.security.passwordRequireUppercase" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Require Lowercase</Label>
                    </div>
                    <Switch v-model="settings.security.passwordRequireLowercase" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Require Numbers</Label>
                    </div>
                    <Switch v-model="settings.security.passwordRequireNumbers" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Require Symbols</Label>
                    </div>
                    <Switch v-model="settings.security.passwordRequireSymbols" />
                  </div>
                </div>
                
                <Separator class="my-4" />
                
                <div class="grid gap-4 sm:grid-cols-2">
                  <div>
                    <Label for="allowedDomains">Allowed Email Domains (comma-separated)</Label>
                    <Input id="allowedDomains" v-model="settings.security.allowedDomains" placeholder="example.com, company.org" class="mt-1" />
                    <p class="text-xs text-muted-foreground mt-1">Leave empty to allow all domains</p>
                  </div>
                  <div>
                    <Label for="ipAllowlist">IP Allowlist (CIDR, comma-separated)</Label>
                    <Input id="ipAllowlist" v-model="settings.security.ipAllowlist" placeholder="192.168.1.0/24, 10.0.0.0/8" class="mt-1" />
                    <p class="text-xs text-muted-foreground mt-1">Leave empty to allow all IPs</p>
                  </div>
                  <div>
                    <Label for="ipBlocklist">IP Blocklist (CIDR, comma-separated)</Label>
                    <Input id="ipBlocklist" v-model="settings.security.ipBlocklist" placeholder="192.168.100.0/24" class="mt-1" />
                    <p class="text-xs text-muted-foreground mt-1">Block specific IP ranges</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <!-- Email Settings -->
          <TabsContent value="email" class="mt-6 space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Email Configuration</CardTitle>
                <CardDescription>Configure email delivery settings</CardDescription>
              </CardHeader>
              <CardContent class="space-y-6">
                <div class="grid gap-4 sm:grid-cols-2">
                  <div>
                    <Label for="emailProvider">Email Provider</Label>
                    <Select v-model="settings.email.provider" class="mt-1">
                      <SelectTrigger>
                        <SelectValue placeholder="Select provider" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="smtp">SMTP</SelectItem>
                        <SelectItem value="sendgrid">SendGrid</SelectItem>
                        <SelectItem value="mailgun">Mailgun</SelectItem>
                        <SelectItem value="postmark">Postmark</SelectItem>
                        <SelectItem value="ses">Amazon SES</SelectItem>
                        <SelectItem value="resend">Resend</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <Label for="fromAddress">From Address</Label>
                    <Input id="fromAddress" v-model="settings.email.fromAddress" type="email" placeholder="noreply@example.com" class="mt-1" />
                  </div>
                  <div>
                    <Label for="fromName">From Name</Label>
                    <Input id="fromName" v-model="settings.email.fromName" placeholder="Axel LMS" class="mt-1" />
                  </div>
                  <div>
                    <Label for="replyTo">Reply-To Address</Label>
                    <Input id="replyTo" v-model="settings.email.replyTo" type="email" placeholder="support@example.com" class="mt-1" />
                  </div>
                </div>
                
                <Separator class="my-4" />
                
                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                  <div>
                    <Label for="smtpHost">SMTP Host</Label>
                    <Input id="smtpHost" v-model="settings.email.smtpHost" placeholder="smtp.example.com" class="mt-1" />
                  </div>
                  <div>
                    <Label for="smtpPort">SMTP Port</Label>
                    <Input id="smtpPort" type="number" v-model.number="settings.email.smtpPort" class="mt-1" />
                  </div>
                  <div>
                    <Label for="smtpUsername">SMTP Username</Label>
                    <Input id="smtpUsername" v-model="settings.email.smtpUsername" placeholder="smtp_user" class="mt-1" />
                  </div>
                  <div>
                    <Label for="smtpPassword">SMTP Password</Label>
                    <Input id="smtpPassword" type="password" v-model="settings.email.smtpPassword" placeholder="••••••••" class="mt-1" />
                  </div>
                </div>
                
                <div class="flex items-center justify-between">
                  <div>
                    <Label for="smtpEncryption">Encryption</Label>
                    <Select v-model="settings.email.smtpEncryption" class="mt-1 w-48">
                      <SelectTrigger>
                        <SelectValue placeholder="Encryption" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="none">None</SelectItem>
                        <SelectItem value="tls">TLS</SelectItem>
                        <SelectItem value="ssl">SSL</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <Button variant="outline" @click="testEmail" class="gap-2 self-end">
                    <SendIcon class="h-4 w-4" />
                    Send Test Email
                  </Button>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <!-- Integrations Settings -->
          <TabsContent value="integrations" class="mt-6 space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Platform Integrations</CardTitle>
                <CardDescription>Enable and configure third-party integrations</CardDescription>
              </CardHeader>
              <CardContent class="space-y-6">
                <div class="space-y-4">
                  <div v-for="integration in settings.integrations.ltiEnabled ? ['lti'] : []" :key="integration" class="flex items-center justify-between p-4 border border-border rounded-lg">
                    <div class="flex items-center gap-3">
                      <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
                        <GlobeIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
                      </div>
                      <div>
                        <p class="font-medium">LTI 1.3 Advantage</p>
                        <p class="text-sm text-muted-foreground">Learning Tools Interoperability integration</p>
                      </div>
                    </div>
                    <Switch v-model="settings.integrations.ltiEnabled" />
                  </div>
                  
                  <div class="flex items-center justify-between p-4 border border-border rounded-lg">
                    <div class="flex items-center gap-3">
                      <div class="p-2 rounded-lg bg-success-100 dark:bg-success-900/30">
                        <FileTextIcon class="h-5 w-5 text-success-600 dark:text-success-400" />
                      </div>
                      <div>
                        <p class="font-medium">SCORM 2004</p>
                        <p class="text-sm text-muted-foreground">SCORM 2004 4th Edition support</p>
                      </div>
                    </div>
                    <Switch v-model="settings.integrations.scormEnabled" />
                  </div>
                  
                  <div class="flex items-center justify-between p-4 border border-border rounded-lg">
                    <div class="flex items-center gap-3">
                      <div class="p-2 rounded-lg bg-warning-100 dark:bg-warning-900/30">
                        <ActivityIcon class="h-5 w-5 text-warning-600 dark:text-warning-400" />
                      </div>
                      <div>
                        <p class="font-medium">xAPI (Tin Can)</p>
                        <p class="text-sm text-muted-foreground">Experience API for learning analytics</p>
                      </div>
                    </div>
                    <Switch v-model="settings.integrations.xapiEnabled" />
                  </div>
                  
                  <div class="flex items-center justify-between p-4 border border-border rounded-lg">
                    <div class="flex items-center gap-3">
                      <div class="p-2 rounded-lg bg-info-100 dark:bg-info-900/30">
                        <ChartIcon class="h-5 w-5 text-info-600 dark:text-info-400" />
                      </div>
                      <div>
                        <p class="font-medium">Caliper Analytics</p>
                        <p class="text-sm text-muted-foreground">IMS Caliper Analytics standard</p>
                      </div>
                    </div>
                    <Switch v-model="settings.integrations.caliperEnabled" />
                  </div>
                  
                  <div class="flex items-center justify-between p-4 border border-border rounded-lg">
                    <div class="flex items-center gap-3">
                      <div class="p-2 rounded-lg bg-secondary-100 dark:bg-secondary-900/30">
                        <UsersIcon class="h-5 w-5 text-secondary-600 dark:text-secondary-400" />
                      </div>
                      <div>
                        <p class="font-medium">OneRoster</p>
                        <p class="text-sm text-muted-foreground">Student information system integration</p>
                      </div>
                    </div>
                    <Switch v-model="settings.integrations.onerosterEnabled" />
                  </div>
                </div>
                
                <Separator class="my-4" />
                
                <div>
                  <h4 class="font-medium mb-4">Single Sign-On Providers</h4>
                  <div class="space-y-4">
                    <div v-for="sso in settings.integrations.ssoProviders" :key="sso.provider" class="flex items-center justify-between p-4 border border-border rounded-lg">
                      <div class="flex items-center gap-3">
                        <div :class="cn('p-2 rounded-lg', sso.enabled ? 'bg-primary-100 dark:bg-primary-900/30' : 'bg-muted')">
                          <component :is="sso.provider === 'google' ? 'ChromeIcon' : sso.provider === 'microsoft' ? 'WindowsIcon' : sso.provider === 'saml' ? 'ShieldIcon' : 'DatabaseIcon'" class="h-5 w-5" :class="sso.enabled ? 'text-primary-600 dark:text-primary-400' : 'text-muted-foreground'" />
                        </div>
                        <div>
                          <p class="font-medium capitalize">{{ sso.provider }}</p>
                          <p class="text-sm text-muted-foreground">{{ sso.provider === 'google' ? 'Google Workspace' : sso.provider === 'microsoft' ? 'Microsoft 365 / Azure AD' : sso.provider === 'saml' ? 'SAML 2.0' : 'LDAP/Active Directory' }}</p>
                        </div>
                      </div>
                      <Switch v-model="sso.enabled" />
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <!-- Storage Settings -->
          <TabsContent value="storage" class="mt-6 space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Storage Configuration</CardTitle>
                <CardDescription>Configure file storage and CDN settings</CardDescription>
              </CardHeader>
              <CardContent class="space-y-6">
                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                  <div>
                    <Label for="storageProvider">Storage Provider</Label>
                    <Select v-model="settings.storage.provider" class="mt-1">
                      <SelectTrigger>
                        <SelectValue placeholder="Select provider" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="s3">AWS S3</SelectItem>
                        <SelectItem value="minio">MinIO</SelectItem>
                        <SelectItem value="azure">Azure Blob</SelectItem>
                        <SelectItem value="gcs">Google Cloud Storage</SelectItem>
                        <SelectItem value="local">Local Filesystem</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <Label for="storageRegion">Region</Label>
                    <Input id="storageRegion" v-model="settings.storage.region" placeholder="us-east-1" class="mt-1" />
                  </div>
                  <div>
                    <Label for="storageBucket">Bucket Name</Label>
                    <Input id="storageBucket" v-model="settings.storage.bucket" placeholder="axel-lms" class="mt-1" />
                  </div>
                  <div>
                    <Label for="storageEndpoint">Endpoint URL</Label>
                    <Input id="storageEndpoint" v-model="settings.storage.endpoint" placeholder="https://s3.amazonaws.com" class="mt-1" />
                  </div>
                </div>
                
                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                  <div>
                    <Label for="accessKey">Access Key</Label>
                    <Input id="accessKey" v-model="settings.storage.accessKey" placeholder="AKIA..." class="mt-1" />
                  </div>
                  <div>
                    <Label for="secretKey">Secret Key</Label>
                    <Input id="secretKey" type="password" v-model="settings.storage.secretKey" placeholder="••••••••" class="mt-1" />
                  </div>
                  <div>
                    <Label for="cdnUrl">CDN URL</Label>
                    <Input id="cdnUrl" v-model="settings.storage.cdnUrl" placeholder="https://cdn.example.com" class="mt-1" />
                  </div>
                  <div>
                    <Label for="maxUploadSize">Max Upload Size (bytes)</Label>
                    <Input id="maxUploadSize" type="number" v-model.number="settings.storage.maxUploadSize" class="mt-1" />
                  </div>
                </div>
                
                <div class="flex items-center justify-between">
                  <div>
                    <Label>Test Connection</Label>
                    <p class="text-sm text-muted-foreground">Verify storage configuration</p>
                  </div>
                  <Button variant="outline" @click="testStorage" class="gap-2">
                    <DatabaseIcon class="h-4 w-4" />
                    Test Connection
                  </Button>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <!-- Advanced Settings -->
          <TabsContent value="advanced" class="mt-6 space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Advanced Settings</CardTitle>
                <CardDescription>Advanced platform configuration options</CardDescription>
              </CardHeader>
              <CardContent class="space-y-6">
                <div class="space-y-4">
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Maintenance Mode</Label>
                      <p class="text-sm text-muted-foreground">Enable maintenance mode globally</p>
                    </div>
                    <Switch v-model="settings.advanced.maintenanceMode" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Debug Mode</Label>
                      <p class="text-sm text-muted-foreground">Enable debug logging and error details</p>
                    </div>
                    <Switch v-model="settings.advanced.debugMode" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Cache Enabled</Label>
                      <p class="text-sm text-muted-foreground">Enable response caching</p>
                    </div>
                    <Switch v-model="settings.advanced.cacheEnabled" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Rate Limiting Enabled</Label>
                      <p class="text-sm text-muted-foreground">Enable API rate limiting</p>
                    </div>
                    <Switch v-model="settings.advanced.rateLimitEnabled" />
                  </div>
                </div>
                
                <Separator class="my-4" />
                
                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                  <div>
                    <Label for="cacheTtl">Cache TTL (seconds)</Label>
                    <Input id="cacheTtl" type="number" v-model.number="settings.advanced.cacheTtl" min="60" max="86400" class="mt-1" />
                  </div>
                  <div>
                    <Label for="logLevel">Log Level</Label>
                    <Select v-model="settings.advanced.logLevel" class="mt-1">
                      <SelectTrigger>
                        <SelectValue placeholder="Log level" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="debug">Debug</SelectItem>
                        <SelectItem value="info">Info</SelectItem>
                        <SelectItem value="warn">Warn</SelectItem>
                        <SelectItem value="error">Error</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <Label for="rateLimitRequests">Rate Limit Requests</Label>
                    <Input id="rateLimitRequests" type="number" v-model.number="settings.advanced.rateLimitRequests" min="100" max="10000" class="mt-1" />
                  </div>
                  <div>
                    <Label for="rateLimitWindow">Rate Limit Window (seconds)</Label>
                    <Input id="rateLimitWindow" type="number" v-model.number="settings.advanced.rateLimitWindow" min="1" max="3600" class="mt-1" />
                  </div>
                </div>
                
                <Separator class="my-4" />
                
                <div class="space-y-4">
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>API Enabled</Label>
                      <p class="text-sm text-muted-foreground">Enable public API access</p>
                    </div>
                    <Switch v-model="settings.advanced.apiEnabled" />
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <div>
                      <Label for="apiRateLimit">API Rate Limit (req/min)</Label>
                      <Input id="apiRateLimit" type="number" v-model.number="settings.advanced.apiRateLimit" min="100" max="10000" class="mt-1 w-48" />
                    </div>
                  </div>
                </div>
                
                <Separator class="my-4" />
                
                <div class="space-y-4">
                  <div class="flex items-center justify-between">
                    <div>
                      <Label>Webhooks Enabled</Label>
                      <p class="text-sm text-muted-foreground">Enable webhook notifications</p>
                    </div>
                    <Switch v-model="settings.advanced.webhooksEnabled" />
                  </div>
                  
                  <div>
                    <Label for="webhookSecret">Webhook Secret</Label>
                    <Input id="webhookSecret" type="password" v-model="settings.advanced.webhookSecret" placeholder="••••••••" class="mt-1" />
                    <p class="text-xs text-muted-foreground mt-1">Secret used to sign webhook payloads</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  </div>
</template>

<script>
const SendIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 19l9 2-9-18-9 18 9-2zm0 0v-8' })]) }})
const ChromeIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('circle', { cx: '12', cy: '12', r: '10', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 2a10 10 0 100 20 10 10 0 000-20z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6v6l4 4' })]) }})
const WindowsIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '2', y: '4', width: '20', height: '16', rx: '2', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 16h12M12 8v12' })]) }})
const ChartIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M18 20V10M12 20V4M6 20v-6' })]) }})
const FileTextIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })]) }})
const DatabaseIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('ellipse', { cx: '12', cy: '5', rx: '9', ry: '3', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 5v14a2 2 0 002 2h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v9' })]) }})
</script>