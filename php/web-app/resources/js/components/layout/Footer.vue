<script setup lang="ts">
import { Link } from '@inertiajs/vue3'

const currentYear = new Date().getFullYear()

const footerLinks = {
  product: [
    { name: 'Features', href: '/features' },
    { name: 'Pricing', href: '/pricing' },
    { name: 'Courses', href: '/courses' },
    { name: 'Learning Paths', href: '/learning-paths' },
    { name: 'Certificates', href: '/certificates' },
    { name: 'Integrations', href: '/integrations' },
  ],
  company: [
    { name: 'About', href: '/about' },
    { name: 'Blog', href: '/blog' },
    { name: 'Careers', href: '/careers' },
    { name: 'Press', href: '/press' },
    { name: 'Partners', href: '/partners' },
    { name: 'Contact', href: '/contact' },
  ],
  resources: [
    { name: 'Documentation', href: '/docs' },
    { name: 'Help Center', href: '/help' },
    { name: 'Community', href: '/community' },
    { name: 'Webinars', href: '/webinars' },
    { name: 'API Reference', href: '/api-docs' },
    { name: 'Status', href: '/status' },
  ],
  legal: [
    { name: 'Privacy', href: '/privacy' },
    { name: 'Terms', href: '/terms' },
    { name: 'Cookie Policy', href: '/cookies' },
    { name: 'Security', href: '/security' },
    { name: 'Accessibility', href: '/accessibility' },
    { name: 'Licenses', href: '/licenses' },
  ],
}

const socialLinks = [
  { name: 'Twitter', href: 'https://twitter.com/axellms', icon: TwitterIcon },
  { name: 'GitHub', href: 'https://github.com/axellms', icon: GitHubIcon },
  { name: 'LinkedIn', href: 'https://linkedin.com/company/axellms', icon: LinkedInIcon },
  { name: 'YouTube', href: 'https://youtube.com/axellms', icon: YouTubeIcon },
  { name: 'Discord', href: 'https://discord.gg/axellms', icon: DiscordIcon },
]
</script>

<template>
  <footer class="border-t border-border bg-muted/30" role="contentinfo">
    <div class="container mx-auto px-4 py-12 lg:py-16">
      <div class="grid grid-cols-2 gap-8 md:grid-cols-4 lg:grid-cols-5">
        <!-- Brand -->
        <div class="col-span-2 md:col-span-1 lg:col-span-1">
          <Link href="/" class="flex items-center gap-2 font-display font-bold text-xl text-foreground mb-4" aria-label="Axel LMS Home">
            <svg class="h-8 w-8 text-primary-600 dark:text-primary-400" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
              <rect width="32" height="32" rx="8" fill="currentColor"/>
              <path d="M8 12L16 8L24 12V20L16 24L8 20V12Z" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M16 8V24" stroke="white" stroke-width="2" stroke-linecap="round"/>
              <path d="M8 12L24 12" stroke="white" stroke-width="2" stroke-linecap="round"/>
              <path d="M8 20L24 20" stroke="white" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <span>Axel LMS</span>
          </Link>
          <p class="text-sm text-muted-foreground mb-6 max-w-xs">
            The enterprise learning platform for modern organizations. Build, deliver, and scale world-class learning experiences.
          </p>
          <div class="flex gap-4">
            <a v-for="social in socialLinks" :key="social.name" :href="social.href" target="_blank" rel="noopener noreferrer" class="text-muted-foreground hover:text-foreground transition-colors" :aria-label="social.name">
              <component :is="social.icon" class="h-5 w-5" />
            </a>
          </div>
        </div>

        <!-- Product -->
        <nav aria-label="Product links">
          <h3 class="font-semibold text-foreground mb-4">Product</h3>
          <ul class="space-y-3">
            <li v-for="link in footerLinks.product" :key="link.name">
              <Link :href="link.href" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                {{ link.name }}
              </Link>
            </li>
          </ul>
        </nav>

        <!-- Company -->
        <nav aria-label="Company links">
          <h3 class="font-semibold text-foreground mb-4">Company</h3>
          <ul class="space-y-3">
            <li v-for="link in footerLinks.company" :key="link.name">
              <Link :href="link.href" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                {{ link.name }}
              </Link>
            </li>
          </ul>
        </nav>

        <!-- Resources -->
        <nav aria-label="Resources links">
          <h3 class="font-semibold text-foreground mb-4">Resources</h3>
          <ul class="space-y-3">
            <li v-for="link in footerLinks.resources" :key="link.name">
              <Link :href="link.href" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                {{ link.name }}
              </Link>
            </li>
          </ul>
        </nav>

        <!-- Legal -->
        <nav aria-label="Legal links">
          <h3 class="font-semibold text-foreground mb-4">Legal</h3>
          <ul class="space-y-3">
            <li v-for="link in footerLinks.legal" :key="link.name">
              <Link :href="link.href" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                {{ link.name }}
              </Link>
            </li>
          </ul>
        </nav>
      </div>

      <!-- Bottom Bar -->
      <div class="mt-12 pt-8 border-t border-border flex flex-col md:flex-row items-center justify-between gap-4">
        <p class="text-sm text-muted-foreground">
          © {{ currentYear }} Axel LMS. All rights reserved.
        </p>
        <div class="flex items-center gap-6 text-sm text-muted-foreground">
          <span>Version {{ $page.props.version || '0.1.0' }}</span>
          <Link href="/privacy" class="hover:text-foreground transition-colors">Privacy</Link>
          <Link href="/terms" class="hover:text-foreground transition-colors">Terms</Link>
          <Link href="/cookies" class="hover:text-foreground transition-colors">Cookies</Link>
        </div>
      </div>
    </div>
  </footer>
</template>

<script>
// Social icons
const TwitterIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z' })]) }})
const GitHubIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z' })]) }})
const LinkedInIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z' })]) }})
const YouTubeIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z' })]) }})
const DiscordIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { d: 'M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.675 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.083.083 0 0 0 .031.057 19.9 19.9 0 0 0 5.995 4.648.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-.074c.361-.118.735-.186 1.102-.186.367 0 .73.068 1.091.186a.077.077 0 0 0 .085.028 19.839 19.839 0 0 0 6.003-4.594.077.077 0 0 0 .032-.054c.413-4.506-.43-9.034-1.548-13.654a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418z' })]) }})
</script>