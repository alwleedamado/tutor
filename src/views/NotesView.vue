<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useLibraryStore } from '@/stores/library.store'
import type { NoteDto } from '@/types/domain'

const library = useLibraryStore()
const filter = ref<'all' | 'lesson' | 'project' | 'general'>('all')
const editingId = ref<number | null>(null)
const editBody = ref('')

onMounted(() => void library.ensureNotes())

async function startEdit(note: NoteDto) {
  editingId.value = note.id
  editBody.value = note.body
}

async function saveEdit(note: NoteDto) {
  await library.saveNote({ ...note, body: editBody.value })
  editingId.value = null
}
</script>

<template>
  <div class="mx-auto max-w-4xl space-y-4">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-slate-900 dark:text-white">📝 Notes</h1>
      <select v-model="filter" class="input max-w-40" aria-label="Filter notes by scope">
        <option value="all">All scopes</option>
        <option value="lesson">Lessons</option>
        <option value="project">Projects</option>
        <option value="general">General</option>
      </select>
    </div>

    <p v-if="!library.notes.length" class="text-sm italic text-slate-400">
      No notes yet — capture insights from the Notes tab inside any lesson.
    </p>

    <ul class="space-y-2">
      <li v-for="note in library.notes.filter((n) => filter === 'all' || n.scopeType === filter)" :key="note.id" class="card p-4">
        <div class="flex items-start justify-between gap-3">
          <div class="min-w-0">
            <p class="truncate font-medium text-slate-900 dark:text-white">{{ note.title || '(untitled)' }}</p>
            <p class="text-xs text-slate-400">{{ note.scopeType }} · {{ new Date(note.updatedAt * 1000).toLocaleString() }}</p>
          </div>
          <div class="flex shrink-0 gap-1">
            <button v-if="editingId !== note.id" class="btn-ghost !py-1 text-xs" @click="startEdit(note)">Edit</button>
            <button v-else class="btn-primary !py-1 text-xs" @click="saveEdit(note)">Save</button>
            <button class="btn-ghost !py-1 text-xs" @click="library.deleteNote(note.id)">Delete</button>
          </div>
        </div>
        <textarea
          v-if="editingId === note.id"
          v-model="editBody"
          rows="4"
          class="input mt-2 font-mono text-xs"
          aria-label="Note body"
        />
        <p v-else class="mt-2 whitespace-pre-wrap text-sm text-slate-700 dark:text-slate-300">{{ note.body }}</p>
      </li>
    </ul>
  </div>
</template>
