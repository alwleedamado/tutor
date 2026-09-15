<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useLessonsStore } from '@/stores/lessons.store'
import { useLibraryStore } from '@/stores/library.store'
import { useSkillStore } from '@/stores/skill.store'

const route = useRoute()
const lessons = useLessonsStore()
const library = useLibraryStore()
const skills = useSkillStore()

onMounted(() => {
  // Notes are user data — load them so offline search can match locally.
  void library.ensureNotes()
})

const query = computed(() => String(route.query.q ?? '').trim().toLowerCase())

interface Hit {
  to: string
  title: string
  where: string
  snippet: string
  kind: 'lesson' | 'track' | 'module' | 'project' | 'skill' | 'note' | 'example'
}

function snippetFrom(text: string, q: string): string {
  const plain = text.replace(/[#*`>]/g, '')
  const at = plain.toLowerCase().indexOf(q)
  if (at < 0) return plain.slice(0, 120)
  return `…${plain.slice(Math.max(0, at - 40), at + 80)}…`
}

const KIND_LABEL: Record<Hit['kind'], string> = {
  lesson: 'Lesson',
  track: 'Track',
  module: 'Module',
  project: 'Project',
  skill: 'Skill',
  note: 'Note',
  example: 'Code example'
}

const results = computed<Hit[]>(() => {
  const q = query.value
  if (q.length < 2) return []
  const hits: Hit[] = []
  const push = (hit: Hit) => {
    if (hits.length < 80) hits.push(hit)
  }

  // Tracks (title + description).
  for (const track of lessons.tracks) {
    const haystack = `${track.title} ${track.description}`.toLowerCase()
    if (haystack.includes(q)) {
      push({
        to: `/${track.id}`,
        title: track.title,
        where: track.category,
        snippet: track.description,
        kind: 'track'
      })
    }
    // Modules (title + summary).
    for (const moduleDef of track.modules) {
      const moduleHay = `${moduleDef.title} ${moduleDef.summary}`.toLowerCase()
      if (moduleHay.includes(q)) {
        push({
          to: `/${track.id}/${moduleDef.slug}`,
          title: moduleDef.title,
          where: `${track.title} · Module`,
          snippet: moduleDef.summary,
          kind: 'module'
        })
      }
      // Lessons (title, body, objectives, smells, skill tags).
      for (const lesson of moduleDef.lessons) {
        const lessonHay =
          `${lesson.title} ${lesson.markdown} ${lesson.objectives.join(' ')} ` +
          `${lesson.smells.map((s) => s.name).join(' ')} ${lesson.skills.join(' ')}`.toLowerCase()
        if (lessonHay.includes(q)) {
          push({
            to: `/${track.id}/${moduleDef.slug}/${lesson.slug}`,
            title: lesson.title,
            where: `${track.title} · ${moduleDef.title}`,
            snippet: snippetFrom(lesson.markdown, q),
            kind: 'lesson'
          })
        }
        // Embedded code examples.
        for (const example of lesson.examples) {
          if (example.code.toLowerCase().includes(q) || example.name.toLowerCase().includes(q)) {
            push({
              to: `/${track.id}/${moduleDef.slug}/${lesson.slug}`,
              title: example.name,
              where: `${lesson.title} · Example`,
              snippet: snippetFrom(example.code, q),
              kind: 'example'
            })
          }
        }
      }
    }
  }

  // Projects (title + summary + requirements).
  for (const project of lessons.projects) {
    const hay = `${project.title} ${project.summary} ${project.requirements.join(' ')}`.toLowerCase()
    if (hay.includes(q)) {
      push({
        to: `/project/${project.id}`,
        title: project.title,
        where: 'Project',
        snippet: project.summary,
        kind: 'project'
      })
    }
  }

  // Skills (label + summary).
  for (const state of skills.states) {
    const node = lessons.skillById(state.id)
    const hay = `${state.label} ${node?.summary ?? ''}`.toLowerCase()
    if (hay.includes(q)) {
      push({
        to: '/tracks',
        title: state.label,
        where: `Skill · ${state.category}${state.demonstrated ? ' · demonstrated' : ''}`,
        snippet: node?.summary || 'Skill in the learning graph.',
        kind: 'skill'
      })
    }
  }

  // Student notes (local only).
  for (const note of library.notes) {
    const hay = `${note.title} ${note.body}`.toLowerCase()
    if (hay.includes(q)) {
      push({
        to: '/notes',
        title: note.title || 'Untitled note',
        where: 'Your note',
        snippet: snippetFrom(note.body, q),
        kind: 'note'
      })
    }
  }

  return hits
})
</script>

<template>
  <div class="mx-auto max-w-4xl space-y-4">
    <h1 class="text-2xl font-bold text-slate-900 dark:text-white">
      {{ query ? `Results for “${query}”` : 'Search' }}
    </h1>

    <p v-if="query.length < 2" class="text-sm italic text-slate-400">Type at least two characters.</p>
    <p v-else-if="!results.length" class="text-sm italic text-slate-400">
      No matches in the embedded curriculum, projects, skills, or your notes.
    </p>

    <ul class="space-y-2">
      <li v-for="hit in results" :key="hit.kind + hit.to + hit.title">
        <router-link :to="hit.to" class="card block p-3 hover:border-teal-500/50">
          <div class="flex items-baseline justify-between gap-2">
            <p class="font-medium text-slate-900 dark:text-white">{{ hit.title }}</p>
            <span class="badge shrink-0 bg-slate-200 text-[10px] uppercase text-slate-500 dark:bg-slate-800 dark:text-slate-400">
              {{ KIND_LABEL[hit.kind] }}
            </span>
          </div>
          <p class="text-xs text-teal-600 dark:text-teal-300">{{ hit.where }}</p>
          <p class="mt-1 line-clamp-2 text-xs text-slate-500 dark:text-slate-400">{{ hit.snippet }}</p>
        </router-link>
      </li>
    </ul>
  </div>
</template>
