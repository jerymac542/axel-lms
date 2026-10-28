<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { cn } from '@/utils/cn'
import { 
  PlayIcon, PauseIcon, Volume2Icon, VolumeXIcon, 
  FullscreenIcon, MinimizeIcon, SettingsIcon, 
  SkipBackIcon, SkipForwardIcon, SpeedIcon,
  PictureInPictureIcon, CaptionsIcon, AirplayIcon,
  DownloadIcon, BookmarkIcon, FlagIcon, Share2Icon,
  ChevronLeftIcon, ChevronRightIcon, ExpandIcon,
  MessageSquareIcon, FileTextIcon, BookOpenIcon,
  CheckCircleIcon, ClockIcon, HelpCircleIcon
} from '@/components/ui/icons'
import { Button } from '@/components/ui/button'
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import { Slider } from '@/components/ui/slider'
import { Badge } from '@/components/ui/badge'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuLabel } from '@/components/ui/dropdown-menu'
import { Tooltip, TooltipTrigger, TooltipContent } from '@/components/ui/tooltip'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { Separator } from '@/components/ui/separator'
import { formatDuration, formatTime } from '@/utils/cn'
import { useAuth } from '@/composables/useAuth'
import { useToast } from '@/composables/useToast'

interface Props {
  lesson: any
  course: any
  enrollment: any
  autoplay?: boolean
  onTimeUpdate?: (current: number, duration: number) => void
  onEnded?: () => void
  onComplete?: () => void
}

const props = withDefaults(defineProps<Props>(), {
  autoplay: true,
})

const auth = useAuth()
const toast = useToast()

const videoRef = ref<HTMLVideoElement>(null)
const isPlaying = ref(false)
const isMuted = ref(false)
const volume = ref(1)
const currentTime = ref(0)
const duration = ref(0)
const buffered = ref(0)
const playbackRate = ref(1)
const isFullscreen = ref(false)
const showControls = ref(true)
const controlsTimeout = ref<ReturnType<typeof setTimeout>>()
const quality = ref('auto')
const qualities = ref(['auto', '1080p', '720p', '480p', '360p'])
const captions = ref<'off' | 'on'>('off')
const availableCaptions = ref<{ label: string, src: string, srclang: string }[]>([])
const pictureInPicture = ref(false)
const notes = ref('')
const bookmarks = ref<{ time: number, note: string }[]>([])

const isVideo = computed(() => props.lesson.type === 'video' && props.lesson.video_url)
const isAudio = computed(() => props.lesson.type === 'audio' && props.lesson.audio_url)
const isInteractive = computed(() => ['quiz', 'assignment', 'discussion', 'reading'].includes(props.lesson.type))

const progress = computed(() => duration.value > 0 ? (currentTime.value / duration.value) * 100 : 0)
const formattedCurrentTime = computed(() => formatTime(currentTime.value))
const formattedDuration = computed(() => formatTime(duration.value))

const handleVideoLoadedMetadata = () => {
  if (videoRef.value) {
    duration.value = videoRef.value.duration
  }
}

const handleTimeUpdate = () => {
  if (videoRef.value) {
    currentTime.value = videoRef.value.currentTime
    buffered.value = videoRef.value.buffered.length > 0 ? videoRef.value.buffered.end(0) : 0
    props.onTimeUpdate?.(currentTime.value, duration.value)
  }
}

const handleProgress = () => {
  if (videoRef.value && videoRef.value.buffered.length > 0) {
    buffered.value = videoRef.value.buffered.end(0)
  }
}

const handleEnded = () => {
  isPlaying.value = false
  props.onEnded?.()
  
  // Auto-complete video lessons
  if (props.lesson.type === 'video' && !props.lesson.completed) {
    props.onComplete?.()
  }
}

const handleWaiting = () => {
  // Show loading state
}

const handleCanPlay = () => {
  // Hide loading state
}

const handleError = () => {
  toast.error('Failed to load video. Please try again.')
}

const togglePlay = () => {
  if (videoRef.value) {
    if (isPlaying.value) {
      videoRef.value.pause()
    } else {
      videoRef.value.play().catch(() => {
        toast.error('Unable to play video')
      })
    }
    isPlaying.value = !isPlaying.value
  }
}

const handleVideoClick = () => {
  togglePlay()
}

const skip = (seconds: number) => {
  if (videoRef.value) {
    videoRef.value.currentTime = Math.max(0, Math.min(duration.value, currentTime.value + seconds))
  }
}

