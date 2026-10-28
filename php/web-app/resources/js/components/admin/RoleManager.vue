<script setup lang="ts">
import { ref, computed } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from '@/components/ui/table'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from '@/components/ui/dialog'
import { Checkbox } from '@/components/ui/checkbox'
import { 
  PlusIcon, SearchIcon, ShieldIcon, KeyIcon, UserIcon,
  EditIcon, TrashIcon, CopyIcon, EyeIcon, ChevronDownIcon,
  LockIcon, UnlockIcon, UsersIcon, ShieldCheckIcon,
  ChevronRightIcon, ChevronLeftIcon
} from '@/components/ui/icons'
import { useToast } from '@/composables/useToast'

interface Permission {
  id: string
  name: string
  description: string
  resource: string
  action: string
  category: string
}

interface Role {
  id: string
  name: string
  description: string
  permissions: string[]
  isSystem: boolean
  userCount: number
  createdAt: string
  updatedAt: string
}

const toast = useToast()

// Mock permissions
const allPermissions: Permission[] = [
  // User management
  { id: 'user.create', name: 'Create Users', description: 'Create new user accounts', resource: 'user', action: 'create', category: 'User Management' },
  { id: 'user.read', name: 'View Users', description: 'View user profiles and lists', resource: 'user', action: 'read', category: 'User Management' },
  { id: 'user.update', name: 'Update Users', description: 'Modify user profiles', resource: 'user', action: 'update', category: 'User Management' },
  { id: 'user.delete', name: 'Delete Users', description: 'Remove user accounts', resource: 'user', action: 'delete', category: 'User Management' },
  { id: 'user.list', name: 'List Users', description: 'Access user listings', resource: 'user', action: 'list', category: 'User Management' },
  
  // Course management
  { id: 'course.create', name: 'Create Courses', description: 'Create new courses', resource: 'course', action: 'create', category: 'Course Management' },
  { id: 'course.read', name: 'View Courses', description: 'View course content', resource: 'course', action: 'read', category: 'Course Management' },
  { id: 'course.update', name: 'Update Courses', description: 'Modify course content', resource: 'course', action: 'update', category: 'Course Management' },
  { id: 'course.delete', name: 'Delete Courses', description: 'Remove courses', resource: 'course', action: 'delete', category: 'Course Management' },
  { id: 'course.publish', name: 'Publish Courses', description: 'Publish/unpublish courses', resource: 'course', action: 'publish', category: 'Course Management' },
  { id: 'course.manage', name: 'Manage Courses', description: 'Full course administration', resource: 'course', action: 'manage', category: 'Course Management' },
  
  // Enrollment
  { id: 'enrollment.create', name: 'Create Enrollments', description: 'Enroll users in courses', resource: 'enrollment', action: 'create', category: 'Enrollment' },
  { id: 'enrollment.read', name: 'View Enrollments', description: 'View enrollment data', resource: 'enrollment', action: 'read', category: 'Enrollment' },
  { id: 'enrollment.update', name: 'Update Enrollments', description: 'Modify enrollments', resource: 'enrollment', action: 'update', category: 'Enrollment' },
  { id: 'enrollment.delete', name: 'Delete Enrollments', description: 'Remove enrollments', resource: 'enrollment', action: 'delete', category: 'Enrollment' },
  
  // Assignment & Grading
  { id: 'assignment.create', name: 'Create Assignments', description: 'Create new assignments', resource: 'assignment', action: 'create', category: 'Assessment' },
  { id: 'assignment.read', name: 'View Assignments', description: 'View assignments', resource: 'assignment', action: 'read', category: 'Assessment' },
  { id: 'assignment.update', name: 'Update Assignments', description: 'Modify assignments', resource: 'assignment', action: 'update', category: 'Assessment' },
  { id: 'assignment.delete', name: 'Delete Assignments', description: 'Remove assignments', resource: 'assignment', action: 'delete', category: 'Assessment' },
  { id: 'assignment.grade', name: 'Grade Assignments', description: 'Grade student submissions', resource: 'assignment', action: 'grade', category: 'Assessment' },
  
  // Analytics
  { id: 'analytics.read', name: 'View Analytics', description: 'Access analytics dashboards', resource: 'analytics', action: 'read', category: 'Analytics' },
  { id: 'analytics.export', name: 'Export Analytics', description: 'Export analytics data', resource: 'analytics', action: 'export', category: 'Analytics' },
  
  // Settings
  { id: 'settings.read', name: 'View Settings', description: 'View system settings', resource: 'settings', action: 'read', category: 'Settings' },
  { id: 'settings.update', name: 'Update Settings', description: 'Modify system settings', resource: 'settings', action: 'update', category: 'Settings' },
  
  // Admin
  { id: 'admin.access', name: 'Admin Access', description: 'Access admin panel', resource: 'admin', action: 'access', category: 'Administration' },
  { id: 'admin.users', name: 'Manage Users', description: 'Manage user accounts', resource: 'admin', action: 'users', category: 'Administration' },
  { id: 'admin.tenants', name: 'Manage Tenants', description: 'Manage tenant accounts', resource: 'admin', action: 'tenants', category: 'Administration' },
  { id: 'admin.system', name: 'System Admin', description: 'Full system administration', resource: 'admin', action: 'system', category: 'Administration' },
]

