import type { Directive, DirectiveBinding } from 'vue'

interface TooltipBinding extends DirectiveBinding {
  value: string | { content: string; placement?: 'top' | 'bottom' | 'left' | 'right'; delay?: number }
  modifiers: Record<string, boolean>
}

let tooltipInstance: HTMLElement | null = null
let currentTarget: HTMLElement | null = null
let hideTimeout: ReturnType<typeof setTimeout> | null = null
let showTimeout: ReturnType<typeof setTimeout> | null = null

function createTooltip() {
  if (tooltipInstance) return tooltipInstance
  
  tooltipInstance = document.createElement('div')
  tooltipInstance.className = 'fixed z-[10000] px-3 py-1.5 text-sm text-popover-foreground bg-popover border rounded-md shadow-lg pointer-events-none opacity-0 transition-opacity duration-200'
  tooltipInstance.setAttribute('role', 'tooltip')
  document.body.appendChild(tooltipInstance)
  
  return tooltipInstance
}

function showTooltip(el: HTMLElement, binding: TooltipBinding) {
  if (showTimeout) clearTimeout(showTimeout)
  if (hideTimeout) clearTimeout(hideTimeout)
  
  const content = typeof binding.value === 'string' ? binding.value : binding.value?.content
  const placement = typeof binding.value === 'object' ? binding.value?.placement || 'top' : 'top'
  const delay = typeof binding.value === 'object' ? binding.value?.delay || 200 : 200
  
  if (!content) return
  
  showTimeout = setTimeout(() => {
    const tooltip = createTooltip()
    tooltip.textContent = content
    tooltip.style.opacity = '0'
    
    // Position tooltip
    const rect = el.getBoundingClientRect()
    const tooltipRect = tooltip.getBoundingClientRect()
    
    let top = 0
    let left = 0
    
    switch (placement) {
      case 'top':
        top = rect.top - tooltipRect.height - 8
        left = rect.left + (rect.width - tooltipRect.width) / 2
        break
      case 'bottom':
        top = rect.bottom + 8
        left = rect.left + (rect.width - tooltipRect.width) / 2
        break
      case 'left':
        top = rect.top + (rect.height - tooltipRect.height) / 2
        left = rect.left - tooltipRect.width - 8
        break
      case 'right':
        top = rect.top + (rect.height - tooltipRect.height) / 2
        left = rect.right + 8
        break
    }
    
    // Keep in viewport
    top = Math.max(8, Math.min(top, window.innerHeight - tooltipRect.height - 8))
    left = Math.max(8, Math.min(left, window.innerWidth - tooltipRect.width - 8))
    
    tooltip.style.top = `${top}px`
    tooltip.style.left = `${left}px`
    tooltip.style.opacity = '1'
    
    currentTarget = el
  }, delay)
}

function hideTooltip() {
  if (showTimeout) clearTimeout(showTimeout)
  
  hideTimeout = setTimeout(() => {
    if (tooltipInstance) {
      tooltipInstance.style.opacity = '0'
      currentTarget = null
    }
  }, 100)
}

export const vTooltip: Directive<HTMLElement, string | { content: string; placement?: 'top' | 'bottom' | 'left' | 'right'; delay?: number }> = {
  mounted(el, binding) {
    el.addEventListener('mouseenter', () => showTooltip(el, binding as TooltipBinding))
    el.addEventListener('mouseleave', hideTooltip)
    el.addEventListener('focus', () => showTooltip(el, binding as TooltipBinding))
    el.addEventListener('blur', hideTooltip)
  },
  unmounted(el) {
    el.removeEventListener('mouseenter', () => {})
    el.removeEventListener('mouseleave', hideTooltip)
    el.removeEventListener('focus', () => {})
    el.removeEventListener('blur', hideTooltip)
    
    if (currentTarget === el) {
      hideTooltip()
    }
  },
}

export const tooltip = {
  install(app: any) {
    app.directive('tooltip', vTooltip)
  },
}