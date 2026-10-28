import { ref, computed } from 'vue'
import { useAuth } from './useAuth'

interface FeatureFlags {
  flags: Record<string, boolean>
  rollout: Record<string, { percentage: number; groups: string[]; tenants: string[]; users: string[] }>
}

const featureFlags = ref<FeatureFlags>({
  flags: {},
  rollout: {},
})

const loaded = ref(false)

export function useFeatureFlags() {
  const auth = useAuth()

  const loadFlags = async () => {
    if (loaded.value) return
    
    try {
      // Fetch from API - would be something like:
      // const response = await axios.get('/api/feature-flags')
      // featureFlags.value = response.data
      
      // Mock data for now
      featureFlags.value = {
        flags: {
          new_dashboard: true,
          ai_recommendations: true,
          dark_mode: true,
          beta_features: false,
          advanced_analytics: true,
          mobile_app_sync: true,
          social_learning: true,
          gamification: true,
          certificates_v2: false,
          lti_13_advantage: true,
          scorm_2004_4th: true,
        },
        rollout: {
          new_course_player: {
            percentage: 50,
            groups: ['beta_testers'],
            tenants: [],
            users: [],
          },
          ai_tutor: {
            percentage: 10,
            groups: ['premium_users'],
            tenants: [],
            users: [],
          },
        },
      }
      loaded.value = true
    } catch (error) {
      console.error('Failed to load feature flags:', error)
      loaded.value = true
    }
  }

  const isEnabled = (flag: string, userId?: string): boolean => {
    // Check explicit flag
    if (flag in featureFlags.value.flags) {
      return featureFlags.value.flags[flag]
    }

    // Check rollout
    const rollout = featureFlags.value.rollout[flag]
    if (rollout) {
      // Check user inclusion
      if (userId && rollout.users.includes(userId)) return true
      
      // Check group inclusion (would need user's groups)
      // if (userGroups && rollout.groups.some(g => userGroups.includes(g))) return true
      
      // Check tenant inclusion
      // if (currentTenant && rollout.tenants.includes(currentTenant)) return true
      
      // Percentage rollout (deterministic based on userId)
      if (userId && rollout.percentage > 0) {
        const hash = simpleHash(userId)
        return (hash % 100) < rollout.percentage
      }
    }

    return false
  }

  const simpleHash = (str: string): number => {
    let hash = 0
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i)
      hash = ((hash << 5) - hash) + char
      hash = hash & hash
    }
    return Math.abs(hash)
  }

  const getFlag = (flag: string) => {
    return featureFlags.value.flags[flag] ?? false
  }

  const getRollout = (flag: string) => {
    return featureFlags.value.rollout[flag] ?? null
  }

  const setFlag = (flag: string, enabled: boolean) => {
    featureFlags.value.flags[flag] = enabled
  }

  const setRollout = (flag: string, config: FeatureFlags['rollout'][string]) => {
    featureFlags.value.rollout[flag] = config
  }

  // Reactivity for template
  const allFlags = computed(() => featureFlags.value.flags)
  const allRollouts = computed(() => featureFlags.value.rollout)

  return {
    // State
    flags: allFlags,
    rollouts: allRollouts,
    loaded: computed(() => loaded.value),

    // Methods
    isEnabled,
    getFlag,
    getRollout,
    setFlag,
    setRollout,
    loadFlags,
  }
}

// Auto-load on first use
let hasLoaded = false
export function useFeatureFlagsAuto() {
  const flags = useFeatureFlags()
  
  if (!hasLoaded) {
    hasLoaded = true
    flags.loadFlags()
  }
  
  return flags
}