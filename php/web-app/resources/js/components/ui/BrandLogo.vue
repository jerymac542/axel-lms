<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'

interface Props {
  brand?: 'axel' | 'apexos' | 'apex_academy'
  variant?: 'primary' | 'icon' | 'secondary' | 'accent' | 'badge'
  theme?: 'light' | 'dark' | 'auto'
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full'
  class?: string
  ariaLabel?: string
}

const props = withDefaults(defineProps<Props>(), {
  brand: 'axel',
  variant: 'primary',
  theme: 'auto',
  size: 'md',
})

const brandConfig = {
  axel: {
    primary: { light: '/images/logos/axel-lms-logo.png', dark: '/images/logos/axel-lms-logo.png', width: 160, height: 40 },
    icon: { light: '/images/logos/axel-lms-icon.png', dark: '/images/logos/axel-lms-icon.png', width: 32, height: 32 },
  },
  apexos: {
    primary: { light: '/images/logos/apexos-icon-primary.png', dark: '/images/logos/apexos-icon-primary.png', width: 140, height: 35 },
    icon: { light: '/images/logos/apexos-icon-primary.png', dark: '/images/logos/apexos-icon-primary.png', width: 32, height: 32 },
    secondary: { light: '/images/logos/apexos-icon-secondary.png', dark: '/images/logos/apexos-icon-secondary.png', width: 140, height: 35 },
    accent: { light: '/images/logos/apexos-icon-accent.png', dark: '/images/logos/apexos-icon-accent.png', width: 140, height: 35 },
  },
  apex_academy: {
    primary: { light: '/images/logos/apex-academy-certifications.png', dark: '/images/logos/apex-academy-certifications.png', width: 180, height: 45 },
    icon: { light: '/images/logos/apex-academy-icon.png', dark: '/images/logos/apex-academy-icon.png', width: 32, height: 32 },
    badge: { light: '/images/logos/apex-academy-badge.png', dark: '/images/logos/apex-academy-badge.png', width: 64, height: 64 },
  },
}

const sizeMultipliers = {
  sm: 0.75,
  md: 1,
  lg: 1.25,
  xl: 1.5,
  full: 2,
}

const logoConfig = computed(() => {
  const brand = brandConfig[props.brand as keyof typeof brandConfig] || brandConfig.axel
  return brand[props.variant as keyof typeof brand] || brand.primary
})

const currentTheme = computed(() => {
  if (props.theme !== 'auto') return props.theme
  if (typeof document !== 'undefined') {
    return document.documentElement.classList.contains('dark') ? 'dark' : 'light'
  }
  return 'light'
})

const logoSrc = computed(() => {
  const config = logoConfig.value
  return config[currentTheme.value as 'light' | 'dark'] || config.light
})

const logoWidth = computed(() => {
  const config = logoConfig.value
  return Math.round(config.width * sizeMultipliers[props.size])
})

const logoHeight = computed(() => {
  const config = logoConfig.value
  return Math.round(config.height * sizeMultipliers[props.size])
})

const ariaLabel = computed(() => {
  if (props.ariaLabel) return props.ariaLabel
  const brandNames = { axel: 'Axel LMS', apexos: 'Apexos', apex_academy: 'Apex Academy' }
  return `${brandNames[props.brand as keyof typeof brandNames] || 'Logo'} ${props.variant} logo`
})
</script>

<template>
  <img
    :src="logoSrc"
    :alt="ariaLabel"
    :width="logoWidth"
    :height="logoHeight"
    :class="cn(
      'transition-opacity duration-200',
      variant === 'icon' && 'block',
      class
    )"
    :aria-label="ariaLabel"
    loading="lazy"
    decoding="async"
  />
</template>