<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { cn } from '@/utils/cn'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { 
  CalendarIcon, DownloadIcon, RefreshCwIcon, TrendingUpIcon,
  UsersIcon, BookOpenIcon, ClockIcon, AwardIcon,
  ChevronLeftIcon, ChevronRightIcon, BarChartIcon, LineChartIcon, PieChartIcon
} from '@/components/ui/icons'
import { formatDate } from '@/utils/cn'

interface Props {
  data?: any
  loading?: boolean
  onExport?: (format: string) => void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

const timeRange = ref('30d')
const metric = ref('enrollments')
const chartType = ref('line')
const dateRange = ref({ start: null, end: null })

const timeRanges = [
  { value: '7d', label: 'Last 7 days' },
  { value: '30d', label: 'Last 30 days' },
  { value: '90d', label: 'Last 90 days' },
  { value: '1y', label: 'Last year' },
  { value: 'custom', label: 'Custom range' },
]

const metrics = [
  { value: 'enrollments', label: 'Enrollments', icon: UsersIcon },
  { value: 'completions', label: 'Course Completions', icon: CheckCircleIcon },
  { value: 'revenue', label: 'Revenue', icon: 'DollarSign' },
  { value: 'engagement', label: 'Engagement', icon: 'Activity' },
  { value: 'completion_rate', label: 'Completion Rate', icon: TargetIcon },
  { value: 'active_users', label: 'Active Users', icon: UsersIcon },
]

const chartTypes = [
  { value: 'line', label: 'Line', icon: LineChartIcon },
  { value: 'bar', label: 'Bar', icon: BarChartIcon },
  { value: 'area', label: 'Area', icon: 'AreaChart' },
  { value: 'pie', label: 'Pie', icon: PieChartIcon },
]

// Mock chart data
const generateMockData = (range: string, metricType: string) => {
  const days = range === '7d' ? 7 : range === '30d' ? 30 : range === '90d' ? 90 : 365
  const data = []
  const now = new Date()
  
  for (let i = days - 1; i >= 0; i--) {
    const date = new Date(now)
    date.setDate(date.getDate() - i)
    
    let baseValue = 0
    switch (metricType) {
      case 'enrollments': baseValue = 50 + Math.random() * 100; break
      case 'completions': baseValue = 20 + Math.random() * 80; break
      case 'revenue': baseValue = 1000 + Math.random() * 5000; break
      case 'engagement': baseValue = 100 + Math.random() * 500; break
      case 'completion_rate': baseValue = 60 + Math.random() * 40; break
      case 'active_users': baseValue = 200 + Math.random() * 300; break
    }
    
    data.push({
      date: date.toISOString().split('T')[0],
      value: Math.round(baseValue),
      label: date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
    })
  }
  
  return data
}

const chartData = computed(() => generateMockData(timeRange.value, metric.value))

const chartConfig = computed(() => ({
  type: chartType.value,
  data: {
    labels: chartData.value.map(d => d.label),
    datasets: [{
      label: metrics.find(m => m.value === metric.value)?.label || metric.value,
      data: chartData.value.map(d => d.value),
      borderColor: 'hsl(var(--primary))',
      backgroundColor: 'hsl(var(--primary) / 0.1)',
      fill: chartType.value === 'area',
      tension: 0.4,
      pointRadius: 3,
      pointHoverRadius: 5,
    }],
  },
  options: {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: { display: false },
      tooltip: {
        mode: 'index',
        intersect: false,
        backgroundColor: 'hsl(var(--popover))',
        titleColor: 'hsl(var(--popover-foreground))',
        bodyColor: 'hsl(var(--popover-foreground))',
        borderColor: 'hsl(var(--border))',
        borderWidth: 1,
        padding: 12,
        displayColors: false,
      },
    },
    scales: {
      x: {
        grid: { display: false },
        ticks: { color: 'hsl(var(--muted-foreground))', font: { size: 11 } },
      },
      y: {
        grid: { color: 'hsl(var(--border))' },
        ticks: { color: 'hsl(var(--muted-foreground))', font: { size: 11 } },
        beginAtZero: true,
      },
    },
    interaction: { mode: 'nearest', axis: 'x', intersect: false },
  },
})

const summaryStats = computed(() => {
  const total = chartData.value.reduce((sum, d) => sum + d.value, 0)
  const avg = total / chartData.value.length
  const latest = chartData.value[chartData.value.length - 1]?.value || 0
  const previous = chartData.value[chartData.value.length - 2]?.value || 0
  const change = previous > 0 ? ((latest - previous) / previous * 100).toFixed(1) : '0'
  
  return {
    total: Math.round(total),
    average: Math.round(avg),
    latest,
    change: parseFloat(change),
    trend: latest >= previous ? 'up' : 'down',
  }
})
</script>

<template>
  <Card class="h-full">
    <CardHeader class="pb-3">
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-primary-100 dark:bg-primary-900/30">
            <BarChartIcon class="h-5 w-5 text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <CardTitle class="text-lg">Analytics Dashboard</CardTitle>
            <CardDescription>Track key metrics and trends</CardDescription>
          </div>
        </div>
        
