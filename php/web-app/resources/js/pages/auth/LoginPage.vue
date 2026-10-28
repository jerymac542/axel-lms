<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Link } from '@inertiajs/vue3'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui/card'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Separator } from '@/components/ui/separator'
import { useAuth } from '@/composables/useAuth'
import { useToast } from '@/composables/useToast'
import { MailIcon, LockIcon, EyeIcon, EyeOffIcon, UserIcon, AlertCircleIcon, CheckCircleIcon } from '@/components/ui/icons'

const router = useRouter()
const route = useRoute()
const auth = useAuth()
const toast = useToast()

const showPassword = ref(false)
const isLoading = ref(false)
const errors = ref<Record<string, string>>({})
const redirect = route.query.redirect as string || '/dashboard'

const form = reactive({
  email: '',
  password: '',
  remember: false,
  mfa_code: '',
  backup_code: '',
})

const validationRules = {
  email: (value: string) => {
    if (!value) return 'Email is required'
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) return 'Please enter a valid email address'
    return true
  },
  password: (value: string) => {
    if (!value) return 'Password is required'
    if (value.length < 8) return 'Password must be at least 8 characters'
    return true
  },
}

const validateField = (name: string) => {
  const rule = validationRules[name as keyof typeof validationRules]
  if (rule) {
    const result = rule(form[name as keyof typeof form])
    if (result !== true) {
      errors.value[name] = result
      return false
    }
  }
  delete errors.value[name]
  return true
}

const validateForm = () => {
  let valid = true
  Object.keys(validationRules).forEach(key => {
    if (!validateField(key)) valid = false
  })
  return valid
}

const handleSubmit = async () => {
  if (!validateForm()) return
  
  isLoading.value = true
  errors.value = {}
  
  try {
    await auth.login({
      email: form.email,
      password: form.password,
      remember: form.remember,
    })
    
    toast.success('Welcome back!')
    router.push(redirect)
  } catch (error: any) {
    isLoading.value = false
    
    if (error.response?.status === 401) {
      if (error.response.data?.mfa_required) {
        // Show MFA form
        return
      }
      errors.value.form = error.response.data?.message || 'Invalid email or password'
    } else if (error.response?.status === 422) {
      Object.assign(errors.value, error.response.data?.errors || {})
    } else {
      toast.error('An error occurred. Please try again.')
    }
  }
}

