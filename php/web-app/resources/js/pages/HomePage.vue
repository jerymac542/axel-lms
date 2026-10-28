<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { ArrowRightIcon, CheckIcon, SparklesIcon, ShieldIcon, UsersIcon, BookOpenIcon, BarChartIcon, GlobeIcon, ZapIcon, AwardIcon, LockIcon, CloudIcon, CpuIcon } from '@/components/ui/icons'
import { Link } from '@inertiajs/vue3'
import { useTheme } from '@/composables/useTheme'

const theme = useTheme()

const features = [
  {
    icon: SparklesIcon,
    title: 'AI-Powered Learning',
    description: 'Personalized recommendations, automated grading, intelligent tutoring, and content generation powered by state-of-the-art ML models.',
    href: '/features#ai',
  },
  {
    icon: BookOpenIcon,
    title: 'Course Authoring Studio',
    description: 'Drag-and-drop course builder with rich media support, SCORM/xAPI compliance, and collaborative authoring workflows.',
    href: '/features#authoring',
  },
  {
    icon: UsersIcon,
    title: 'Social Learning',
    description: 'Discussion forums, peer review, study groups, mentorship programs, and community-driven knowledge sharing.',
    href: '/features#social',
  },
  {
    icon: BarChartIcon,
    title: 'Advanced Analytics',
    description: 'Real-time dashboards, predictive analytics, learning insights, and customizable reports for data-driven decisions.',
    href: '/features#analytics',
  },
  {
    icon: ShieldIcon,
    title: 'Enterprise Security',
    description: 'SSO/SAML/OIDC, granular RBAC, audit logs, data encryption, GDPR/FERPA compliance, and SOC 2 Type II certified.',
    href: '/features#security',
  },
  {
    icon: GlobeIcon,
    title: 'Global Scale',
    description: 'Multi-tenant architecture, 50+ languages, RTL support, CDN delivery, and 99.9% SLA with geo-redundancy.',
    href: '/features#scale',
  },
  {
    icon: AwardIcon,
    title: 'Certifications & Badges',
    description: 'Verifiable credentials, blockchain certificates, skill badges, learning pathways, and portfolio exports.',
    href: '/features#credentials',
  },
  {
    icon: LockIcon,
    title: 'Standards Compliant',
    description: 'LTI 1.3 Advantage, SCORM 2004 4th Ed, xAPI, Caliper, OneRoster, QTI 3.0, and WCAG 2.1 AA accessible.',
    href: '/features#standards',
  },
  {
    icon: CloudIcon,
    title: 'Cloud & On-Premise',
    description: 'Deploy on Kubernetes, VMs, or air-gapped environments. Helm charts, Ansible playbooks, and Terraform modules included.',
    href: '/features#deployment',
  },
]

const stats = [
  { label: 'Organizations', value: '2,500+', icon: BuildingIcon },
  { label: 'Active Learners', value: '1.2M+', icon: UsersIcon },
  { label: 'Courses Delivered', value: '500K+', icon: BookOpenIcon },
  { label: 'Countries', value: '85+', icon: GlobeIcon },
  { label: 'Uptime SLA', value: '99.9%', icon: ShieldIcon },
  { label: 'API Requests/Day', value: '500M+', icon: CpuIcon },
]

const testimonials = [
  {
    quote: "Axel LMS transformed how our global workforce learns. The AI recommendations alone increased course completion by 40%.",
    author: "Sarah Chen",
    role: "Chief Learning Officer",
    company: "TechGlobal Inc.",
    avatar: "/avatars/sarah-chen.jpg",
  },
  {
    quote: "Finally, an LMS that developers love to integrate with. The API-first design and comprehensive docs saved us months of work.",
    author: "Marcus Johnson",
    role: "VP Engineering",
    company: "EduTech Solutions",
    avatar: "/avatars/marcus-johnson.jpg",
  },
  {
    quote: "The accessibility features are best-in-class. Our WCAG 2.1 AA audit passed with zero issues - a first for our platform.",
    author: "Dr. Emily Rodriguez",
    role: "Accessibility Lead",
    company: "State University System",
    avatar: "/avatars/emily-rodriguez.jpg",
  },
]

