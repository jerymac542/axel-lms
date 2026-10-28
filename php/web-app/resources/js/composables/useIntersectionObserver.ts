import { ref, computed, onMounted, onUnmounted } from 'vue'

interface IntersectionObserverOptions {
  root?: Element | null
  rootMargin?: string
  threshold?: number | number[]
  triggerOnce?: boolean
}

export function useIntersectionObserver(
  callback: (entries: IntersectionObserverEntry[], observer: IntersectionObserver) => void,
  options: IntersectionObserverOptions = {}
) {
  const target = ref<Element | null>(null)
  let observer: IntersectionObserver | null = null

  const { root = null, rootMargin = '0px', threshold = 0, triggerOnce = false } = options

  const observe = (element: Element) => {
    if (observer) {
      observer.disconnect()
    }

    observer = new IntersectionObserver(
      (entries, obs) => {
        callback(entries, obs)
        
        if (triggerOnce && entries.some(e => e.isIntersecting)) {
          obs.disconnect()
        }
      },
      { root, rootMargin, threshold }
    )

    observer.observe(element)
    target.value = element
  }

  const unobserve = () => {
    if (observer && target.value) {
      observer.unobserve(target.value)
      target.value = null
    }
  }

  const disconnect = () => {
    if (observer) {
      observer.disconnect()
      observer = null
      target.value = null
    }
  }

  onUnmounted(() => {
    disconnect()
  })

  return {
    observe,
    unobserve,
    disconnect,
    target: computed(() => target.value),
  }
}

// Specialized hooks
export function useInView(options: IntersectionObserverOptions = {}) {
  const isInView = ref(false)
  
  const { observe, unobserve, disconnect, target } = useIntersectionObserver(
    (entries) => {
      isInView.value = entries[0]?.isIntersecting ?? false
    },
    { triggerOnce: true, ...options }
  )

  return { observe, unobserve, disconnect, target, isInView }
}

export function useScrollSpy(elements: Element[], options: IntersectionObserverOptions = {}) {
  const activeElement = ref<Element | null>(null)

  const { observe, disconnect } = useIntersectionObserver(
    (entries) => {
      const visibleEntries = entries.filter(e => e.isIntersecting)
      if (visibleEntries.length > 0) {
        const mostVisible = visibleEntries.reduce((max, entry) => 
          entry.intersectionRatio > max.intersectionRatio ? entry : max
        )
        activeElement.value = mostVisible.target
      }
    },
    { threshold: [0, 0.25, 0.5, 0.75, 1], ...options }
  )

  onMounted(() => {
    elements.forEach(el => observe(el))
  })

  onUnmounted(() => {
    disconnect()
  })

  return { activeElement, observe, disconnect }
}