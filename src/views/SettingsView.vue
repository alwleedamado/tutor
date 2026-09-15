<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings.store'

const settings = useSettingsStore()

async function resetEverything() {
  const { ipc } = await import('@/services/ipc')
  if (!confirm('Delete ALL local data — progress, notes, bookmarks, settings?')) return
  if (!confirm('This cannot be undone. Final confirmation: wipe everything?')) return
  await ipc.resetAllData()
  location.reload()
}
</script>

<template>
  <div class="mx-auto max-w-3xl space-y-6">
    <h1 class="text-2xl font-bold text-slate-900 dark:text-white">Settings</h1>

    <section class="card space-y-4 p-5">
      <div>
        <h2 class="mb-2 font-semibold text-slate-900 dark:text-white">Theme</h2>
        <div class="flex gap-2" role="radiogroup" aria-label="Theme">
          <button
            v-for="theme in (['dark', 'light', 'system'] as const)"
            :key="theme"
            class="btn"
            role="radio"
            :aria-checked="settings.settings.theme === theme"
            :class="settings.settings.theme === theme ? 'btn-primary' : 'btn-ghost'"
            @click="settings.patch({ theme })"
          >
            {{ theme === 'dark' ? '🌙' : theme === 'light' ? '☀️' : '🖥️' }} {{ theme }}
          </button>
        </div>
      </div>

      <label class="block">
        <span class="text-sm font-medium">Base font size — {{ settings.settings.fontSize }}px</span>
        <input
          type="range"
          min="10"
          max="28"
          :value="settings.settings.fontSize"
          class="mt-1 w-full accent-teal-600"
          aria-label="Font size"
          @change="settings.patch({ fontSize: Number(($event.target as HTMLInputElement).value) })"
        />
      </label>

      <label class="block">
        <span class="text-sm font-medium">Editor tab size</span>
        <select
          class="input mt-1 max-w-32"
          aria-label="Tab size"
          @change="settings.patch({ tabSize: Number(($event.target as HTMLSelectElement).value) })"
        >
          <option v-for="size in [2, 4, 8]" :key="size" :value="size" :selected="settings.settings.tabSize === size">
            {{ size }}
          </option>
        </select>
      </label>

      <div class="space-y-2">
        <label class="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            class="accent-teal-600"
            :checked="settings.settings.fontLigatures"
            @change="settings.patch({ fontLigatures: ($event.target as HTMLInputElement).checked })"
          />
          Font ligatures in the editor
        </label>
        <label class="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            class="accent-teal-600"
            :checked="settings.settings.reducedMotion"
            @change="settings.patch({ reducedMotion: ($event.target as HTMLInputElement).checked })"
          />
          Reduced motion (accessibility)
        </label>
        <label class="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            class="accent-teal-600"
            :checked="settings.settings.highContrast"
            @change="settings.patch({ highContrast: ($event.target as HTMLInputElement).checked })"
          />
          High contrast mode
        </label>
      </div>
    </section>

    <section class="card border-red-300 p-5 dark:border-red-900">
      <h2 class="font-semibold text-red-700 dark:text-red-300">Danger zone</h2>
      <p class="mt-1 text-sm text-slate-600 dark:text-slate-300">
        Wipes every local artifact stored in SQLite. The curriculum and schema stay intact.
      </p>
      <button class="btn-danger mt-3" @click="resetEverything">Reset all data…</button>
    </section>
  </div>
</template>