        <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
          <Select v-model="timeRange" class="w-40">
            <SelectTrigger>
              <SelectValue placeholder="Time Range" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="r in timeRanges" :key="r.value" :value="r.value">{{ r.label }}</SelectItem>
            </SelectContent>
          </Select>
          
          <Select v-model="metric" class="w-44">
            <SelectTrigger>
              <SelectValue placeholder="Metric" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="m in metrics" :key="m.value" :value="m.value">
                <component :is="m.icon" class="h-3 w-3 mr-2" />
                {{ m.label }}
              </SelectItem>
            </SelectContent>
          </Select>
          
          <Select v-model="chartType" class="w-32">
            <SelectTrigger>
              <SelectValue placeholder="Chart Type" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="c in chartTypes" :key="c.value" :value="c.value">
                <component :is="c.icon" class="h-3 w-3 mr-2" />
                {{ c.label }}
              </SelectItem>
            </SelectContent>
          </Select>
          
          <div class="flex items-center gap-2">
            <Button variant="outline" size="icon" @click="refreshData" :disabled="loading" class="gap-1">
              <RefreshCwIcon class="h-4 w-4" :class="{ 'animate-spin': loading }" />
            </Button>
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button variant="outline" size="icon" class="gap-1">
                  <DownloadIcon class="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem @click="exportData('csv')">
                  <FileSpreadsheetIcon class="h-4 w-4" />
                  Export CSV
                </DropdownMenuItem>
                <DropdownMenuItem @click="exportData('json')">
                  <FileJsonIcon class="h-4 w-4" />
                  Export JSON
                </DropdownMenuItem>
                <DropdownMenuItem @click="exportData('pdf')">
                  <FileTextIcon class="h-4 w-4" />
                  Export PDF
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      </CardHeader>
      
      <CardContent class="pt-0">
        <!-- Summary Stats -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <StatCard 
            label="Total" 
            :value="summaryStats.total.toLocaleString()" 
            :change="summaryStats.change" 
            :trend="summaryStats.trend"
            icon="UsersIcon"
            color="primary"
          />
          <StatCard 
            label="Average" 
            :value="summaryStats.average.toLocaleString()" 
            icon="TrendingUpIcon"
            color="success"
          />
          <StatCard 
            label="Latest" 
            :value="summaryStats.latest.toLocaleString()" 
            icon="ClockIcon"
            color="info"
          />
          <StatCard 
            label="Change" 
            :value="summaryStats.change + '%'" 
            :trend="summaryStats.trend"
            icon="TrendingUpIcon"
            color="warning"
          />
        </div>
        
        <!-- Chart Container -->
        <div class="h-[400px] relative">
          <canvas 
            ref="chartCanvas" 
            class="w-full h-full" 
            @click="handleChartClick"
          ></canvas>
          
          <!-- Chart Controls Overlay -->
          <div class="absolute top-4 right-4 flex items-center gap-2">
            <Tooltip>
              <TooltipTrigger as-child>
                <Button variant="ghost" size="icon" class="h-8 w-8">
                  <ExpandIcon class="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Expand chart</TooltipContent>
            </Tooltip>
          </div>
        </div>
        
