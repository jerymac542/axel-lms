// Axios configuration
import axios from 'axios'

// Configure axios defaults
axios.defaults.headers.common['X-Requested-With'] = 'XMLHttpRequest'
axios.defaults.withCredentials = true

// Add request interceptor for CSRF token
axios.interceptors.request.use((config) => {
  const token = document.querySelector('meta[name="csrf-token"]')?.getAttribute('content')
  if (token) {
    config.headers['X-CSRF-TOKEN'] = token
  }
  return config
})

// Add response interceptor for error handling
axios.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Handle unauthorized - redirect to login
      if (!window.location.pathname.startsWith('/login')) {
        window.location.href = '/login'
      }
    }
    if (error.response?.status === 403) {
      // Handle forbidden
      console.error('Access forbidden:', error.config?.url)
    }
    if (error.response?.status >= 500) {
      // Handle server errors
      console.error('Server error:', error.config?.url)
    }
    return Promise.reject(error)
  }
)

// Export configured axios instance
export { axios }

// Ziggy route helper
import { ZiggyVue } from 'ziggy-js'
export { ZiggyVue }

// Type declarations
declare global {
  interface Window {
    axios: typeof axios
    Ziggy: any
  }
}

window.axios = axios