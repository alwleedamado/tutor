<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useLibraryStore } from '@/stores/library.store'

const props = defineProps<{ scopeType: string; scopeId: string }>()

const library = useLibraryStore()
const title = ref('')
const body = ref('')
const saving = ref(false)

onMounted(async () => {
  await library.ensureNotes(props.scopeType, props.scopeId)
})

async function save() {
  if (!body.value.trim()) return
  saving.value = true
  try {
    const existing = library.notesForScope(props.scopeType, props.scopeId)[0]
    await library.saveNote({
      id: existing?.id ?? null,
      scopeType: props.scopeType,
      scopeId: props.scopeId,
      title: title.value.trim(),
      body: body.value
    })
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <div class="space-y-2">
    <input v-model="title" placeholder="Note title (optional)" class="input" aria-label="Note title" />
    <textarea
      v-model="body"
      rows="5"
      placeholder="What clicked? What still feels fuzzy? Everything stays on this machine."
      class="input font-mono text-xs"
      aria-label="Lesson notes"
    />
    <div class="flex items-center gap-2">
      <button class="btn-primary !py-1" :disabled="saving || !body.trim()" @click="save">
        {{ saving ? 'Saving…' : 'Save note' }}
      </button>
      <span v-if="library.notesForScope(scopeType, scopeId).length" class="text-xs text-slate-400">
        Saved · updates the existing note for this lesson
      </span>
    </div>
  </div>
</template>
