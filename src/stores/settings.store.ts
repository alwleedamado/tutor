import { defineStore } from 'pinia'

import { ipc } from '@/services/ipc'
import { DEFAULT_SETTINGS } from '@/types/domain'
import type { Settings } from '@/types/domain'

interface SettingsState {
  settings: Settings
}

/** Owns user preferences; persists through the settings service. */
export const useSettingsStore = defineStore('settings', {
  state: (): SettingsState => ({
    settings: { ...DEFAULT_SETTINGS }
  }),

  actions: {
    hydrate(settings: Settings) {
      this.settings = { ...DEFAULT_SETTINGS, ...settings }
      this.applyToDocument()
    },

    async patch(partial: Partial<Settings>) {
      this.settings = { ...this.settings, ...partial }
      this.applyToDocument()
      await ipc.saveSettings(this.settings)
    },

    /** Reflect preferences onto <html> (theme, motion, contrast, base font). */
    applyToDocument() {
      if (typeof document === 'undefined') return
      const s = this.settings
      const root = document.documentElement
      const prefersDark =
        window.matchMedia?.('(prefers-color-scheme: dark)').matches ?? true
      root.classList.toggle('dark', s.theme === 'dark' || (s.theme === 'system' && prefersDark))
      root.dataset.highContrast = String(s.highContrast)
      root.dataset.reducedMotion = String(s.reducedMotion)
      root.style.fontSize = `${s.fontSize}px`
    }
  }
})
