import type { Directive, DirectiveBinding } from 'vue'

interface RippleBinding extends DirectiveBinding {
  value: boolean | { color?: string; centered?: boolean }
  modifiers: Record<string, boolean>
}

function createRipple(el: HTMLElement, event: MouseEvent, options: { color?: string; centered?: boolean } = {}) {
  const { color = 'currentColor', centered = false } = options
  
  const ripple = document.createElement('span')
  ripple.className = 'absolute rounded-full bg-current opacity-20 pointer-events-none animate-ripple'
  ripple.style.cssText = `
    background-color: ${color};
    transform: scale(0);
    animation: ripple 0.6s ease-out forwards;
  `
  
  const rect = el.getBoundingClientRect()
  const size = Math.max(rect.width, rect.height) * 2
  
  ripple.style.width = `${size}px`
  ripple.style.height = `${size}px`
  
  if (centered) {
    ripple.style.top = `${rect.height / 2 - size / 2}px`
    ripple.style.left = `${rect.width / 2 - size / 2}px`
  } else {
    const x = event.clientX - rect.left - size / 2
    const y = event.clientY - rect.top - size / 2
    ripple.style.top = `${y}px`
    ripple.style.left = `${x}px`
  }
  
  // Ensure relative positioning
  const computedStyle = window.getComputedStyle(el)
  if (computedStyle.position === 'static') {
    el.style.position = 'relative'
  }
  el.style.overflow = 'hidden'
  
  el.appendChild(ripple)
  
  // Clean up after animation
  setTimeout(() => {
    ripple.remove()
  }, 600)
}

export const vRipple: Directive<HTMLElement, boolean | { color?: string; centered?: boolean }> = {
  mounted(el, binding) {
    const enabled = binding.value === true || binding.value === undefined || 
      (typeof binding.value === 'object' && binding.value !== null)
    
    if (!enabled) return
    
    const options = typeof binding.value === 'object' ? binding.value : {}
    
    const handler = (event: MouseEvent) => {
      // Don't ripple on right click
      if (event.button === 2) return
      
      // Don't ripple on disabled elements
      if (el.hasAttribute('disabled') || el.getAttribute('aria-disabled') === 'true') return
      
      createRipple(el, event, options)
    }
    
    el.addEventListener('mousedown', handler)
    ;(el as any).__rippleHandler = handler
  },
  unmounted(el) {
    const handler = (el as any).__rippleHandler
    if (handler) {
      el.removeEventListener('mousedown', handler)
      delete (el as any).__rippleHandler
    }
  },
}

// Add CSS animation globally
if (typeof document !== 'undefined') {
  const style = document.createElement('style')
  style.textContent = `
    @keyframes ripple {
      0% {
        transform: scale(0);
        opacity: 0.3;
      }
      100% {
        transform: scale(1);
        opacity: 0;
      }
    }
    .animate-ripple {
      animation: ripple 0.6s ease-out forwards;
    }
  `
  document.head.appendChild(style)
}

export const ripple = {
  install(app: any) {
    app.directive('ripple', vRipple)
  },
}