<script setup lang="ts">
import { ref, computed } from 'vue'
import { cn } from '@/utils/cn'
import { ChevronDownIcon } from '@/components/ui/icons'

interface Props {
  modelValue: boolean
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

const isOpen = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
})

const toggle = () => {
  if (!props.disabled) {
    isOpen.value = !isOpen.value
  }
}
</script>

<template>
  <div :class="cn('overflow-hidden transition-all duration-300')">
    <button
      @click="toggle"
      :disabled="disabled"
      :aria-expanded="isOpen"
      :class="cn(
        'w-full flex items-center justify-between px-2 py-1.5 rounded-lg text-sm font-medium',
        'text-muted-foreground hover:text-foreground hover:bg-accent',
        'transition-colors disabled:opacity-50 disabled:pointer-events-none',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring'
      )"
    >
      <slot name="trigger" />
      <ChevronDownIcon 
        class="h-4 w-4 flex-shrink-0 transition-transform duration-200" 
        :class="{ 'rotate-180': isOpen }"
      />
    </button>

    <div 
      v-show="isOpen"
      class="overflow-hidden"
      role="region"
      aria-expanded="isOpen"
    >
      <div class="animate-slide-down-2">
        <slot />
      </div>
    </div>
  </div>
</template>