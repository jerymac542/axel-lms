import type { Directive, DirectiveBinding } from 'vue'
import { usePermissions } from '@/composables/usePermissions'

const permissions = usePermissions()

function checkPermission(binding: DirectiveBinding) {
  const { value } = binding
  
  if (!value) return true
  
  if (typeof value === 'string') {
    return permissions.can(value)
  }
  
  if (Array.isArray(value)) {
    // Check if user has ANY of the permissions
    return value.some(p => permissions.can(p))
  }
  
  if (typeof value === 'object') {
    // Complex permission object
    const { all, any, role, roles } = value
    
    if (all && !permissions.canAll(...all)) return false
    if (any && !permissions.canAny(...any)) return false
    if (role && !permissions.isRole(role)) return false
    if (roles && !permissions.isAnyRole(...roles)) return false
    
    return true
  }
  
  return false
}

export const vPermission: Directive<HTMLElement, string | string[] | Record<string, any>> = {
  mounted(el, binding) {
    if (!checkPermission(binding)) {
      el.style.display = 'none'
      // Or remove entirely: el.remove()
    }
  },
  updated(el, binding) {
    if (!checkPermission(binding)) {
      el.style.display = 'none'
    } else {
      el.style.display = ''
    }
  },
}

// Global property for use in templates
export const permission = {
  install(app: any) {
    app.directive('permission', vPermission)
    app.config.globalProperties.$can = permissions.can
    app.config.globalProperties.$canAll = permissions.canAll
    app.config.globalProperties.$canAny = permissions.canAny
    app.config.globalProperties.$isRole = permissions.isRole
    app.config.globalProperties.$isAnyRole = permissions.isAnyRole
  },
}