import { ref, computed, readonly } from 'vue'
import { useRouter } from 'vue-router'
import { useToast } from './useToast'
import type { User, AuthState } from '@/types/auth'

const authState = ref<AuthState>({
  user: null,
  isAuthenticated: false,
  isLoading: true,
  permissions: [],
  roles: [],
})

const router = useRouter()
const toast = useToast()

export function useAuth() {
  const user = computed(() => authState.value.user)
  const isAuthenticated = computed(() => authState.value.isAuthenticated)
  const isLoading = computed(() => authState.value.isLoading)
  const permissions = computed(() => authState.value.permissions)
  const roles = computed(() => authState.value.roles)

  const hasPermission = (permission: string): boolean => {
    return permissions.value.includes(permission) || permissions.value.includes('*')
  }

  const hasRole = (role: string): boolean => {
    return roles.value.includes(role) || roles.value.includes('admin')
  }

  const hasAnyRole = (roleList: string[]): boolean => {
    return roleList.some(role => hasRole(role))
  }

  const hasAllPermissions = (permissionList: string[]): boolean => {
    return permissionList.every(permission => hasPermission(permission))
  }

  const hasAnyPermission = (permissionList: string[]): boolean => {
    return permissionList.some(permission => hasPermission(permission))
  }

  const setAuth = (state: Partial<AuthState>) => {
    authState.value = { ...authState.value, ...state }
  }

  const login = async (credentials: { email: string; password: string; remember?: boolean }) => {
    authState.value.isLoading = true
    try {
      // This would typically call an API
      // const response = await axios.post('/login', credentials)
      // For now, we'll simulate
      await new Promise(resolve => setTimeout(resolve, 1000))
      
      // Mock user data - replace with actual API response
      authState.value = {
        user: {
          id: '1',
          email: credentials.email,
          name: 'Demo User',
          avatar_url: null,
          role: 'student',
          role_display: 'Student',
          stats: {
            active_courses: 3,
            badges_earned: 5,
            certificates_earned: 2,
            bookmarks_count: 10,
            unread_messages: 2,
            unread_discussions: 1,
          },
        },
        isAuthenticated: true,
        isLoading: false,
        permissions: ['course.read', 'course.enroll', 'assignment.submit'],
        roles: ['student'],
      }
      
      toast.success('Welcome back!')
      router.push('/dashboard')
    } catch (error) {
      authState.value.isLoading = false
      throw error
    }
  }

  const register = async (data: { name: string; email: string; password: string }) => {
    authState.value.isLoading = true
    try {
      await new Promise(resolve => setTimeout(resolve, 1000))
      
      authState.value = {
        user: {
          id: '1',
          email: data.email,
          name: data.name,
          avatar_url: null,
          role: 'student',
          role_display: 'Student',
          stats: {
            active_courses: 0,
            badges_earned: 0,
            certificates_earned: 0,
            bookmarks_count: 0,
            unread_messages: 0,
            unread_discussions: 0,
          },
        },
        isAuthenticated: true,
        isLoading: false,
        permissions: ['course.read', 'course.enroll', 'assignment.submit'],
        roles: ['student'],
      }
      
      toast.success('Account created successfully!')
      router.push('/dashboard')
    } catch (error) {
      authState.value.isLoading = false
      throw error
    }
  }

  const logout = async () => {
    try {
      // await axios.post('/logout')
      authState.value = {
        user: null,
        isAuthenticated: false,
        isLoading: false,
        permissions: [],
        roles: [],
      }
      toast.success('You have been signed out')
      router.push('/login')
    } catch (error) {
      console.error('Logout failed:', error)
      throw error
    }
  }

  const updateProfile = async (data: Partial<User>) => {
    if (!user.value) return
    
    try {
      // const response = await axios.put('/profile', data)
      authState.value.user = { ...user.value, ...data }
      toast.success('Profile updated')
    } catch (error) {
      toast.error('Failed to update profile')
      throw error
    }
  }

  const refreshUser = async () => {
    try {
      // const response = await axios.get('/user')
      // authState.value.user = response.data
    } catch (error) {
      console.error('Failed to refresh user:', error)
    }
  }

  // Initialize auth on app load
  const initAuth = async () => {
    try {
      // Check for existing session
      // const response = await axios.get('/user')
      // if (response.data) {
      //   authState.value = {
      //     user: response.data.user,
      //     isAuthenticated: true,
      //     isLoading: false,
      //     permissions: response.data.permissions || [],
      //     roles: response.data.roles || [],
      //   }
      // } else {
      //   authState.value.isLoading = false
      // }
      
      // Mock initialization
      authState.value.isLoading = false
    } catch (error) {
      authState.value.isLoading = false
    }
  }

  return {
    // State
    user: readonly(user),
    isAuthenticated: readonly(isAuthenticated),
    isLoading: readonly(isLoading),
    permissions: readonly(permissions),
    roles: readonly(roles),
    
    // Methods
    hasPermission,
    hasRole,
    hasAnyRole,
    hasAllPermissions,
    hasAnyPermission,
    login,
    register,
    logout,
    updateProfile,
    refreshUser,
    initAuth,
    setAuth,
  }
}

// Global auth instance
let globalAuth: ReturnType<typeof useAuth> | null = null

export function useGlobalAuth() {
  if (!globalAuth) {
    globalAuth = useAuth()
  }
  return globalAuth
}