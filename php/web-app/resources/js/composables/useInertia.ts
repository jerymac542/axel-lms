import { ref, computed } from 'vue'
import { router } from '@inertiajs/vue3'

interface InertiaPageProps {
  url: string
  component: string
  props: Record<string, any>
  version: string
  clearHistory: boolean
  encryptHistory: boolean
  preserveScroll: boolean
  preserveState: boolean
  replace: boolean
  method: string
  headers: Record<string, string>
  onSuccess: (page: InertiaPageProps) => void
  onError: (errors: Record<string, string>) => void
  onFinish: () => void
}

export function useInertia() {
  const isNavigating = ref(false)
  const progress = ref(0)
  const recentPages = ref<InertiaPageProps[]>([])

  const visit = (url: string, options: Partial<InertiaPageProps> = {}) => {
    isNavigating.value = true
    progress.value = 0
    
    router.visit(url, {
      ...options,
      onStart: () => { isNavigating.value = true },
      onProgress: (p) => { progress.value = p },
      onFinish: () => { 
        isNavigating.value = false
        progress.value = 100
        setTimeout(() => { progress.value = 0 }, 500)
      },
    })
  }

  const get = (url: string, options = {}) => visit(url, { method: 'get', ...options })
  const post = (url: string, data: any, options = {}) => visit(url, { method: 'post', data, ...options })
  const put = (url: string, data: any, options = {}) => visit(url, { method: 'put', data, ...options })
  const patch = (url: string, data: any, options = {}) => visit(url, { method: 'patch', data, ...options })
  const del = (url: string, options = {}) => visit(url, { method: 'delete', ...options })

  const reload = (options = {}) => router.reload(options)
  const back = () => router.back()
  const forward = () => router.forward()
  const remember = (key: string, defaultValue: any) => router.remember(key, defaultValue)
  const restore = (key: string) => router.restore(key)

  const currentPage = computed(() => ({
    url: router.url(),
    component: router.page().component,
    props: router.page().props,
  }))

  const isCurrentPage = (url: string) => router.url() === url

  return {
    visit,
    get,
    post,
    put,
    patch,
    delete: del,
    reload,
    back,
    forward,
    remember,
    restore,
    currentPage,
    isCurrentPage,
    isNavigating: computed(() => isNavigating.value),
    progress: computed(() => progress.value),
  }
}