const setPlaybackRate = (rate: number) => {
  if (videoRef.value) {
    videoRef.value.playbackRate = rate
    playbackRate.value = rate
  }
}

const setVolume = (vol: number) => {
  if (videoRef.value) {
    videoRef.value.volume = vol
    volume.value = vol
    isMuted.value = vol === 0
  }
}

const toggleMute = () => {
  if (videoRef.value) {
    isMuted.value = !isMuted.value
    videoRef.value.muted = isMuted.value
  }
}

const toggleFullscreen = async () => {
  if (videoRef.value) {
    try {
      if (!isFullscreen.value) {
        await videoRef.value.requestFullscreen()
      } else {
        await document.exitFullscreen()
      }
    } catch (err) {
      console.error('Fullscreen error:', err)
    }
  }
}

const togglePictureInPicture = async () => {
  if (videoRef.value && document.pictureInPictureEnabled) {
    try {
      if (!pictureInPicture.value) {
        await videoRef.value.requestPictureInPicture()
      } else {
        await document.exitPictureInPicture()
      }
    } catch (err) {
      console.error('PiP error:', err)
    }
  }
}

const handleSeek = (value: number) => {
  if (videoRef.value) {
    videoRef.value.currentTime = (value / 100) * duration.value
  }
}

const handleVolumeChange = (value: number) => {
  setVolume(value / 100)
}

const addBookmark = () => {
  bookmarks.value.push({
    time: currentTime.value,
    note: '',
    timestamp: Date.now(),
  })
  toast.success('Bookmark added')
}

const saveNotes = () => {
  // Save to backend
  toast.success('Notes saved')
}

const handleKeyDown = (event: KeyboardEvent) => {
  if (!videoRef.value) return
  
  // Don't handle shortcuts if user is typing in input
  if (['INPUT', 'TEXTAREA'].includes((event.target as HTMLElement).tagName)) return
  
  switch (event.key) {
    case ' ':
    case 'k':
      event.preventDefault()
      togglePlay()
      break
    case 'ArrowLeft':
    case 'j':
      event.preventDefault()
      skip(-10)
      break
    case 'ArrowRight':
    case 'l':
      event.preventDefault()
      skip(10)
      break
    case 'ArrowUp':
      event.preventDefault()
      setVolume(Math.min(1, volume.value + 0.1))
      break
    case 'ArrowDown':
      event.preventDefault()
      setVolume(Math.max(0, volume.value - 0.1))
      break
    case 'm':
      event.preventDefault()
      toggleMute()
      break
    case 'f':
      event.preventDefault()
      toggleFullscreen()
      break
    case '>':
    case '.':
      event.preventDefault()
      setPlaybackRate(Math.min(2, playbackRate.value + 0.25))
      break
    case '<':
    case ',':
      event.preventDefault()
      setPlaybackRate(Math.max(0.25, playbackRate.value - 0.25))
      break
    case 'c':
      event.preventDefault()
      captions.value = captions.value === 'on' ? 'off' : 'on'
      break
  }
}

const showControlsTimer = () => {
  showControls.value = true
  if (controlsTimeout.value) clearTimeout(controlsTimeout.value)
  controlsTimeout.value = setTimeout(() => {
    showControls.value = false
  }, 3000)
}

onMounted(() => {
  document.addEventListener('keydown', handleKeyDown)
  document.addEventListener('fullscreenchange', () => {
    isFullscreen.value = !!document.fullscreenElement
  })
  
  // Auto-play
  if (props.autoplay && videoRef.value) {
    videoRef.value.play().catch(() => {
      isPlaying.value = false
    })
  }
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeyDown)
  if (controlsTimeout.value) clearTimeout(controlsTimeout.value)
})

watch(() => props.lesson.id, () => {
  // Reset state when lesson changes
  isPlaying.value = false
  currentTime.value = 0
  duration.value = 0
  buffered.value = 0
  if (props.autoplay) {
    nextTick(() => {
      videoRef.value?.play().catch(() => {})
    })
  }
})
</script>