        <!-- Data Table -->
        <div class="mt-6 border-t border-border pt-6">
          <h4 class="font-medium mb-4">Data Points</h4>
          <div class="overflow-x-auto">
            <table class="w-full text-sm">
              <thead>
                <tr class="border-b border-border">
                  <th class="text-left p-3 text-muted-foreground">Date</th>
                  <th class="text-right p-3 text-muted-foreground">Value</th>
                  <th class="text-right p-3 text-muted-foreground">Change</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(point, index) in chartData.slice(-10).reverse()" :key="point.date" class="border-b border-border/50 hover:bg-accent/50">
                  <td class="p-3">{{ point.label }}</td>
                  <td class="text-right p-3 font-mono tabular-nums">{{ point.value.toLocaleString() }}</td>
                  <td class="text-right p-3">
                    <span v-if="index > 0" :class="point.value >= chartData[chartData.length - 10 + index - 1]?.value ? 'text-success-600' : 'text-destructive-600'">
                      {{ point.value >= chartData[chartData.length - 10 + index - 1]?.value ? '+' : '' }}
                      {{ point.value - (chartData[chartData.length - 10 + index - 1]?.value || 0) }}
                    </span>
                    <span v-else class="text-muted-foreground">—</span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script>
const StatCard = defineComponent({
  props: ['label', 'value', 'change', 'trend', 'icon', 'color'],
  setup(props) {
    const icons = {
      UsersIcon: h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' })]),
      TrendingUpIcon: h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 7h8m0 0v8m0-8l-8 8-4-4-6 6' })]),
      ClockIcon: h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' })]),
    }
    
    const colorClasses = {
      primary: 'bg-primary-100 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400',
      success: 'bg-success-100 dark:bg-success-900/30 text-success-600 dark:text-success-400',
      warning: 'bg-warning-100 dark:bg-warning-900/30 text-warning-600 dark:text-warning-400',
      info: 'bg-info-100 dark:bg-info-900/30 text-info-600 dark:text-info-400',
      warning: 'bg-warning-100 dark:bg-warning-900/30 text-warning-600 dark:text-warning-400',
    }
    
    return () => h('div', { class: 'p-4 rounded-xl bg-card border border-border' }, [
      h('div', { class: 'flex items-center justify-between mb-2' }, [
        h('div', { class: cn('p-2 rounded-lg', colorClasses[props.color]) }, [
          h('component', { is: icons[props.icon as keyof typeof icons] || icons.UsersIcon, class: 'h-5 w-5' })
        ]),
        h('Badge', { variant: props.trend === 'up' ? 'success' : 'destructive', class: 'gap-1' }, [
          props.trend === 'up' ? h('TrendingUpIcon', { class: 'h-3 w-3' }) : h('TrendingUpIcon', { class: 'h-3 w-3 rotate-180' }),
          h('span', { class: 'text-xs' }, props.change + '%')
        ]),
      ]),
      h('p', { class: 'text-2xl font-display font-bold mt-2' }, props.value),
      h('p', { class: 'text-xs text-muted-foreground' }, props.label),
    ])
  }
})

const FileSpreadsheetIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z' })]) }})
const FileJsonIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M8 17H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2.5M8 17v-2a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V17' })]) }})
const FileTextIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' })]) }})
const AreaChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 3v18h18' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 21V9a2 2 0 012-2h4l4-4 4 4h4v12a2 2 0 01-2 2H5a2 2 0 01-2-2v-8' })]) }})
const PieChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 12a9 9 0 01-9 9c2.52 0 4.93-1.04 6.74-2.8L12 8.29V21a9 9 0 009-9c0-2.52-1.04-4.93-2.8-6.74L15.71 12H21a9 9 0 01-9 9z' })]) }})
const DollarSign = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('line', { x1: '12', y1: '2', x2: '12', y2: '22', 'stroke-width': '2' }), h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6' })]) }})
const Activity = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 2L3 14h9l-1 8 10-12h-9l11-4 10-12H6l-1 8 10-12' })]) }})
const LineChartIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M3 3v18h18M9 13l6-6 6 6m-12 0h18' })]) }})
const TrendingUpIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M13 7h8m0 0v8m0-8l-8 8-4-4-6 6' })]) }})
const CheckCircleIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z' })]) }})
const ExpandIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 8V4m0 0h4M4 8h4m-4 8v4m0 0h4m-4 0v-4m8 0H4m0 0h4m0 0v-4m8 8H4m0 0h4m0 0v4m8 0h4m0 0v-4' })]) }})
const DownloadIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M7 10l5 5 5-5M17 10v10' })]) }})
const RefreshCwIcon = defineComponent({ render() { return h('svg', { class: 'h-4 w-4', fill: 'none', stroke: 'currentColor', viewBox: '0 0 24 24' }, [h('path', { 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'stroke-width': '2', d: 'M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15' })]) }})
</script>