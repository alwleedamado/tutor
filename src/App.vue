<script setup lang="ts">
import { useLessonsStore } from '@/stores/lessons.store'
import AppSidebar from '@/components/layout/AppSidebar.vue'
import TopBar from '@/components/layout/TopBar.vue'

// The router guard awaits bootstrap; this call warms the same single flight.
const lessons = useLessonsStore()
void lessons.ensureLoaded()
</script>

<template>
  <!-- Bootstrap failure: no Rust backend attached (browser preview) or backend error. -->
  <div v-if="lessons.error" class="flex h-screen items-center justify-center p-8" role="alert">
    <div class="card max-w-xl space-y-4 p-8 text-center">
      <p class="text-5xl" aria-hidden="true">🖥️</p>
      <h1 class="text-lg font-bold text-slate-900 dark:text-white">Learning library unavailable</h1>
      <p class="text-sm leading-relaxed text-slate-600 dark:text-slate-300">{{ lessons.error }}</p>
      <ol class="mx-auto max-w-md space-y-1 rounded-lg bg-slate-100 p-3 text-left font-mono text-xs text-slate-600 dark:bg-slate-800 dark:text-slate-300">
        <li>1. Stop the plain Vite server (Ctrl+C)</li>
        <li>2. Run: npm run tauri dev</li>
      </ol>
      <button class="btn-primary mx-auto" data-testid="bootstrap-retry" @click="lessons.retry()">
        Retry
      </button>
    </div>
  </div>

  <div v-else class="flex h-screen overflow-hidden">
    <AppSidebar />
    <div class="flex min-w-0 flex-1 flex-col">
      <TopBar />
      <main
        class="min-h-0 flex-1 overflow-y-auto bg-slate-50 px-6 py-5 text-slate-800 dark:bg-[#0b1220] dark:text-slate-200"
      >
        <router-view />
      </main>
    </div>
  </div>
</template>

