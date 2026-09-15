<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { useLessonsStore } from '@/stores/lessons.store'
import { useProgressStore } from '@/stores/progress.store'

/**
 * `/rust` · `/vue` · `/tauri` land here and bounce to the most useful lesson:
 * first unlocked module with an incomplete lesson, else the module's opener.
 */
const route = useRoute()
const router = useRouter()
const lessons = useLessonsStore()
const progress = useProgressStore()

const label = computed(() => lessons.findTrack(String(route.meta.track))?.title ?? 'track')

onMounted(async () => {
  await lessons.ensureLoaded()
  const trackId = String(route.meta.track)
  const moduleParam = typeof route.params.module === 'string' ? route.params.module : undefined
  const track = lessons.findTrack(trackId)

  let target = '/'
  if (track && track.modules.length > 0) {
    const byIndex = [...track.modules].sort((a, b) => a.index - b.index)
    const pool = moduleParam
      ? byIndex.filter((m) => m.slug === moduleParam || m.id === moduleParam)
      : byIndex
    const candidate =
      pool.find((m) => m.lessons.length > 0 && !progress.isModuleLocked(m.id)) ??
      pool.find((m) => m.lessons.length > 0) ??
      byIndex.find((m) => m.lessons.length > 0)
    if (candidate) {
      const next =
        candidate.lessons.find((l) => !progress.isLessonCompleted(trackId, candidate.id, l.id)) ??
        candidate.lessons[0]
      target = `/${trackId}/${candidate.slug}/${next.slug}`
    }
  }
  await router.replace(target)
})

</script>

<template>
  <p class="animate-pulse py-16 text-center italic text-slate-400">Opening {{ label }}…</p>
</template>
