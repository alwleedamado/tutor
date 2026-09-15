<script setup lang="ts">
import { computed } from 'vue'
import type { Lesson, Module } from '@/types/domain'
import { useProgressStore } from '@/stores/progress.store'

const props = defineProps<{ module: Module; trackId: string; lesson: Lesson }>()
const progress = useProgressStore()

const items = computed(() =>
  props.module.lessons.map((l) => ({
    lesson: l,
    completed: progress.isLessonCompleted(props.trackId, props.module.id, l.id)
  }))
)

const routeFor = (lesson: Lesson) => `/${props.trackId}/${props.module.slug}/${lesson.slug}`
</script>

<template>
  <nav aria-label="Module lessons" class="card p-3 text-sm">
    <p class="mb-2 px-1 text-xs font-semibold uppercase tracking-wide text-slate-400">
      {{ module.title }}
    </p>
    <ul class="space-y-0.5">
      <li v-for="(item, i) in items" :key="item.lesson.id">
        <router-link
          :to="routeFor(item.lesson)"
          class="flex items-center gap-2 rounded-lg px-2 py-1.5 hover:bg-slate-100 dark:hover:bg-slate-800"
          :class="{ 'bg-teal-600/10 text-teal-800 dark:text-teal-300': item.lesson.id === lesson.id }"
          :aria-current="item.lesson.id === lesson.id ? 'page' : undefined"
        >
          <span
            class="flex h-5 w-5 shrink-0 items-center justify-center rounded-full text-[11px] font-bold"
            :class="item.completed ? 'bg-emerald-500 text-white' : 'bg-slate-200 text-slate-600 dark:bg-slate-700 dark:text-slate-300'"
            aria-hidden="true"
          >
            {{ item.completed ? '✓' : i + 1 }}
          </span>
          <span class="truncate">{{ item.lesson.title }}</span>
        </router-link>
      </li>
    </ul>
  </nav>
</template>
