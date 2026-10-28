import type { Directive, DirectiveBinding } from 'vue'

interface FocusTrapBinding extends DirectiveBinding {
  value: boolean
  modifiers: Record<string, boolean>
}

let lastFocusedElement: HTMLElement | null = null
let focusableElementsSelector = 'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'

function getFocusableElements(container: HTMLElement): HTMLElement[] {
  return Array.from(container.querySelectorAll<HTMLElement>(focusableElementsSelector))
    .filter(el => !el.hasAttribute('disabled') && !el.getAttribute('aria-hidden'))
}

function handleTabKey(event: KeyboardEvent, container: HTMLElement) {
  if (event.key !== 'Tab') return
  
  const focusableElements = getFocusableElements(container)
  if (focusableElements.length === 0) return
  
  const firstElement = focusableElements[0]
  const lastElement = focusableElements[focusableElements.length - 1]
  
  if (event.shiftKey) {
    if (document.activeElement === firstElement) {
      event.preventDefault()
      lastElement.focus()
    }
  } else {
    if (document.activeElement === lastElement) {
      event.preventDefault()
      firstElement.focus()
    }
  }
}

function handleEscapeKey(event: KeyboardEvent, binding: FocusTrapBinding) {
  if (event.key === 'Escape' && binding.modifiers.closeOnEscape !== false) {
    // Dispatch custom event for parent to handle
    const closeEvent = new CustomEvent('focus-trap-escape', { detail: { element: binding.instance?.$el } })
    document.dispatchEvent(closeEvent)
  }
}

export const vFocusTrap: Directive<HTMLElement, boolean> = {
  mounted(el, binding) {
    if (!binding.value) return
    
    // Store last focused element
    lastFocusedElement = document.activeElement as HTMLElement
    
    // Focus first focusable element
    const focusableElements = getFocusableElements(el)
    if (focusableElements.length > 0) {
      focusableElements[0].focus()
    }
    
    // Handle keydown
    const keydownHandler = (event: KeyboardEvent) => {
      handleTabKey(event, el)
      handleEscapeKey(event, binding as FocusTrapBinding)
    }
    
    el.addEventListener('keydown', keydownHandler)
    ;(el as any).__focusTrapKeydownHandler = keydownHandler
    
    // Prevent focus from leaving
    const focusinHandler = (event: FocusEvent) => {
      if (!el.contains(event.target as Node)) {
        event.preventDefault()
        const focusableElements = getFocusableElements(el)
        if (focusableElements.length > 0) {
          focusableElements[0].focus()
        }
      }
    }
    
    document.addEventListener('focusin', focusinHandler)
    ;(el as any).__focusTrapFocusinHandler = focusinHandler
  },
  updated(el, binding) {
    if (binding.value === binding.oldValue) return
    
    if (binding.value) {
      // Enable focus trap
      const focusableElements = getFocusableElements(el)
      if (focusableElements.length > 0) {
        focusableElements[0].focus()
      }
    } else {
      // Disable focus trap - restore focus
      if (lastFocusedElement) {
        lastFocusedElement.focus()
        lastFocusedElement = null
      }
    }
  },
  unmounted(el) {
    // Restore focus
    if (lastFocusedElement) {
      lastFocusedElement.focus()
      lastFocusedElement = null
    }
    
    const keydownHandler = (el as any).__focusTrapKeydownHandler
    if (keydownHandler) {
      el.removeEventListener('keydown', keydownHandler)
      delete (el as any).__focusTrapKeydownHandler
    }
    
    const focusinHandler = (el as any).__focusTrapFocusinHandler
    if (focusinHandler) {
      document.removeEventListener('focusin', focusinHandler)
      delete (el as any).__focusTrapFocusinHandler
    }
  },
}

export const focusTrap = {
  install(app: any) {
    app.directive('focus-trap', vFocusTrap)
  },
}