import { defineConfig } from 'vite'
import laravel from 'laravel-vite-plugin'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig({
  plugins: [
    laravel({
      input: [
        'resources/css/app.css',
        'resources/js/app.ts',
      ],
      ssr: 'resources/js/ssr.ts',
      refresh: true,
    }),
    vue({
      template: {
        transformAssetUrls: {
          base: null,
          includeAbsolute: false,
        },
      },
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './resources/js'),
      'ziggy-js': path.resolve(__dirname, 'vendor/tightenco/ziggy/dist/vue.m.js'),
    },
  },
  build: {
    target: 'es2022',
    minify: 'esbuild',
    cssMinify: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['vue', 'vue-router', 'pinia', '@inertiajs/vue3'],
          ui: ['@headlessui/vue', '@heroicons/vue', 'radix-vue'],
          charts: ['chart.js', 'vue-chartjs'],
          utils: ['date-fns', 'lodash-es', 'zod', 'fuse.js'],
        },
      },
    },
    chunkSizeWarningLimit: 1000,
  },
  server: {
    hmr: {
      host: 'localhost',
    },
  },
  optimizeDeps: {
    include: ['vue', 'vue-router', 'pinia', '@inertiajs/vue3'],
  },
})