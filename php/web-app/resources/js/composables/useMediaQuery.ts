import { ref, computed, onMounted, onUnmounted } from 'vue'

export function useMediaQuery(query: string) {
  const matches = ref(false)
  let mediaQuery: MediaQueryList | null = null

  const updateMatches = () => {
    if (mediaQuery) {
      matches.value = mediaQuery.matches
    }
  }

  onMounted(() => {
    if (typeof window !== 'undefined') {
      mediaQuery = window.matchMedia(query)
      matches.value = mediaQuery.matches
      mediaQuery.addEventListener('change', updateMatches)
    }
  })

  onUnmounted(() => {
    if (mediaQuery) {
      mediaQuery.removeEventListener('change', updateMatches)
    }
  })

  return computed(() => matches.value)
}

// Breakpoint helpers
export const useIsMobile = () => useMediaQuery('(max-width: 639px)')
export const useIsTablet = () => useMediaQuery('(min-width: 640px) and (max-width: 1023px)')
export const useIsDesktop = () => useMediaQuery('(min-width: 1024px)')
export const useIsWide = () => useMediaQuery('(min-width: 1280px)')
export const useIsUltraWide = () => useMediaQuery('(min-width: 1536px)')
export const usePrefersDark = () => useMediaQuery('(prefers-color-scheme: dark)')
export const usePrefersReducedMotion = () => useMediaQuery('(prefers-reduced-motion: reduce)')
export const usePrefersHighContrast = () => useMediaQuery('(prefers-contrast: more)')
export const useIsPortrait = () => useMediaQuery('(orientation: portrait)')
export const useIsLandscape = () => useMediaQuery('(orientation: landscape)')
export const useIsHover = () => useMediaQuery('(hover: hover)')
export const useIsPointerFine = () => useMediaQuery('(pointer: fine)')