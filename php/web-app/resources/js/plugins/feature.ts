import type { Directive, DirectiveBinding } from 'vue'
import { useFeatureFlags } from '@/composables/useFeatureFlags'

const features = useFeatureFlagsAuto()

function checkFeature(binding: DirectiveBinding) {
  const { value } = binding
  
  if (!value) return true
  
  if (typeof value === 'string') {
    return features.isEnabled(value)
  }
  
  if (Array.isArray(value)) {
    // Check if ANY feature is enabled
    return value.some(f => features.isEnabled(f))
  }
  
  if (typeof value === 'object') {
    const { all, any } = value
    
    if (all && !all.every(f => features.isEnabled(f))) return false
    if (any && !any.some(f => features.isEnabled(f))) return false
    
    return true
  }
  
  return false
}

export const vFeature: Directive<HTMLElement, string | string[] | Record<string, any>> = {
  mounted(el, binding) {
    if (!checkFeature(binding)) {
      el.style.display = 'none'
    }
  },
  updated(el, binding) {
    if (!checkFeature(binding)) {
      el.style.display = 'none'
    } else {
      el.style.display = ''
    }
  },
}

export const feature = {
  install(app: any) {
    app.directive('feature', vFeature)
    app.config.globalProperties.$feature = features.isEnabled
    app.config.globalProperties.$featureFlags = features
  },
}