import { computed, ref } from 'vue'
import { useAuth } from './useAuth'

export function usePermissions() {
  const auth = useAuth()

  const permissions = computed(() => auth.permissions.value)
  const roles = computed(() => auth.roles.value)

  // Single permission check
  const can = (permission: string): boolean => {
    return auth.hasPermission(permission)
  }

  // Multiple permissions - all required
  const canAll = (...permissions: string[]): boolean => {
    return auth.hasAllPermissions(permissions)
  }

  // Multiple permissions - any required
  const canAny = (...permissions: string[]): boolean => {
    return auth.hasAnyPermission(permissions)
  }

  // Role checks
  const isRole = (role: string): boolean => auth.hasRole(role)
  const isAnyRole = (...roles: string[]): boolean => auth.hasAnyRole(roles)

  // Resource-specific permissions
  const canView = (resource: string) => can(`${resource}.read`)
  const canCreate = (resource: string) => can(`${resource}.create`)
  const canUpdate = (resource: string) => can(`${resource}.update`)
  const canDelete = (resource: string) => can(`${resource}.delete`)
  const canManage = (resource: string) => can(`${resource}.manage`)

  // Course-specific
  const canEnroll = (courseId?: string) => can('enrollment.create')
  const canTeach = (courseId?: string) => can('course.teach')
  const canGrade = (courseId?: string) => can('assignment.grade')
  const canPublish = (courseId?: string) => can('course.publish')

  // Admin permissions
  const isAdmin = () => isRole('admin')
  const isInstructor = () => isRole('instructor')
  const isStudent = () => isRole('student')
  const isOrgAdmin = () => isRole('org_admin')

  // Conditional rendering helpers
  const ifCan = <T>(permission: string, trueValue: T, falseValue?: T): T | undefined => {
    return can(permission) ? trueValue : falseValue
  }

  const ifRole = <T>(role: string, trueValue: T, falseValue?: T): T | undefined => {
    return isRole(role) ? trueValue : falseValue
  }

  const ifAdmin = <T>(trueValue: T, falseValue?: T): T | undefined => {
    return isAdmin() ? trueValue : falseValue
  }

  return {
    // State
    permissions,
    roles,

    // Checks
    can,
    canAll,
    canAny,
    isRole,
    isAnyRole,

    // Resource CRUD
    canView,
    canCreate,
    canUpdate,
    canDelete,
    canManage,

    // Course-specific
    canEnroll,
    canTeach,
    canGrade,
    canPublish,

    // Role checks
    isAdmin,
    isInstructor,
    isStudent,
    isOrgAdmin,

    // Conditional rendering
    ifCan,
    ifRole,
    ifAdmin,
  }
}