const integrations = [
  { name: 'Microsoft Teams', logo: '/logos/teams.svg', category: 'Communication' },
  { name: 'Slack', logo: '/logos/slack.svg', category: 'Communication' },
  { name: 'Zoom', logo: '/logos/zoom.svg', category: 'Video Conferencing' },
  { name: 'Google Workspace', logo: '/logos/google.svg', category: 'Productivity' },
  { name: 'Microsoft 365', logo: '/logos/m365.svg', category: 'Productivity' },
  { name: 'Salesforce', logo: '/logos/salesforce.svg', category: 'CRM' },
  { name: 'Workday', logo: '/logos/workday.svg', category: 'HRIS' },
  { name: 'SAP SuccessFactors', logo: '/logos/sap.svg', category: 'HRIS' },
  { name: 'Canvas', logo: '/logos/canvas.svg', category: 'LMS' },
  { name: 'Moodle', logo: '/logos/moodle.svg', category: 'LMS' },
  { name: 'Blackboard', logo: '/logos/blackboard.svg', category: 'LMS' },
  { name: 'Contentful', logo: '/logos/contentful.svg', category: 'CMS' },
  { name: 'Auth0', logo: '/logos/auth0.svg', category: 'Identity' },
  { name: 'Okta', logo: '/logos/okta.svg', category: 'Identity' },
  { name: 'Azure AD', logo: '/logos/azure-ad.svg', category: 'Identity' },
  { name: 'OneLogin', logo: '/logos/onelogin.svg', category: 'Identity' },
]
</script>

