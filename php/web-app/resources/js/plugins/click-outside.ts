import type { Directive, DirectiveBinding } from 'vue'

interface ClickOutsideBinding extends DirectiveBinding {
  value: (event: MouseEvent) => void
  modifiers: Record<string, boolean>
}

function onClickOutside(el: HTMLElement, event: MouseEvent, handler: (event: MouseEvent) => void) {
  if (!el.contains(event.target as Node)) {
    handler(event)
  }
}

export const vClickOutside: Directive<HTMLElement, (event: MouseEvent) => void> = {
  mounted(el, binding) {
    const handler = (event: MouseEvent) => {
      onClickOutside(el, event, binding.value)
    }
    
    // Store handler for cleanup
    ;(el as any).__clickOutsideHandler = handler
    
    // Use capture to catch events before they bubble
    document.addEventListener('click', handler, true)
    document.addEventListener('touchstart', handler, true)
  },
  unmounted(el) {
    const handler = (el as any).__clickOutsideHandler
    if (handler) {
      document.removeEventListener('click', handler, true)
      document.removeEventListener('touchstart', handler, true)
      delete (el as any).__clickOutsideHandler
    }
  },
}

export const clickOutside = {
  install(app: any) {
    app.directive('click-outside', vClickOutside)
  },
}