<template>
  <div class="relative w-full">
    <!-- Video Player -->
    <div class="aspect-video bg-black rounded-xl overflow-hidden relative" @mousemove="showControlsTimer" @mouseleave="showControls.value = false">
      <!-- Video Element -->
      <video
        ref="videoRef"
        :src="lesson.video_url"
        :poster="lesson.thumbnail_url"
        :muted="isMuted"
        :volume="volume"
        :playbackRate="playbackRate"
        class="w-full h-full object-contain"
        @loadedmetadata="handleVideoLoadedMetadata"
        @timeupdate="handleTimeUpdate"
        @progress="handleProgress"
        @ended="handleEnded"
        @waiting="handleWaiting"
        @canplay="handleCanPlay"
        @error="handleError"
        @click="handleVideoClick"
        playsinline
        webkit-playsinline
        crossOrigin="anonymous"
      >
        <track v-for="caption in availableCaptions" :key="caption.srclang" kind="captions" :src="caption.src" :srclang="caption.srclang" :label="caption.label" />
        Your browser does not support the video tag.
      </video>

      <!-- Loading Overlay -->
      <div v-if="!duration" class="absolute inset-0 flex items-center justify-center bg-black/50">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-primary-500 border-t-transparent" />
      </div>

      <!-- Error Overlay -->
      <div v-if="videoRef?.error" class="absolute inset-0 flex items-center justify-center bg-black/50 p-4 text-center">
        <div class="bg-white/10 rounded-lg p-6 max-w-md">
          <AlertCircleIcon class="h-12 w-12 text-destructive-500 mx-auto mb-4" />
          <h3 class="text-lg font-semibold mb-2">Unable to play video</h3>
          <p class="text-muted-foreground mb-4">There was an error loading this video. Please try again later.</p>
          <Button @click="videoRef.value?.load()" variant="default">Retry</Button>
        </div>
      </div>

      <!-- Top Bar -->
      <div v-show="showControls" class="absolute top-0 left-0 right-0 p-4 bg-gradient-to-b from-black/60 to-transparent transition-opacity duration-300 pointer-events-none">
        <div class="flex items-center justify-between pointer-events-auto">
          <div class="flex items-center gap-2">
            <Button variant="ghost" size="icon" @click="$emit('back')" class="text-white hover:text-primary-300">
              <ChevronLeftIcon class="h-5 w-5" />
            </Button>
            <div>
              <h3 class="text-white font-medium truncate max-w-xs">{{ lesson.title }}</h3>
              <p class="text-white/70 text-xs">{{ course.title }}</p>
            </div>
          </div>
          <div class="flex items-center gap-1 ml-auto">
            <Tooltip>
              <TooltipTrigger as-child>
                <Button variant="ghost" size="icon" class="text-white/80 hover:text-white" @click="togglePictureInPicture">
                  <PictureInPictureIcon class="h-5 w-5" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Picture in Picture</TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger as-child>
                <Button variant="ghost" size="icon" class="text-white/80 hover:text-white" @click="toggleFullscreen">
                  <FullscreenIcon v-if="!isFullscreen" class="h-5 w-5" />
                  <MinimizeIcon v-else class="h-5 w-5" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Fullscreen</TooltipContent>
            </Tooltip>
          </div>
        </div>
      </div>

      <!-- Bottom Controls -->
      <div v-show="showControls" class="absolute bottom-0 left-0 right-0 p-4 bg-gradient-to-t from-black/80 to-transparent transition-opacity duration-300 pointer-events-none">
        <div class="pointer-events-auto space-y-3">
          <!-- Progress Bar -->
          <div class="relative h-2 cursor-pointer" @click="seekFromClick">
            <!-- Buffered -->
            <div class="absolute inset-0 h-full rounded-full bg-white/30" :style="{ width: duration > 0 ? (buffered / duration) * 100 + '%' : '0%' }" />
            <!-- Played -->
            <div class="absolute inset-0 h-full rounded-full bg-primary-500" :style="{ width: progress + '%' }" />
            <!-- Thumb -->
            <div 
              class="absolute top-1/2 -translate-y-1/2 w-4 h-4 rounded-full bg-white shadow-lg border-2 border-primary-500 transition-transform duration-100"
              :style="{ left: `calc(${progress}% - 10px)` }"
            />
          </div>
          
          <div class="flex items-center justify-between text-white/80 text-xs mt-1">
            <span>{{ formattedCurrentTime }}</span>
            <span>{{ formattedDuration }}</span>
          </div>

          <!-- Main Controls -->
          <div class="flex items-center justify-between">
            <!-- Left Controls -->
            <div class="flex items-center gap-1">
              <Tooltip>
                <TooltipTrigger as-child>
                  <Button variant="ghost" size="icon" class="text-white/80 hover:text-white h-10 w-10" @click="skip(-10)">
                    <SkipBackIcon class="h-5 w-5" />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>Rewind 10s (J)</TooltipContent>
              </Tooltip>
              
              <Tooltip>
                <TooltipTrigger as-child>
                  <Button 
                    variant="ghost" 
                    size="icon" 
                    class="text-white/80 hover:text-white h-12 w-12 rounded-full bg-white/10"
                    @click="togglePlay"
                    :aria-label="isPlaying ? 'Pause' : 'Play'"
                  >
                    <PauseIcon v-if="isPlaying" class="h-6 w-6" />
                    <PlayIcon v-else class="h-6 w-6" />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>Play/Pause (Space/K)</TooltipContent>
              </Tooltip>
              
              <Tooltip>
                <TooltipTrigger as-child>
                  <Button variant="ghost" size="icon" class="text-white/80 hover:text-white h-10 w-10" @click="skip(10)">
                    <SkipForwardIcon class="h-5 w-5" />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>Forward 10s (L)</TooltipContent>
              </Tooltip>
            </div>

            <!-- Center - Time -->
            <div class="flex items-center gap-2 text-white/90 font-mono text-sm">
              <span>{{ formattedCurrentTime }}</span>
              <span class="text-white/50">/</span>
              <span>{{ formattedDuration }}</span>
            </div>

            <!-- Right Controls -->
            <div class="flex items-center gap-1">
              <!-- Captions -->
              <Tooltip v-if="availableCaptions.length > 0">
                <TooltipTrigger as-child>
                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Button 
                        variant="ghost" 
                        size="icon" 
                        class="text-white/80 hover:text-white h-10 w-10"
                        :class="{ 'text-primary-400': captions === 'on' }"
                      >
                        <CaptionsIcon class="h-5 w-5" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem @click="captions = 'off'">
                        <CheckCircleIcon v-if="captions === 'off'" class="h-4 w-4" />
                        Off
                      </DropdownMenuItem>
                      <DropdownMenuItem v-for="caption in availableCaptions" :key="caption.srclang" @click="captions = caption.srclang">
                        <CheckCircleIcon v-if="captions === caption.srclang" class="h-4 w-4" />
                        {{ caption.label }}
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TooltipTrigger>
                <TooltipContent>Captions (C)</TooltipContent>
              </Tooltip>

              <!-- Playback Speed -->
              <Tooltip>
                <TooltipTrigger as-child>
                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Button variant="ghost" size="icon" class="text-white/80 hover:text-white h-10 w-10">
                        <SpeedIcon class="h-5 w-5" />
                        <span class="ml-1 text-xs">{{ playbackRate }}x</span>
                        <ChevronDownIcon class="h-3 w-3 ml-1" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuLabel>Playback Speed</DropdownMenuLabel>
                      <DropdownMenuItem v-for="rate in [0.25, 0.5, 0.75, 1, 1.25, 1.5, 1.75, 2]" :key="rate" @click="setPlaybackRate(rate)">
                        <CheckCircleIcon v-if="playbackRate === rate" class="h-4 w-4" />
                        {{ rate }}x
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TooltipTrigger>
                <TooltipContent>Playback Speed (Shift+., Shift+,)</TooltipContent>
              </Tooltip>

              <!-- Quality -->
              <Tooltip v-if="qualities.length > 1">
                <TooltipTrigger as-child>
                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Button variant="ghost" size="icon" class="text-white/80 hover:text-white h-10 w-10">
                        <SettingsIcon class="h-5 w-5" />
                        <span class="ml-1 text-xs">{{ quality }}</span>
                        <ChevronDownIcon class="h-3 w-3 ml-1" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuLabel>Quality</DropdownMenuLabel>
                      <DropdownMenuItem v-for="q in qualities" :key="q" @click="quality = q">
                        <CheckCircleIcon v-if="quality === q" class="h-4 w-4" />
                        {{ q }}
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TooltipTrigger>
                <TooltipContent>Quality</TooltipContent>
              </Tooltip>

              <!-- Volume -->
              <Tooltip>
                <TooltipTrigger as-child>
                  <div class="relative">
                    <Button 
                      variant="ghost" 
                      size="icon" 
                      class="text-white/80 hover:text-white h-10 w-10"
                      @click="toggleMute"
                    >
                      <Volume2Icon v-if="!isMuted && volume > 0" class="h-5 w-5" />
                      <VolumeXIcon v-else class="h-5 w-5" />
                    </Button>
                    <div class="absolute bottom-full right-0 mb-2 hidden group-hover:block" style="width: 120px;">
                      <Slider 
                        :model-value="volume * 100" 
                        @update:modelValue="handleVolumeChange" 
                        min="0" 
                        max="100" 
                        step="1"
                        class="h-2 w-32"
                        orientation="horizontal"
                      />
                    </div>
                  </div>
                </TooltipTrigger>
                <TooltipContent>Volume (M to mute)</TooltipContent>
              </Tooltip>

              <!-- Settings -->
              <Tooltip>
                <TooltipTrigger as-child>
                  <Button variant="ghost" size="icon" class="text-white/80 hover:text-white h-10 w-10">
                    <SettingsIcon class="h-5 w-5" />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>Settings</TooltipContent>
              </Tooltip>
            </div>
          </div>
        </div>
      </div>

      <!-- Picture in Picture indicator -->
      <div v-if="pictureInPicture" class="absolute top-4 right-4 z-10">
        <Badge variant="secondary" class="gap-1">
          <PictureInPictureIcon class="h-3 w-3" />
          PiP Active
        </Badge>
      </div>
    </div>

    <!-- Non-Video Content -->
    <div v-if="!isVideo && !isAudio" class="aspect-video bg-muted rounded-xl overflow-hidden flex items-center justify-center">
      <div class="text-center p-8 max-w-2xl">
        <component 
          :is="lesson.type === 'reading' ? FileTextIcon : lesson.type === 'quiz' ? HelpCircleIcon : lesson.type === 'assignment' ? FlagIcon : BookOpenIcon"
          class="h-16 w-16 mx-auto text-muted-foreground/50 mb-4"
        />
        <h3 class="text-xl font-semibold mb-2">{{ lesson.title }}</h3>
        <p class="text-muted-foreground mb-6">{{ lesson.description }}</p>
        <div class="flex gap-4 justify-center">
          <Button v-if="lesson.content_url" @click="window.open(lesson.content_url, '_blank')" class="gap-2">
            <ExternalLinkIcon class="h-4 w-4" />
            Open Content
          </Button>
          <Button variant="outline" @click="markComplete" :disabled="lesson.completed">
            <CheckCircleIcon class="h-4 w-4" />
            {{ lesson.completed ? 'Completed' : 'Mark Complete' }}
          </Button>
        </div>
      </div>
    </div>

    <!-- Side Panel: Notes, Discussion, Transcript -->
    <div class="lg:absolute lg:inset-y-0 lg:right-0 lg:w-80 lg:border-l lg:border-border lg:bg-card hidden lg:block">
      <div class="flex flex-col h-full">
        <Tabs default-value="notes" class="flex-1 flex flex-col">
          <TabsList class="grid grid-cols-3 border-b border-border">
            <TabsTrigger value="notes">Notes</TabsTrigger>
            <TabsTrigger value="discussion">Discuss</TabsTrigger>
            <TabsTrigger value="transcript">Transcript</TabsTrigger>
          </TabsList>

          <TabsContent value="notes" class="flex-1 overflow-y-auto p-4 space-y-4">
            <div class="space-y-3">
              <textarea
                v-model="notes"
                placeholder="Take notes... (auto-saved)"
                class="w-full min-h-[200px] p-3 rounded-lg border border-input bg-background resize-none text-sm"
                @blur="saveNotes"
              />
              <div class="flex gap-2">
                <Button size="sm" @click="saveNotes" class="gap-1">
                  <SaveIcon class="h-3 w-3" />
                  Save Notes
                </Button>
                <Button variant="outline" size="sm" @click="addBookmark" class="gap-1">
                  <BookmarkIcon class="h-3 w-3" />
                  Bookmark
                </Button>
              </div>
              
              <div v-if="bookmarks.length > 0" class="space-y-2 border-t border-border pt-4">
                <h4 class="font-medium text-sm">Bookmarks</h4>
                <div v-for="(bookmark, index) in bookmarks" :key="index" class="flex items-start gap-2 p-2 rounded-lg bg-muted/50">
                  <Button variant="ghost" size="icon" class="text-muted-foreground hover:text-primary-600" @click="seekToBookmark(bookmark.time)">
                    <PlayIcon class="h-4 w-4" />
                  </Button>
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-mono text-muted-foreground">{{ formatTime(bookmark.time) }}</p>
                    <p class="text-sm truncate">{{ bookmark.note || 'No note' }}</p>
                  </div>
                </div>
              </div>
            </div>

          <TabsContent value="discussion" class="flex-1 overflow-y-auto p-4">
            <div class="space-y-4">
              <div class="flex gap-2">
                <Avatar :size="sm" :src="auth.user?.avatar_url" :fallback="auth.user?.name" />
                <textarea placeholder="Share your thoughts..." class="flex-1 min-h-[80px] p-3 rounded-lg border border-input bg-background resize-none text-sm" />
              </div>
              <Button class="w-full gap-2" @click="postDiscussion">
                <MessageSquareIcon class="h-4 w-4" />
                Post
              </Button>
              
              <div class="border-t border-border pt-4 space-y-4">
                <h4 class="font-medium">Discussion</h4>
                <div v-if="lesson.discussion?.length" class="space-y-4">
                  <div v-for="reply in lesson.discussion" :key="reply.id" class="flex gap-3">
                    <Avatar :size="sm" :src="reply.author.avatar_url" :fallback="reply.author.name" />
                    <div class="flex-1">
                      <div class="flex items-center gap-2">
                        <span class="font-medium text-sm">{{ reply.author.name }}</span>
                        <span class="text-xs text-muted-foreground">{{ formatRelativeTime(reply.created_at) }}</span>
                      </div>
                      <p class="text-sm mt-1">{{ reply.content }}</p>
                    </div>
                  </div>
                </div>
                <p v-else class="text-sm text-muted-foreground text-center py-4">No discussion yet. Be the first to ask a question!</p>
              </div>
            </div>
          </TabsContent>

          <TabsContent value="transcript" class="flex-1 overflow-y-auto p-4">
            <div v-if="lesson.transcript" class="space-y-2 max-h-[600px] overflow-y-auto">
              <h4 class="font-medium pb-2 border-b border-border">Transcript</h4>
              <div class="space-y-1">
                <button v-for="(segment, index) in lesson.transcript" :key="index" 
                  @click="seekToTime(segment.start)"
                  class="w-full text-left p-2 rounded-lg hover:bg-accent transition-colors flex items-center gap-2"
                  :class="{ 'bg-primary-50 dark:bg-primary-900/30': currentTime >= segment.start && currentTime < (segment.end || duration) }"
                >
                  <span class="text-xs font-mono text-muted-foreground w-16">{{ formatTime(segment.start) }}</span>
                  <span class="text-sm">{{ segment.text }}</span>
                </button>
              </div>
            </div>
            <p v-else class="text-sm text-muted-foreground text-center py-8">No transcript available</p>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  </div>
