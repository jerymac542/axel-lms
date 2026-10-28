import { ref, computed } from 'vue'

interface Toast {
  id: string
  type: 'success' | 'error' | 'warning' | 'info' | 'loading'
  title: string
  description?: string
  duration?: number
  dismissible?: boolean
  action?: {
    label: string
    onClick: () => void
  }
}

const toasts = ref<Toast[]>([])
const toastId = 0

const generateId = () => `toast-${++toastId}-${Date.now()}`

export function useToast() {
  const addToast = (toast: Omit<Toast, 'id'>) => {
    const id = generateId()
    const newToast: Toast = {
      id,
      dismissible: true,
      duration: toast.type === 'loading' ? Infinity : 5000,
      ...toast,
    }
    
    toasts.value = [...toasts.value, newToast]
    
    // Auto dismiss
    if (newToast.duration && newToast.duration !== Infinity) {
      setTimeout(() => {
        removeToast(id)
      }, newToast.duration)
    }
    
    return id
  }

  const removeToast = (id: string) => {
    toasts.value = toasts.value.filter(t => t.id !== id)
  }

  const clearToasts = () => {
    toasts.value = []
  }

  const updateToast = (id: string, updates: Partial<Toast>) => {
    const index = toasts.value.findIndex(t => t.id === id)
    if (index !== -1) {
      toasts.value[index] = { ...toasts.value[index], ...updates }
    }
  }

  // Convenience methods
  const success = (title: string, description?: string, options?: Partial<Toast>) => {
    return addToast({ type: 'success', title, description, ...options })
  }

  const error = (title: string, description?: string, options?: Partial<Toast>) => {
    return addToast({ type: 'error', title, description, duration: 8000, ...options })
  }

  const warning = (title: string, description?: string, options?: Partial<Toast>) => {
    return addToast({ type: 'warning', title, description, ...options })
  }

  const info = (title: string, description?: string, options?: Partial<Toast>) => {
    return addToast({ type: 'info', title, description, ...options })
  }

  const loading = (title: string, description?: string, options?: Partial<Toast>) => {
    return addToast({ type: 'loading', title, description, duration: Infinity, dismissible: false, ...options })
  }

  const promise = async <T>(
    promise: Promise<T>,
    messages: {
      loading: string
      success: string | ((data: T) => string)
      error: string | ((error: Error) => string)
    }
  ) => {
    const loadingId = loading(messages.loading)
    
    try {
      const data = await promise
      removeToast(loadingId)
      const successMessage = typeof messages.success === 'function' ? messages.success(data) : messages.success
      success(successMessage)
      return data
    } catch (err) {
      removeToast(loadingId)
      const errorMessage = typeof messages.error === 'function' ? messages.error(err as Error) : messages.error
      error(errorMessage)
      throw err
    }
  }

  return {
    toasts: computed(() => toasts.value),
    addToast,
    removeToast,
    clearToasts,
    updateToast,
    success,
    error,
    warning,
    info,
    loading,
    promise,
  }
}