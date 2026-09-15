<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ref } from 'vue'

const router = useRouter()
const query = ref('')

function submitSearch() {
  if (query.value.trim()) router.push({ name: 'search', query: { q: query.value.trim() } })
}

/** Clears every persisted split size/collapse state and reloads. */
function resetLayout(): void {
  if (!confirm('Reset all panel sizes and collapse states to defaults?')) return
  const keys: string[] = []
  for (let i = 0; i < localStorage.length; i++) {
    const key = localStorage.key(i)
    if (key && key.startsWith('rm-split:')) keys.push(key)
  }
  keys.forEach((key) => localStorage.removeItem(key))
  location.reload()
}
</script>

<template>
  <header
    class="flex h-14 shrink-0 items-center gap-4 border-b border-slate-200 bg-white px-6 dark:border-slate-800 dark:bg-[#101a2e]"
  >
    <form class="w-full max-w-md" role="search" @submit.prevent="submitSearch">
      <input
        v-model="query"
        type="search"
        placeholder="Search lessons, objectives, code smells…"
        aria-label="Search lessons"
        class="input"
      />
    </form>

    <button
      class="btn-ghost ml-auto !py-1 text-xs"
      title="Restore default panel sizes and collapse states"
      @click="resetLayout"
    >
      ⧉ Reset layout
    </button>

    <span class="badge bg-teal-600/10 text-teal-700 dark:text-teal-300">100% offline</span>
  </header>
</template>