</template>

<script>
const SkipBackIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M11 19l-7-7 7-7m8 14l-7-7 7-7' })]) }})
const SkipForwardIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 5l7 7-7 7M5 5l7 7-7 7' })]) }})
const SpeedIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 10V3L4 14h7v7l9-11h-7z' })]) }})
const PictureInPictureIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 7V4H6l-2 2v11l2 2h14l2-2V9l-2-2z' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10 15l5-5-5-5' })]) }})
const CaptionsIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('rect', { x: '2', y: '4', width: '20', height: '16', rx: '2', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10 10h4M10 14h4M6 10h4' })]) }})
const AirplayIcon = defineComponent({ render() { return h('svg', { class: 'h-5 w-5', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 17H4a2 2 0 01-2-2V5a2 2 0 012-2h16a2 2 0 012 2v10a2 2 0 01-2 2h-1' }), h('polygon', { points: '12 15 17 21 7 21 12 15' })]) }})
const DownloadIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M7 10l5 5 5-5M17 10v10' })]) }})
const BookmarkIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z' })]) }})
const FlagIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1v12z' })]) }})
const Share2Icon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('circle', { cx: '18', cy: '5', r: '3', 'stroke-width': '2' }), h('circle', { cx: '6', cy: '12', r: '3', 'stroke-width': '2' }), h('circle', { cx: '18', cy: '19', r: '3', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8.59 13.51l-3.29 3.29M15.41 10.49l3.29 3.29M15.41 19.51l-3.29-3.29M8.59 16.51l3.29-3.29' })]) }})
const ExternalLinkIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M15 3h6v6' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M10 14L21 3' })]) }})
const SaveIcon = defineComponent({ render() { return h('svg', { class: 'h-3 w-3', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M19 21H5a2 2 0 01-2-2V5a2 2 0 012-2h11l5 5v11a2 2 0 01-2 2z' }), h('polyline', { points: '17 21 17 13 7 13 7 21', 'stroke-width': '2' }), h('polyline', { points: '7 3 7 8 15 8', 'stroke-width': '2' })]) }})
</script>