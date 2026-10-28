<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/utils/cn'

interface Props {
  value: number
  max?: number
  showLabel?: boolean
  label?: string
  variant?: 'default' | 'circle' | 'steps'
  size?: 'sm' | 'md' | 'lg'
  color?: 'primary' | 'success' | 'warning' | 'destructive' | 'info'
  striped?: boolean
  animated?: boolean
  class?: string
}

const props = withDefaults(defineProps<Props>(), {
  max: 100,
  showLabel: false,
  variant: 'default',
  size: 'md',
  color: 'primary',
  striped: false,
  animated: false,
})

const percentage = computed(() => Math.min(Math.max((props.value / props.max) * 100, 0), 100))

const colorClasses = {
  primary: 'bg-primary-600 dark:bg-primary-500',
  success: 'bg-success-600 dark:bg-success-500',
  warning: 'bg-warning-600 dark:bg-warning-500',
  destructive: 'bg-destructive-600 dark:bg-destructive-500',
  info: 'bg-info-600 dark:bg-info-500',
}

const sizeClasses = {
  sm: 'h-1',
  md: 'h-2',
  lg: 'h-3',
}

const trackClasses = {
  sm: 'h-1',
  md: 'h-2',
  lg: 'h-3',
}

const labelSizeClasses = {
  sm: 'text-xs',
  md: 'text-sm',
  lg: 'text-base',
}
</script>

<template>
  <div :class="cn('w-full', class)">
    <!-- Label -->
    <div v-if="showLabel || label" class="flex items-center justify-between mb-1">
      <span v-if="label" :class="cn('font-medium', labelSizeClasses[size])">{{ label }}</span>
      <span v-if="showLabel" :class="cn('font-mono tabular-nums text-muted-foreground', labelSizeClasses[size])">
        {{ percentage.toFixed(0) }}%
      </span>
    </div>

    <!-- Linear Progress -->
    <div 
      v-if="variant === 'default'"
      :class="cn(
        'relative w-full overflow-hidden rounded-full bg-secondary-200 dark:bg-secondary-800',
        trackClasses[size]
      )"
      role="progressbar"
      :aria-valuenow="percentage.value"
      :aria-valuemin="0"
      :aria-valuemax="100"
      :aria-label="label"
    >
      <div
        :class="cn(
          'h-full rounded-full transition-all duration-300 ease-out',
          colorClasses[color],
          striped && 'bg-gradient-to-r bg-[length:20px_20px] bg-[linear-gradient(45deg,rgba(255,255,255,.15)_25%,transparent_25%,transparent_50%,rgba(255,255,255,.15)_50%,rgba(255,255,255,.15)_75%,transparent_75%,transparent)',
          animated && 'animate-pulse'
        )"
        :style="{ width: percentage.value + '%' }"
      />
    </div>

    <!-- Circular Progress -->
    <div v-if="variant === 'circle'" class="inline-flex items-center justify-center">
      <svg :width="size === 'sm' ? 40 : size === 'md' ? 60 : 80" :height="size === 'sm' ? 40 : size === 'md' ? 60 : 80" class="transform -rotate-90">
        <circle
          class="text-secondary-200 dark:text-secondary-800"
          stroke-width="4"
          stroke="currentColor"
          fill="transparent"
          r="size === 'sm' ? 16 : size === 'md' ? 26 : 36"
          cx="size === 'sm' ? 20 : size === 'md' ? 30 : 40"
          cy="size === 'sm' ? 20 : size === 'md' ? 30 : 40"
        />
        <circle
          :class="cn('transition-all duration-500 ease-out', colorClasses[color])"
          stroke-width="4"
          stroke-dasharray="size === 'sm' ? 100 : size === 'md' ? 163 : 226"
          :stroke-dashoffset="size === 'sm' ? 100 - percentage : size === 'md' ? 163 - percentage * 1.63 : 226 - percentage * 2.26"
          stroke-linecap="round"
          stroke="currentColor"
          fill="transparent"
          r="size === 'sm' ? 16 : size === 'md' ? 26 : 36"
          cx="size === 'sm' ? 20 : size === 'md' ? 30 : 40"
          cy="size === 'sm' ? 20 : size === 'md' ? 30 : 40"
        />
        <text
          x="50%"
          y="50%"
          dominant-baseline="middle"
          text-anchor="middle"
          :class="cn('font-medium', size === 'sm' ? 'text-xs' : size === 'md' ? 'text-sm' : 'text-base')"
        >
          {{ percentage.toFixed(0) }}%
        </text>
      </svg>
    </div>

    <!-- Steps Progress -->
    <div v-if="variant === 'steps'" class="relative">
      <div class="flex items-center justify-between">
        <template v-for="(step, index) in steps" :key="index">
          <div class="relative flex flex-col items-center">
            <div 
              :class="cn(
                'w-8 h-8 rounded-full border-2 flex items-center justify-center transition-all duration-300',
                index < currentStep ? 'bg-primary-600 border-primary-600 text-white' : 
                index === currentStep ? 'bg-primary-100 dark:bg-primary-900 border-primary-600 text-primary-600' :
                'bg-background border-border text-muted-foreground'
              )"
            >
              <CheckIcon v-if="index < currentStep" class="h-4 w-4" />
              <span v-else class="text-sm font-medium">{{ index + 1 }}</span>
            </div>
            <span class="mt-2 text-xs text-center text-muted-foreground max-w-[80px]">{{ step.label }}</span>
          </div>
          <div v-if="index < steps.length - 1" class="flex-1 h-1 bg-border relative">
            <div 
              v-if="index < currentStep - 1"
              class="absolute inset-0 bg-primary-600"
            />
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script>
const CheckIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 13l4 4L19 7' })]) }})
</script>