import type { Directive, DirectiveBinding } from 'vue'

interface AutoFocusBinding extends DirectiveBinding {
  value: boolean | number
  modifiers: Record<string, boolean>
}

export const vAutoFocus: Directive<HTMLElement, boolean | number> = {
  mounted(el, binding) {
    const shouldFocus = binding.value === true || binding.value === undefined
    
    if (!shouldFocus) return
    
    const delay = typeof binding.value === 'number' ? binding.value : 0
    
    const focus = () => {
      // Focus the element or first focusable child
      if (el.matches('input, select, textarea, button, [href], [tabindex]')) {
        el.focus()
      } else {
        const focusable = el.querySelector<HTMLElement>(
          'input, select, textarea, button, [href], [tabindex]:not([tabindex="-1"])'
        )
        focusable?.focus()
      }
      
      // Prevent scroll on focus if modifier is present
      if (binding.modifiers.noScroll) {
        ;(el as any).scrollIntoView?.({ preventScroll: true })
      }
    }
    
    if (delay > 0) {
      setTimeout(focus, delay)
    } else {
      // Use nextTick equivalent - requestAnimationFrame
      requestAnimationFrame(focus)
    }
  },
}

export const autoFocus = {
  install(app: any) {
    app.directive('auto-focus', vAutoFocus)
  },
}