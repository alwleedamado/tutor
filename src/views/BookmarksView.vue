<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useLibraryStore } from '@/stores/library.store'
import { useLessonsStore } from '@/stores/lessons.store'

const library = useLibraryStore()
const lessons = useLessonsStore()

onMounted(async () => {
  await library.ensureBookmarks()
})

const items = computed(() =>
  library.bookmarks.map((bookmark) => {
    if (bookmark.itemType !== 'lesson') {
      return { key: `${bookmark.itemType}:${bookmark.itemId}`, label: bookmark.itemId, where: 'Project', to: `/project/${bookmark.itemId}` }
    }
    const parts = bookmark.itemId.split(':')
    const trackId = parts[0] ?? ''
    const moduleId = parts[1] ?? ''
    const lessonId = parts[2] ?? ''
    const moduleDef = lessons.moduleById.get(moduleId)
    const lesson = moduleDef?.lessons.find((l) => l.id === lessonId)
    return {
      key: bookmark.itemId,
      label: lesson?.title ?? lessonId,
      where: moduleDef?.title ?? moduleId,
      to: lesson ? `/${trackId}/${moduleDef?.slug}/${lesson.slug}` : '/'
    }
  })
)
</script>

<template>
  <div class="mx-auto max-w-4xl space-y-4">
    <h1 class="text-2xl font-bold text-slate-900 dark:text-white">🔖 Bookmarks</h1>
    <p v-if="!items.length" class="text-sm italic text-slate-400">
      Bookmark lessons with the 📑 button in any lesson header.
    </p>
    <ul class="space-y-2">
      <li v-for="item in items" :key="item.key" class="card flex items-center gap-3 p-3">
        <router-link :to="item.to" class="min-w-0 flex-1 hover:text-teal-700 dark:hover:text-teal-300">
          <p class="truncate font-medium">{{ item.label }}</p>
          <p class="text-xs text-slate-500 dark:text-slate-400">{{ item.where }}</p>
        </router-link>
        <button
          class="btn-ghost !py-1 text-xs"
          @click="library.toggleBookmark(item.key.includes('/') ? 'lesson' : 'lesson', item.key)"
        >
          Remove
        </button>
      </li>
    </ul>
  </div>
</template>