const handleForgotPassword = () => {
  router.push({ name: 'forgot-password', query: { email: form.email } })
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-muted/30 px-4 py-12">
    <div class="w-full max-w-md">
      <!-- Logo -->
      <div class="text-center mb-8">
        <Link href="/" class="inline-flex items-center gap-2 font-display font-bold text-2xl text-foreground">
          <svg class="h-10 w-10 text-primary-600 dark:text-primary-400" viewBox="0 0 32 32" fill="none">
            <rect width="32" height="32" rx="8" fill="currentColor"/>
            <path d="M8 12L16 8L24 12V20L16 24L8 20V12Z" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M16 8V24" stroke="white" stroke-width="2" stroke-linecap="round"/>
            <path d="M8 12L24 12" stroke="white" stroke-width="2" stroke-linecap="round"/>
            <path d="M8 20L24 20" stroke="white" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <span>Axel LMS</span>
        </Link>
      </div>

      <!-- Card -->
      <Card class="shadow-xl">
        <CardHeader class="text-center">
          <CardTitle class="text-2xl">Welcome back</CardTitle>
          <CardDescription>Sign in to your account to continue learning</CardDescription>
        </CardHeader>

        <CardContent class="space-y-4">
          <!-- Error Alert -->
          <Alert v-if="errors.form" variant="destructive" class="mb-4">
            <AlertCircleIcon class="h-4 w-4" />
            <AlertDescription>{{ errors.form }}</AlertDescription>
          </Alert>

          <!-- Form -->
          <form @submit.prevent="handleSubmit" class="space-y-4">
            <!-- Email -->
            <div class="space-y-2">
              <Label for="email">Email</Label>
              <div class="relative">
                <MailIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" />
                <Input
                  id="email"
                  type="email"
                  v-model="form.email"
                  :class="{ 'input-error': errors.email }"
                  :aria-invalid="!!errors.email"
                  :aria-describedby="errors.email ? 'email-error' : undefined"
                  placeholder="you@example.com"
                  autocomplete="email"
                  @blur="validateField('email')"
                  @input="errors.email && validateField('email')"
                  :disabled="isLoading"
                />
              </div>
              <p v-if="errors.email" id="email-error" class="text-sm text-destructive-600 dark:text-destructive-400" role="alert">
                {{ errors.email }}
              </p>
            </div>

            <!-- Password -->
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <Label for="password">Password</Label>
                <Link :href="{ name: 'forgot-password', query: { email: form.email } }" class="text-sm text-primary-600 hover:underline">
                  Forgot password?
                </Link>
              </div>
              <div class="relative">
                <LockIcon class="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" />
                <Input
                  id="password"
                  :type="showPassword ? 'text' : 'password'"
                  v-model="form.password"
                  :class="{ 'input-error': errors.password }"
                  :aria-invalid="!!errors.password"
                  :aria-describedby="errors.password ? 'password-error' : undefined"
                  placeholder="••••••••"
                  autocomplete="current-password"
                  @blur="validateField('password')"
                  @input="errors.password && validateField('password')"
                  :disabled="isLoading"
                />
                <button
                  type="button"
                  @click="showPassword = !showPassword"
                  class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                  :aria-label="showPassword ? 'Hide password' : 'Show password'"
                >
                  <EyeIcon v-if="!showPassword" class="h-5 w-5" />
                  <EyeOffIcon v-else class="h-5 w-5" />
                </button>
              </div>
              <p v-if="errors.password" id="password-error" class="text-sm text-destructive-600 dark:text-destructive-400" role="alert">
                {{ errors.password }}
              </p>
            </div>

            <!-- Remember Me -->
            <div class="flex items-center justify-between">
              <label class="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  v-model="form.remember"
                  class="h-4 w-4 rounded border-border text-primary-600 focus:ring-primary-500"
                />
                <span class="text-sm text-muted-foreground">Remember me</span>
              </label>
            </div>

            <!-- Submit -->
            <Button
              type="submit"
              class="w-full"
              size="lg"
              :loading="isLoading"
              :disabled="isLoading"
            >
              <CheckCircleIcon class="h-4 w-4" />
              <span>Sign In</span>
            </Button>
          </form>
        </CardContent>

        <CardFooter class="flex flex-col gap-4">
          <Separator>Or continue with</Separator>
          
          <div class="grid grid-cols-2 gap-3">
            <Button
              variant="outline"
              @click="$refs.googleLogin?.click()"
              :disabled="isLoading"
              class="gap-2"
            >
              <svg class="h-5 w-5" viewBox="0 0 24 24">
                <path fill="currentColor" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                <path fill="currentColor" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23c3.13 0 5.85-1.25 7.98-3.34z"/>
              </svg>
              <span>Google</span>
            </Button>
            <Button
              variant="outline"
              @click="$refs.microsoftLogin?.click()"
              :disabled="isLoading"
              class="gap-2"
            >
              <svg class="h-5 w-5" viewBox="0 0 24 24">
                <path fill="currentColor" d="M19.3 19.1c-.2 1.2-.5 2.3-.8 3.4H24c1.4-3.1 2.2-6.7 2.2-10.5 0-.7 0-1.3-.1-2H12V6.4h2.8l.3-2.8H12V3.2c1.3 0 2.6.4 2.6 2.6v2.3H8.7v3.9H11v6.7h-4V24h6.4z"/>
              </svg>
              <span>Microsoft</span>
            </Button>
          </div>
        </CardFooter>
      </Card>

      <!-- Sign Up Link -->
      <p class="mt-6 text-center text-sm text-muted-foreground">
        Don't have an account?{' '}
        <Link :href="{ name: 'register', query: { redirect } }" class="font-medium text-primary-600 hover:underline">
          Create one
        </Link>
      </p>

      <!-- Demo Notice -->
      <div class="mt-8 p-4 rounded-lg bg-muted/50 border border-border">
        <p class="text-sm text-muted-foreground text-center">
          <strong>Demo Mode:</strong> Use any email/password to sign in. MFA is optional in demo.
        </p>
      </div>
    </div>

    <!-- Hidden OAuth buttons -->
    <button ref="googleLogin" style="display: none" @click="handleOAuth('google')" />
    <button ref="microsoftLogin" style="display: none" @click="handleOAuth('microsoft')" />
  </div>
</template>

<script>
const MailIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z' })]) }})
const LockIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z' })]) }})
const EyeIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z' })]) }})
const EyeOffIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L3 3m0 0l3.59 3.59m0 0a9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411' })]) }})
const AlertCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z' })]) }})
const CheckCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
</script>