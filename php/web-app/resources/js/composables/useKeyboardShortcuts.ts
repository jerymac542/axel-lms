import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useAuth } from './useAuth'

interface Shortcut {
  key: string
  ctrl?: boolean
  meta?: boolean
  shift?: boolean
  alt?: boolean
  action: () => void
  description?: string
  preventDefault?: boolean
  stopPropagation?: boolean
  target?: 'global' | 'input' | 'textarea'
}

export function useKeyboardShortcuts(shortcuts: Shortcut[] = []) {
  const auth = useAuth()
  const enabled = ref(true)

  const handleKeyDown = (event: KeyboardEvent) => {
    if (!enabled.value) return

    // Check if we should handle this event based on target
    const activeElement = document.activeElement
    const isInput = activeElement?.tagName === 'INPUT'
    const isTextarea = activeElement?.tagName === 'TEXTAREA'
    const isContentEditable = activeElement?.getAttribute('contenteditable') === 'true'

    for (const shortcut of shortcuts) {
      if (shortcut.target === 'input' && !isInput && !isTextarea) continue
      if (shortcut.target === 'textarea' && !isTextarea) continue
      if (shortcut.target === 'global' && (isInput || isTextarea || isContentEditable)) continue

      const keyMatches = event.key.toLowerCase() === shortcut.key.toLowerCase()
      const ctrlMatches = !!shortcut.ctrl === (event.ctrlKey || event.metaKey)
      const shiftMatches = !!shortcut.shift === event.shiftKey
      const altMatches = !!shortcut.alt === event.altKey

      if (keyMatches && ctrlMatches && shiftMatches && altMatches) {
        if (shortcut.preventDefault !== false) event.preventDefault()
        if (shortcut.stopPropagation) event.stopPropagation()
        shortcut.action()
        break
      }
    }
  }

  onMounted(() => {
    document.addEventListener('keydown', handleKeyDown)
  })

  onUnmounted(() => {
    document.removeEventListener('keydown', handleKeyDown)
  })

  const enable = () => { enabled.value = true }
  const disable = () => { enabled.value = false }
  const toggle = () => { enabled.value = !enabled.value }

  return {
    enabled: computed(() => enabled.value),
    enable,
    disable,
    toggle,
  }
}

// Predefined shortcuts for the app
export const defaultShortcuts = (auth: ReturnType<typeof useAuth>) => [
  {
    key: 'k',
    meta: true,
    action: () => {
      // Open command palette
      const event = new CustomEvent('open-command-palette')
      window.dispatchEvent(event)
    },
    description: 'Open command palette',
  },
  {
    key: '/',
    meta: true,
    action: () => {
      // Focus search
      const searchInput = document.querySelector('[data-search-input]') as HTMLInputElement
      searchInput?.focus()
    },
    description: 'Focus search',
    target: 'global' as const,
  },
  {
    key: 'n',
    ctrl: true,
    action: () => {
      // New course/lesson (if instructor)
      if (auth.hasRole('instructor')) {
        const event = new CustomEvent('create-new-content')
        window.dispatchEvent(event)
      }
    },
    description: 'Create new content',
  },
  {
    key: 's',
    ctrl: true,
    action: () => {
      // Save (if in editor)
      const event = new CustomEvent('save-content')
      window.dispatchEvent(event)
    },
    description: 'Save content',
  },
  {
    key: 'Escape',
    action: () => {
      // Close modals, dropdowns, etc.
      const event = new CustomEvent('close-all-overlays')
      window.dispatchEvent(event)
    },
    description: 'Close overlays',
  },
  {
    key: '?',
    shift: true,
    action: () => {
      // Show keyboard shortcuts help
      const event = new CustomEvent('show-shortcuts-help')
      window.dispatchEvent(event)
    },
    description: 'Show shortcuts help',
  },
]