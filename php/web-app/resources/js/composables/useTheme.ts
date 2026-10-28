import { ref, computed, onMounted, watch } from 'vue'
import { useLocalStorage } from './useLocalStorage'

type Theme = 'light' | 'dark' | 'system'

const theme = ref<Theme>('system')
const resolvedTheme = ref<'light' | 'dark'>('light')

const { get: getStoredTheme, set: setStoredTheme } = useLocalStorage<Theme>('theme', 'system')

export function useTheme() {
  const init = () => {
    // Get stored theme
    const stored = getStoredTheme()
    theme.value = stored
    
    // Apply theme
    applyTheme(stored)
    
    // Listen for system theme changes
    if (typeof window !== 'undefined') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      mediaQuery.addEventListener('change', handleSystemThemeChange)
      
      // Initial system theme check
      if (stored === 'system') {
        resolvedTheme.value = mediaQuery.matches ? 'dark' : 'light'
      }
    }
  }

  const handleSystemThemeChange = (e: MediaQueryListEvent) => {
    if (theme.value === 'system') {
      resolvedTheme.value = e.matches ? 'dark' : 'light'
      applyTheme('system')
    }
  }

  const applyTheme = (newTheme: Theme) => {
    if (typeof document === 'undefined') return
    
    const root = document.documentElement
    
    if (newTheme === 'system') {
      const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      root.classList.toggle('dark', isDark)
      resolvedTheme.value = isDark ? 'dark' : 'light'
    } else {
      root.classList.toggle('dark', newTheme === 'dark')
      resolvedTheme.value = newTheme
    }
  }

  const setTheme = (newTheme: Theme) => {
    theme.value = newTheme
    setStoredTheme(newTheme)
    applyTheme(newTheme)
  }

  const toggleTheme = () => {
    const themes: Theme[] = ['light', 'dark', 'system']
    const currentIndex = themes.indexOf(theme.value)
    const nextIndex = (currentIndex + 1) % themes.length
    setTheme(themes[nextIndex])
  }

  const setLight = () => setTheme('light')
  const setDark = () => setTheme('dark')
  const setSystem = () => setTheme('system')

  // Watch for theme changes
  watch(theme, (newTheme) => {
    applyTheme(newTheme)
  })

  onMounted(() => {
    init()
  })

  return {
    theme: computed(() => theme.value),
    resolvedTheme: computed(() => resolvedTheme.value),
    isDark: computed(() => resolvedTheme.value === 'dark'),
    isSystem: computed(() => theme.value === 'system'),
    setTheme,
    toggleTheme,
    setLight,
    setDark,
    setSystem,
    init,
  }
}