const categories = [...new Set(allPermissions.map(p => p.category))].sort()

const permissionsByCategory = computed(() => {
  const grouped: Record<string, Permission[]> = {}
  allPermissions.forEach(p => {
    if (!grouped[p.category]) grouped[p.category] = []
    grouped[p.category].push(p)
  })
  return grouped
}

const roles = ref<Role[]>([
  {
    id: 'role_super_admin',
    name: 'Super Admin',
    description: 'Full system access across all tenants',
    permissions: allPermissions.map(p => p.id),
    isSystem: true,
    userCount: 2,
    createdAt: '2023-01-01',
    updatedAt: '2024-01-15',
  },
  {
    id: 'role_admin',
    name: 'Admin',
    description: 'Full tenant administration access',
    permissions: allPermissions.filter(p => !['admin.tenants', 'admin.system'].includes(p.id)).map(p => p.id),
    isSystem: true,
    userCount: 15,
    createdAt: '2023-01-01',
    updatedAt: '2024-01-15',
  },
  {
    id: 'role_org_admin',
    name: 'Organization Admin',
    description: 'Manage organization-level settings and users',
    permissions: allPermissions.filter(p => ['user.read', 'user.list', 'course.read', 'course.create', 'course.update', 'enrollment.read', 'enrollment.create', 'analytics.read'].includes(p.id)).map(p => p.id),
    isSystem: true,
    userCount: 45,
    createdAt: '2023-01-01',
    updatedAt: '2024-01-15',
  },
  {
    id: 'role_instructor',
    name: 'Instructor',
    description: 'Create and manage courses, grade assignments',
    permissions: allPermissions.filter(p => ['course.read', 'course.create', 'course.update', 'course.publish', 'assignment.create', 'assignment.read', 'assignment.update', 'assignment.grade', 'enrollment.read', 'analytics.read'].includes(p.id)).map(p => p.id),
    isSystem: true,
    userCount: 120,
    createdAt: '2023-01-01',
    updatedAt: '2024-01-15',
  },
  {
    id: 'role_student',
    name: 'Student',
    description: 'Access courses, submit assignments, view grades',
    permissions: allPermissions.filter(p => ['course.read', 'enrollment.create', 'enrollment.read', 'assignment.read', 'assignment.update'].includes(p.id)).map(p => p.id),
    isSystem: true,
    userCount: 5000,
    createdAt: '2023-01-01',
    updatedAt: '2024-01-15',
  },
])

const activeTab = ref<'roles' | 'permissions' | 'matrix'>('roles')
const showCreateRoleDialog = ref(false)
const editingRole = ref<Role | null>(null)
const newRole = ref({
  name: '',
  description: '',
  permissions: [] as string[],
})

const selectedRole = ref<Role | null>(null)
const roleSearch = ref('')

const filteredRoles = computed(() => 
  roles.value.filter(r => 
    r.name.toLowerCase().includes(roleSearch.value.toLowerCase()) ||
    r.description.toLowerCase().includes(roleSearch.value.toLowerCase())
  )
)

const categories = computed(() => 
  [...new Set(allPermissions.map(p => p.category))].sort()
)

const openCreateRole = () => {
  editingRole.value = null
  newRole.value = { name: '', description: '', permissions: [] }
  showCreateRoleDialog.value = true
}

const openEditRole = (role: Role) => {
  editingRole.value = role
  newRole.value = { 
    name: role.name, 
    description: role.description, 
    permissions: [...role.permissions] 
  }
  showCreateRoleDialog.value = true
}

const saveRole = () => {
  if (!newRole.value.name.trim()) {
    toast.error('Role name is required')
    return
  }
  
  if (editingRole.value) {
    const index = roles.value.findIndex(r => r.id === editingRole.value!.id)
    if (index !== -1) {
      roles.value[index] = {
        ...editingRole.value,
        name: newRole.value.name,
        description: newRole.value.description,
        permissions: newRole.value.permissions,
        updatedAt: new Date().toISOString(),
      }
      toast.success('Role updated successfully')
    }
  } else {
    const newRoleObj: Role = {
      id: `role_${Date.now()}`,
      name: newRole.value.name,
      description: newRole.value.description,
      permissions: newRole.value.permissions,
      isSystem: false,
      userCount: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    }
    roles.value.push(newRoleObj)
    toast.success('Role created successfully')
  }
  
  showCreateRoleDialog.value = false
}

const deleteRole = (roleId: string) => {
  const role = roles.value.find(r => r.id === roleId)
  if (role?.isSystem) {
    toast.error('Cannot delete system role')
    return
  }
  
  if (confirm(`Delete role "${role?.name}"? This cannot be undone.`)) {
    roles.value = roles.value.filter(r => r.id !== roleId)
    toast.success('Role deleted')
  }
}

const selectRoleForDetails = (role: Role) => {
  selectedRole.value = role
}

const togglePermission = (roleId: string, permissionId: string) => {
  const role = roles.value.find(r => r.id === roleId)
  if (!role) return
  
  const index = role.permissions.indexOf(permissionId)
  if (index === -1) {
    role.permissions.push(permissionId)
  } else {
    role.permissions.splice(index, 1)
  }
  role.updatedAt = new Date().toISOString()
}

const hasPermission = (role: Role, permissionId: string) => role.permissions.includes(permissionId)

const allSelected = computed(() => 
  permissionsByCategory.value[selectedRole.value?.id || '']?.every(p => 
    selectedRole.value?.permissions.includes(p.id)
  ) || false
)

const toggleAllPermissions = (category: string) => {
  if (!selectedRole.value) return
  
  const categoryPermissions = permissionsByCategory.value[category] || []
  const allSelected = categoryPermissions.every(p => selectedRole.value!.permissions.includes(p.id))
  
  if (allSelected) {
    selectedRole.value.permissions = selectedRole.value.permissions.filter(p => !categoryPermissions.some(cp => cp.id === p))
  } else {
    categoryPermissions.forEach(p => {
      if (!selectedRole.value!.permissions.includes(p.id)) {
        selectedRole.value!.permissions.push(p.id)
      }
    })
  }
  selectedRole.value!.updatedAt = new Date().toISOString()
}

const copyRole = (role: Role) => {
  const newRoleObj: Role = {
    id: `role_${Date.now()}`,
    name: `${role.name} (Copy)`,
    description: role.description,
    permissions: [...role.permissions],
    isSystem: false,
    userCount: 0,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  }
  roles.value.push(newRoleObj)
  toast.success('Role duplicated')
}

const exportRole = (role: Role) => {
  const data = JSON.stringify(role, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `${role.name.toLowerCase().replace(/\s+/g, '_')}_role.json`
  a.click()
  URL.revokeObjectURL(url)
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
      <div class="flex items-center gap-3">
        <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
          <ShieldIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
        </div>
        <div>
          <h1 class="text-2xl font-bold">Role Manager</h1>
          <p class="text-muted-foreground">Manage roles and permissions across the platform</p>
        </div>
      </div>

      <!-- Tabs -->
      <Tabs default-value="roles" class="w-full">
        <TabsList class="grid grid-cols-3">
          <TabsTrigger value="roles">
            <UsersIcon class="h-4 w-4 mr-2" />
            Roles
          </TabsTrigger>
          <TabsTrigger value="permissions">
            <KeyIcon class="h-4 w-4 mr-2" />
            Permissions
          </TabsTrigger>
          <TabsTrigger value="matrix">
            <ShieldCheckIcon class="h-4 w-4 mr-2" />
            Matrix
          </TabsTrigger>
        </TabsList>

        <!-- Roles Tab -->
        <TabsContent value="roles" class="mt-6 flex-1 overflow-y-auto">
          <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
            <div class="relative flex-1 max-w-md">
              <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <Input v-model="roleSearch" placeholder="Search roles..." class="pl-10" />
            </div>
            <Button @click="openCreateRole" class="gap-2">
              <PlusIcon class="h-4 w-4" />
              Create Role
            </Button>
          </div>

          <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            <Card v-for="role in filteredRoles" :key="role.id" class="relative overflow-hidden hover:shadow-lg transition-shadow">
              <div class="absolute top-3 right-3 flex gap-1">
                <DropdownMenu>
                  <DropdownMenuTrigger as-child>
                    <Button variant="ghost" size="icon" class="h-8 w-8">
                      <MoreHorizontalIcon class="h-4 w-4" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end">
                    <DropdownMenuItem @click="selectRoleForDetails(role)">
                      <EyeIcon class="h-4 w-4" />
                      View Details
                    </DropdownMenuItem>
                    <DropdownMenuItem @click="openEditRole(role)">
                      <EditIcon class="h-4 w-4" />
                      Edit Role
                    </DropdownMenuItem>
                    <DropdownMenuItem @click="copyRole(role)">
                      <CopyIcon class="h-4 w-4" />
                      Duplicate
                    </DropdownMenuItem>
                    <DropdownMenuItem @click="exportRole(role)">
                      <DownloadIcon class="h-4 w-4" />
                      Export JSON
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem v-if="!role.isSystem" @click="deleteRole(role.id)" class="text-destructive">
                      <TrashIcon class="h-4 w-4" />
                      Delete Role
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>

              <div class="p-5">
                <div class="flex items-start justify-between mb-3">
                  <div class="flex items-center gap-3">
                    <div class="p-3 rounded-xl bg-primary-100 dark:bg-primary-900/30">
                      <ShieldIcon class="h-6 w-6 text-primary-600 dark:text-primary-400" />
                    </div>
                    <div>
                      <h3 class="font-semibold text-lg">{{ role.name }}</h3>
                      <p class="text-sm text-muted-foreground">{{ role.description }}</p>
                    </div>
                  </div>
                  
                  <Badge v-if="role.isSystem" variant="secondary" class="gap-1">
                    <LockIcon class="h-3 w-3" />
                    System Role
                  </Badge>
                </div>

                <div class="space-y-2 text-sm">
                  <div class="flex items-center justify-between text-muted-foreground">
                    <span>{{ role.permissions.length }} permissions</span>
                    <span>{{ role.userCount }} users</span>
                  </div>
                  <div class="flex items-center justify-between text-muted-foreground">
                    <span>Created</span>
                    <span>{{ formatRelativeTime(role.createdAt) }}</span>
                  </div>
                </div>

                <div class="mt-4 pt-4 border-t border-border flex gap-2">
                  <Button variant="outline" size="sm" class="flex-1 gap-1" @click="selectRoleForDetails(role)">
                    <EyeIcon class="h-3 w-3" />
                    Details
                  </Button>
                  <Button variant="outline" size="sm" class="flex-1 gap-1" @click="openEditRole(role)">
                    <EditIcon class="h-3 w-3" />
                    Edit
                  </Button>
                </div>
              </div>
            </Card>
          </div>

          <div v-if="filteredRoles.length === 0" class="text-center py-12">
            <ShieldIcon class="h-12 w-12 mx-auto text-muted-foreground/50 mb-4" />
            <h3 class="font-semibold mb-2">No roles found</h3>
            <p class="text-muted-foreground mb-4">Create a new role to get started</p>
            <Button @click="openCreateRole" class="gap-2">
              <PlusIcon class="h-4 w-4" />
              Create Role
            </Button>
          </div>
        </TabsContent>

        <!-- Permissions Tab -->
        <TabsContent value="permissions" class="mt-6">
          <div class="space-y-6">
            <div v-for="category in categories" :key="category">
              <Card>
                <CardHeader>
                  <div class="flex items-center justify-between">
                    <CardTitle class="capitalize">{{ category }}</CardTitle>
                    <Badge variant="secondary">{{ permissionsByCategory[category].length }} permissions</Badge>
                  </CardHeader>
                  <CardContent>
                    <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                      <div v-for="permission in permissionsByCategory[category]" :key="permission.id" class="flex items-center gap-3 p-3 rounded-lg border border-border hover:bg-accent/50 transition-colors">
                        <Checkbox 
                          :checked="editingRole && editingRole.permissions.includes(permission.id)"
                          @update:checked="checked => editingRole && togglePermission(editingRole.id, permission.id)"
                          :disabled="!editingRole"
                        />
                        <div class="flex-1 min-w-0">
                          <p class="font-mono text-sm font-medium">{{ permission.name }}</p>
                          <p class="text-xs text-muted-foreground truncate">{{ permission.description }}</p>
                          <p class="text-xs text-muted-foreground font-mono">{{ permission.resource }}.{{ permission.action }}</p>
                        </div>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </div>
            </div>
          </TabsContent>

          <!-- Matrix Tab -->
          <TabsContent value="matrix" class="mt-6">
            <div v-if="selectedRole" class="space-y-6">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-semibold text-lg">{{ selectedRole.name }} - Permission Matrix</h3>
                  <p class="text-muted-foreground">{{ selectedRole.permissions.length }} permissions assigned</p>
                </div>
                <div class="flex gap-2">
                  <Button variant="outline" @click="toggleAllPermissions('User Management')">Select All</Button>
                  <Button variant="outline" @click="selectedRole.permissions = []">Clear All</Button>
                  <Button variant="outline" @click="exportRole(selectedRole)">
                    <DownloadIcon class="h-4 w-4 mr-1" />
                    Export
                  </Button>
                </div>
              </div>

              <div class="overflow-x-auto">
                <table class="w-full text-sm">
                  <thead>
                    <tr class="border-b border-border">
                      <th class="text-left p-3 font-medium sticky left-0 bg-card z-10 w-64">Permission</th>
                      <th class="text-center p-3 font-medium w-16">Granted</th>
                      <th class="text-left p-3 font-medium text-muted-foreground">Description</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="category in categories" :key="category" class="border-b border-border">
                      <td class="p-3 font-semibold bg-muted/50 sticky left-0 bg-card z-10" colspan="3">
                        <div class="flex items-center justify-between">
                          <span class="capitalize">{{ category }}</span>
                          <div class="flex gap-2">
                            <Button 
                              size="sm" 
                              variant="ghost" 
                              class="gap-1"
                              @click="toggleAllPermissions(category)"
                              :disabled="!selectedRole"
                            >
                              <ChevronRightIcon class="h-3 w-3" />
                              Select All
                            </Button>
                            <Button 
                              size="sm" 
                              variant="ghost" 
                              class="gap-1 text-destructive"
                              @click="toggleAllPermissions(category)"
                              :disabled="!selectedRole"
                            >
                              <ChevronLeftIcon class="h-3 w-3" />
                              Clear All
                            </Button>
                          </div>
                        </span>
                      </div>
                    </tr>
                    <tr v-for="permission in permissionsByCategory[category]" :key="permission.id">
                      <td class="p-3">
                        <code class="text-xs font-mono">{{ permission.resource }}.{{ permission.action }}</code>
                        <p class="text-xs text-muted-foreground mt-0.5">{{ permission.name }}</p>
                      </td>
                      <td class="p-3 text-center">
                        <Checkbox 
                          :checked="hasPermission(selectedRole!, permission.id)"
                          @update:checked="checked => togglePermission(selectedRole!.id, permission.id)"
                        />
                      </td>
                      <td class="p-3 text-muted-foreground text-xs">{{ permission.description }}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
            <div v-else class="text-center py-12">
              <ShieldIcon class="h-12 w-12 mx-auto text-muted-foreground/50 mb-4" />
              <h3 class="font-semibold mb-2">Select a role</h3>
              <p class="text-muted-foreground mb-4">Click on a role in the Roles tab to view its permission matrix</p>
            </div>
          </TabsContent>
        </Tabs>
      </div>

      <!-- Create/Edit Role Dialog -->
      <Dialog v-model:open="showCreateRoleDialog">
        <DialogTrigger as-child>
          <Button variant="outline">Open Dialog</Button>
        </DialogTrigger>
        <DialogContent class="max-w-2xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{{ editingRole ? 'Edit Role' : 'Create New Role' }}</DialogTitle>
            <DialogDescription>
              {{ editingRole ? 'Modify role details and permissions' : 'Define a new role with specific permissions' }}
            </DialogDescription>
          </DialogHeader>
          
          <div class="space-y-6 py-4">
            <div class="grid gap-4 sm:grid-cols-2">
              <div class="sm:col-span-2">
                <Label for="name">Role Name</Label>
                <Input 
                  id="name" 
                  v-model="newRole.name" 
                  placeholder="e.g., Content Manager"
                  class="mt-1"
                />
              </div>
              <div class="sm:col-span-2">
                <Label for="description">Description</Label>
                <textarea
                  id="description"
                  v-model="newRole.description"
                  placeholder="Describe what this role can do..."
                  class="mt-1 w-full rounded-lg border border-input bg-background p-3 text-sm resize-y min-h-[80px]"
                />
              </div>
            </div>

            <div>
              <Label class="block mb-3">Permissions</Label>
              <div class="space-y-4 max-h-96 overflow-y-auto">
                <div v-for="category in categories" :key="category">
                  <h4 class="font-medium text-sm text-muted-foreground mb-2 capitalize">{{ category }}</h4>
                  <div class="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
                    <label v-for="permission in permissionsByCategory[category]" :key="permission.id" class="flex items-center gap-2 cursor-pointer">
                      <Checkbox 
                        :checked="newRole.permissions.includes(permission.id)"
                        @update:checked="checked => {
                          if (checked) {
                            newRole.permissions.push(permission.id)
                          } else {
                            newRole.permissions = newRole.permissions.filter(p => p !== permission.id)
                          }
                        }"
                      />
                      <div class="flex-1 min-w-0">
                        <p class="font-mono text-sm text-foreground">{{ permission.name }}</p>
                        <p class="text-xs text-muted-foreground truncate">{{ permission.description }}</p>
                      </div>
                    </label>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <DialogFooter class="flex justify-end gap-2 border-t border-border pt-4">
            <Button variant="outline" @click="showCreateRoleDialog = false">Cancel</Button>
            <Button @click="saveRole" :disabled="!newRole.name.trim()">
              {{ editingRole ? 'Update Role' : 'Create Role' }}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  </div>
</template>

<script>
const UsersIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })]) }})
const KeyIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z' })]) }})
const ShieldCheckIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' })]) }})
const ChevronRightIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 5l7 7-7 7' })]) }})
const ChevronLeftIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 19l-7-7 7-7' })]) }})
const LockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z' })]) }})
const UnlockIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '3', y: '11', width: '18', height: '11', rx: '2', ry: '2', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M7 11V7a5 5 0 0110 0v4' })]) }})
</script>