<template>
  <div class="min-h-screen bg-background">
    <!-- Hero Section -->
    <section class="relative overflow-hidden bg-gradient-to-b from-primary-50/50 to-background dark:from-primary-950/20 dark:to-background">
      <div class="container mx-auto px-4 py-20 lg:py-32">
        <div class="max-w-4xl mx-auto text-center">
          <!-- Badge -->
          <div class="inline-flex items-center gap-2 rounded-full bg-primary-100/80 px-4 py-1.5 text-sm font-medium text-primary-800 dark:bg-primary-900/30 dark:text-primary-200 mb-8 animate-in">
            <Badge variant="default" class="gap-1">
              <ZapIcon class="h-3 w-3" />
              <span>Version 2.0 Released</span>
            </Badge>
            <span class="text-muted-foreground">Now with AI Tutoring & Advanced Analytics</span>
          </div>

          <!-- Headline -->
          <h1 class="text-4xl md:text-5xl lg:text-6xl font-display font-bold tracking-tight text-foreground mb-6 animate-in">
            The Enterprise Learning Platform
            <br />
            <span class="text-primary-600 dark:text-primary-400">Built for Modern Organizations</span>
          </h1>

          <!-- Subheadline -->
          <p class="text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto mb-10 animate-in">
            Build, deliver, and scale world-class learning experiences. From employee onboarding to customer education, 
            Axel LMS powers learning for the world's most innovative organizations.
          </p>

          <!-- CTA Buttons -->
          <div class="flex flex-col sm:flex-row items-center justify-center gap-4 animate-in">
            <Link href="/demo">
              <Button size="lg" class="gap-2 w-full sm:w-auto">
                <span>Start Free Trial</span>
                <ArrowRightIcon class="h-4 w-4" />
              </Button>
            </Link>
            <Link href="/demo">
              <Button size="lg" variant="outline" class="w-full sm:w-auto">
                Watch Demo
              </Button>
            </Link>
          </div>

          <!-- Trust Indicators -->
          <div class="mt-16 flex flex-wrap items-center justify-center gap-8 text-sm text-muted-foreground animate-in">
            <div class="flex items-center gap-2">
              <CheckIcon class="h-4 w-4 text-success-500" />
              <span>No credit card required</span>
            </div>
            <div class="flex items-center gap-2">
              <CheckIcon class="h-4 w-4 text-success-500" />
              <span>14-day free trial</span>
            </div>
            <div class="flex items-center gap-2">
              <CheckIcon class="h-4 w-4 text-success-500" />
              <span>Cancel anytime</span>
            </div>
            <div class="flex items-center gap-2">
              <ShieldIcon class="h-4 w-4 text-success-500" />
              <span>SOC 2 Type II certified</span>
            </div>
          </div>
        </div>

        <!-- Floating Elements -->
        <div class="absolute inset-0 pointer-events-none overflow-hidden">
          <div class="absolute top-1/4 left-1/4 w-72 h-72 bg-primary-100/30 dark:bg-primary-900/20 rounded-full blur-3xl animate-pulse-slow" />
          <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-accent-100/30 dark:bg-accent-900/20 rounded-full blur-3xl animate-pulse-slow" style="animation-delay: 1s;" />
        </div>
      </div>
    </section>

    <!-- Stats Bar -->
    <section class="py-12 border-y border-border bg-background/50">
      <div class="container mx-auto px-4">
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-8">
          <div v-for="stat in stats" :key="stat.label" class="text-center">
            <div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-primary-100 dark:bg-primary-900/30 mb-4">
              <component :is="stat.icon" class="h-8 w-8 text-primary-600 dark:text-primary-400" />
            </div>
            <div class="text-3xl md:text-4xl font-display font-bold text-foreground">{{ stat.value }}</div>
            <div class="text-sm text-muted-foreground">{{ stat.label }}</div>
          </div>
        </div>
      </div>
    </section>

    <!-- Features -->
    <section class="py-24 lg:py-32">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-2xl mx-auto mb-16">
          <Badge variant="secondary" class="mb-4">Why Axel LMS</Badge>
          <h2 class="text-3xl md:text-4xl font-display font-bold tracking-tight mb-4">
            Everything You Need to Deliver Exceptional Learning
          </h2>
          <p class="text-lg text-muted-foreground">
            Nine powerful pillars that make Axel LMS the complete learning platform for enterprise.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <Card v-for="feature in features" :key="feature.title" class="group hover:shadow-lg transition-all duration-300 h-full">
            <CardHeader>
              <div class="inline-flex items-center justify-center w-12 h-12 rounded-xl bg-primary-100 dark:bg-primary-900/30 mb-4 group-hover:scale-110 transition-transform">
                <component :is="feature.icon" class="h-6 w-6 text-primary-600 dark:text-primary-400" />
              </div>
              <CardTitle class="text-xl">{{ feature.title }}</CardTitle>
              <CardDescription>{{ feature.description }}</CardDescription>
            </CardHeader>
            <CardContent>
              <Link :href="feature.href" class="inline-flex items-center gap-1 text-sm font-medium text-primary-600 dark:text-primary-400 hover:underline">
                Learn more
                <ArrowRightIcon class="h-4 w-4 transition-transform group-hover:translate-x-1" />
              </Link>
            </CardContent>
          </Card>
        </div>
      </div>
    </section>

    <!-- Testimonials -->
    <section class="py-24 lg:py-32 bg-muted/30">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-2xl mx-auto mb-16">
          <Badge variant="secondary" class="mb-4">Trusted by Industry Leaders</Badge>
          <h2 class="text-3xl md:text-4xl font-display font-bold tracking-tight mb-4">
            Loved by Learning Leaders Worldwide
          </h2>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-5xl mx-auto">
          <Card v-for="(testimonial, index) in testimonials" :key="index" class="h-full">
            <CardContent class="pt-6">
              <div class="flex gap-2 mb-4">
                <span v-for="i in 5" :key="i" class="text-warning-500">★</span>
              </div>
              <p class="text-lg text-foreground mb-6 italic">"{{ testimonial.quote }}"</p>
              <div class="flex items-center gap-4">
                <Avatar :size="lg" :src="testimonial.avatar" :fallback="testimonial.author.split(' ').map(n => n[0]).join('')" />
                <div>
                  <p class="font-medium">{{ testimonial.author }}</p>
                  <p class="text-sm text-muted-foreground">{{ testimonial.role }}, {{ testimonial.company }}</p>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </section>

    <!-- Integrations -->
    <section class="py-24 lg:py-32">
      <div class="container mx-auto px-4">
        <div class="text-center max-w-2xl mx-auto mb-16">
          <Badge variant="secondary" class="mb-4">Seamless Integration</Badge>
          <h2 class="text-3xl md:text-4xl font-display font-bold tracking-tight mb-4">
            Works with Your Existing Stack
          </h2>
          <p class="text-lg text-muted-foreground">
            100+ pre-built integrations and a robust API for custom connections.
          </p>
        </div>

        <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
          <Card v-for="integration in integrations" :key="integration.name" class="p-4 text-center hover:bg-accent transition-colors cursor-pointer group">
            <div class="w-12 h-12 mx-auto mb-3 bg-muted-100 dark:bg-muted-800 rounded-xl flex items-center justify-center group-hover:scale-110 transition-transform">
              <span class="text-sm font-medium text-muted-foreground">{{ integration.name.split(' ').map(w => w[0]).join('') }}</span>
            </div>
            <p class="text-sm font-medium">{{ integration.name }}</p>
            <p class="text-xs text-muted-foreground">{{ integration.category }}</p>
          </Card>
        </div>

        <div class="text-center mt-12">
          <Link href="/integrations">
            <Button variant="outline" size="lg">View All 100+ Integrations</Button>
          </Link>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section class="py-24 lg:py-32">
      <div class="container mx-auto px-4">
        <div class="max-w-3xl mx-auto text-center">
          <div class="relative rounded-3xl bg-gradient-to-r from-primary-600 via-primary-700 to-accent-600 p-8 md:p-12 lg:p-16 overflow-hidden">
            <div class="absolute inset-0 bg-[url('/patterns/grid.svg')] opacity-10" />
            <div class="relative z-10">
              <h2 class="text-3xl md:text-4xl lg:text-5xl font-display font-bold text-white mb-6">
                Ready to Transform Your Learning?
              </h2>
              <p class="text-lg md:text-xl text-primary-100 mb-8 max-w-xl mx-auto">
                Join 2,500+ organizations delivering exceptional learning experiences. Start your free trial today.
              </p>
              <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
                <Link href="/register">
                  <Button size="xl" variant="secondary" class="gap-2 w-full sm:w-auto">
                    <span>Start Free Trial</span>
                    <ArrowRightIcon class="h-5 w-5" />
                  </Button>
                </Link>
                <Link href="/contact-sales">
                  <Button size="xl" variant="outline" class="w-full sm:w-auto border-white text-white hover:bg-white/10">
                    Contact Sales
                  </Button>
                </Link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="border-t border-border bg-muted/30 py-16">
      <div class="container mx-auto px-4">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8 mb-12">
          <div class="col-span-2 md:col-span-1">
            <Link href="/" class="flex items-center gap-2 font-display font-bold text-xl text-foreground mb-4">
              <svg class="h-8 w-8 text-primary-600 dark:text-primary-400" viewBox="0 0 32 32" fill="none">
                <rect width="32" height="32" rx="8" fill="currentColor"/>
                <path d="M8 12L16 8L24 12V20L16 24L8 20V12Z" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M16 8V24" stroke="white" stroke-width="2" stroke-linecap="round"/>
                <path d="M8 12L24 12" stroke="white" stroke-width="2" stroke-linecap="round"/>
                <path d="M8 20L24 20" stroke="white" stroke-width="2" stroke-linecap="round"/>
              </svg>
              <span>Axel LMS</span>
            </Link>
            <p class="text-sm text-muted-foreground mb-6 max-w-xs">
              The enterprise learning platform for modern organizations.
            </p>
            <div class="flex gap-4">
              <a href="https://twitter.com/axellms" target="_blank" rel="noopener" class="text-muted-foreground hover:text-foreground transition-colors">
                <TwitterIcon class="h-5 w-5" />
              </a>
              <a href="https://github.com/axellms" target="_blank" rel="noopener" class="text-muted-foreground hover:text-foreground transition-colors">
                <GitHubIcon class="h-5 w-5" />
              </a>
              <a href="https://linkedin.com/company/axellms" target="_blank" rel="noopener" class="text-muted-foreground hover:text-foreground transition-colors">
                <LinkedInIcon class="h-5 w-5" />
              </a>
            </div>
          </div>
          <nav>
            <h4 class="font-semibold mb-4">Product</h4>
            <ul class="space-y-2 text-sm text-muted-foreground">
              <li><Link href="/features" class="hover:text-foreground transition-colors">Features</Link></li>
              <li><Link href="/pricing" class="hover:text-foreground transition-colors">Pricing</Link></li>
              <li><Link href="/integrations" class="hover:text-foreground transition-colors">Integrations</Link></li>
              <li><Link href="/changelog" class="hover:text-foreground transition-colors">Changelog</Link></li>
              <li><Link href="/roadmap" class="hover:text-foreground transition-colors">Roadmap</Link></li>
            </ul>
          </nav>
          <nav>
            <h4 class="font-semibold mb-4">Resources</h4>
            <ul class="space-y-2 text-sm text-muted-foreground">
              <li><Link href="/docs" class="hover:text-foreground transition-colors">Documentation</Link></li>
              <li><Link href="/help" class="hover:text-foreground transition-colors">Help Center</Link></li>
              <li><Link href="/community" class="hover:text-foreground transition-colors">Community</Link></li>
              <li><Link href="/webinars" class="hover:text-foreground transition-colors">Webinars</Link></li>
              <li><Link href="/api-docs" class="hover:text-foreground transition-colors">API Reference</Link></li>
            </ul>
          </nav>
          <nav>
            <h4 class="font-semibold mb-4">Company</h4>
            <ul class="space-y-2 text-sm text-muted-foreground">
              <li><Link href="/about" class="hover:text-foreground transition-colors">About</Link></li>
              <li><Link href="/blog" class="hover:text-foreground transition-colors">Blog</Link></li>
              <li><Link href="/careers" class="hover:text-foreground transition-colors">Careers</Link></li>
              <li><Link href="/press" class="hover:text-foreground transition-colors">Press</Link></li>
              <li><Link href="/contact" class="hover:text-foreground transition-colors">Contact</Link></li>
            </ul>
          </nav>
        </div>
        <div class="border-t border-border pt-8 flex flex-col md:flex-row items-center justify-between gap-4 text-sm text-muted-foreground">
          <p>© {{ new Date().getFullYear() }} Axel LMS. All rights reserved.</p>
          <div class="flex gap-6">
            <Link href="/privacy" class="hover:text-foreground transition-colors">Privacy</Link>
            <Link href="/terms" class="hover:text-foreground transition-colors">Terms</Link>
            <Link href="/cookies" class="hover:text-foreground transition-colors">Cookies</Link>
            <Link href="/security" class="hover:text-foreground transition-colors">Security</Link>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script>
// Icons
const ZapIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 10V3L4 14h7v7l9-11h-7z' })]) }})
const ArrowRightIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17 8l4 4m0 0l-4 4m4-4H3' })]) }})
const CheckIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 13l4 4L19 7' })]) }})
const ShieldIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
const UsersIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })]) }})
const BookOpenIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' })]) }})
const BarChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 012-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' })]) }})
const GlobeIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9' })]) }})
const ZapIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 10V3L4 14h7v7l9-11h-7z' })]) }})
const AwardIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const LockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z' })]) }})
const CloudIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 00-9.78 2.096A4.001 4.001 0 003 15z' })]) }})
const CpuIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '4', y: '4', width: '16', height: '16', rx: '2', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 8h6M9 12h6M9 16h6' })]) }})
const BuildingIcon = defineComponent({ render() { return h('svg', { class: 'h-8 w-8', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4' })]) }})
const TwitterIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z' })]) }})
const GitHubIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z' })]) }})
const LinkedInIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z' })]) }})
</script>