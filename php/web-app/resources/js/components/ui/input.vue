<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import { computed } from 'vue'
import type { HTMLAttributes } from 'vue'

const inputVariants = cva(
  'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
  {
    variants: {
      variant: {
        default: '',
        error: 'border-destructive-500 focus-visible:ring-destructive-500',
        success: 'border-success-500 focus-visible:ring-success-500',
      },
      size: {
        default: 'h-10 px-3 text-sm',
        sm: 'h-8 px-2 text-xs',
        lg: 'h-12 px-4 text-base',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  }
)

interface Props extends VariantProps<typeof inputVariants> {
  type?: string
  modelValue?: string
  placeholder?: string
  disabled?: boolean
  required?: boolean
  readonly?: boolean
  autocomplete?: string
  name?: string
  id?: string
  ariaLabel?: string
  ariaDescribedBy?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  variant: 'default',
  size: 'default',
  disabled: false,
  required: false,
  readonly: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
  blur: [event: FocusEvent]
  focus: [event: FocusEvent]
  change: [event: Event]
  input: [event: Event]
}>()

const classes = computed(() => inputVariants({
  variant: props.variant,
  size: props.size,
}))

const inputId = computed(() => props.id || `input-${Math.random().toString(36).slice(2, 9)}`)
</script>

<template>
  <div class="relative">
    <input
      :id="inputId"
      :class="classes"
      :type="props.type"
      :value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      :required="required"
      :readonly="readonly"
      :autocomplete="autocomplete"
      :name="name"
      :aria-label="ariaLabel"
      :aria-describedby="ariaDescribedBy"
      :aria-invalid="variant === 'error'"
      @input="$emit('update:modelValue', $event.target.value)"
      @blur="$emit('blur', $event)"
      @focus="$emit('focus', $event)"
      @change="$emit('change', $event)"
    />
    <slot name="prefix" :id="inputId" />
    <slot name="suffix" :id="inputId" />
  </div>
</template>