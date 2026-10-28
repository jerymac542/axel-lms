import { ref, computed, onMounted, onUnmounted } from 'vue'

export function useLocalStorage<T>(key: string, defaultValue: T) {
  const stored = ref<T | null>(null)
  const isLoaded = ref(false)

  const get = (): T => {
    if (!isLoaded.value) {
      if (typeof window !== 'undefined') {
        try {
          const item = window.localStorage.getItem(key)
          stored.value = item ? JSON.parse(item) : defaultValue
        } catch {
          stored.value = defaultValue
        }
      } else {
        stored.value = defaultValue
      }
      isLoaded.value = true
    }
    return stored.value as T
  }

  const set = (value: T) => {
    if (typeof window !== 'undefined') {
      try {
        window.localStorage.setItem(key, JSON.stringify(value))
        stored.value = value
      } catch (error) {
        console.error(`Failed to set localStorage key "${key}":`, error)
      }
    }
  }

  const remove = () => {
    if (typeof window !== 'undefined') {
      window.localStorage.removeItem(key)
      stored.value = null
    }
  }

  onMounted(() => {
    get()
  })

  return {
    value: computed(() => get()),
    get,
    set,
    remove,
  }
}

export function useSessionStorage<T>(key: string, defaultValue: T) {
  const stored = ref<T | null>(null)
  const isLoaded = ref(false)

  const get = (): T => {
    if (!isLoaded.value) {
      if (typeof window !== 'undefined') {
        try {
          const item = window.sessionStorage.getItem(key)
          stored.value = item ? JSON.parse(item) : defaultValue
        } catch {
          stored.value = defaultValue
        }
      } else {
        stored.value = defaultValue
      }
      isLoaded.value = true
    }
    return stored.value as T
  }

  const set = (value: T) => {
    if (typeof window !== 'undefined') {
      try {
        window.sessionStorage.setItem(key, JSON.stringify(value))
        stored.value = value
      } catch (error) {
        console.error(`Failed to set sessionStorage key "${key}":`, error)
      }
    }
  }

  const remove = () => {
    if (typeof window !== 'undefined') {
      window.sessionStorage.removeItem(key)
      stored.value = null
    }
  }

  onMounted(() => {
    get()
  })

  return {
    value: computed(() => get()),
    get,
    set,
    remove,
  }
}