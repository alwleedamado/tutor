<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import type { Quiz } from '@/types/domain'
import { useProgressStore } from '@/stores/progress.store'

const props = defineProps<{
  quiz: Quiz
  trackId: string
  moduleId: string
  lessonId: string
}>()

const progress = useProgressStore()
const answers = reactive<Record<string, number>>({})
const submitted = ref(false)
const lastScore = ref<{ score: number; total: number; passed: boolean } | null>(null)

const answeredCount = computed(
  () => props.quiz.questions.filter((q) => answers[q.id] !== undefined).length
)
const allAnswered = computed(() => answeredCount.value === props.quiz.questions.length)

function submit() {
  if (!allAnswered.value || submitted.value) return
  const total = props.quiz.questions.length
  const score = props.quiz.questions.filter((q) => answers[q.id] === q.answerIndex).length
  const passed = total > 0 && (score * 100) / total >= props.quiz.passScore

  lastScore.value = { score, total, passed }
  submitted.value = true
  void progress.recordQuiz(props.trackId, props.moduleId, props.lessonId, props.quiz.id, score, total)
}

function retake() {
  for (const key of Object.keys(answers)) delete answers[key]
  submitted.value = false
  lastScore.value = null
}
</script>

<template>
  <div class="space-y-4">
    <p class="text-sm text-slate-500 dark:text-slate-400">
      {{ quiz.questions.length }} questions · pass mark {{ quiz.passScore }}%
    </p>

    <fieldset v-for="(question, qi) in quiz.questions" :key="question.id" class="card p-3">
      <legend class="sr-only">Question {{ qi + 1 }}</legend>
      <p class="mb-2 font-medium text-slate-800 dark:text-slate-200">{{ qi + 1 }}. {{ question.prompt }}</p>
      <pre v-if="question.code" class="mb-2 overflow-x-auto rounded bg-slate-950 p-3 text-xs text-slate-100">{{ question.code }}</pre>

      <ul class="space-y-1.5">
        <li v-for="(option, oi) in question.options" :key="oi">
          <label class="flex cursor-pointer items-center gap-2 rounded-lg px-2 py-1 hover:bg-slate-100 dark:hover:bg-slate-800">
            <input
              v-model="answers[question.id]"
              type="radio"
              :name="`q-${question.id}`"
              :value="oi"
              :disabled="submitted"
              class="accent-teal-600"
            />
            <span class="text-sm">{{ option }}</span>
          </label>
        </li>
      </ul>

      <p
        v-if="submitted"
        class="mt-2 rounded-lg px-3 py-2 text-sm"
        :class="
          answers[question.id] === question.answerIndex
            ? 'bg-emerald-600/10 text-emerald-700 dark:text-emerald-300'
            : 'bg-red-600/10 text-red-700 dark:text-red-300'
        "
      >
        {{ answers[question.id] === question.answerIndex ? '✔ Correct.' : `✘ Correct answer: ${question.options[question.answerIndex]}` }}
        <span v-if="question.explanation" class="block text-slate-600 dark:text-slate-400">{{ question.explanation }}</span>
      </p>
    </fieldset>

    <div class="flex items-center gap-3">
      <button
        v-if="!submitted"
        class="btn-primary"
        :disabled="!allAnswered"
        data-testid="quiz-submit"
        @click="submit"
      >
        Submit ({{ answeredCount }}/{{ quiz.questions.length }})
      </button>
      <template v-else>
        <span
          class="badge text-base"
          :class="lastScore?.passed ? 'bg-emerald-600/15 text-emerald-700 dark:text-emerald-300' : 'bg-red-600/15 text-red-700 dark:text-red-300'"
          data-testid="quiz-result"
        >
          {{ lastScore?.passed ? 'Passed' : 'Keep practicing' }} ·
          {{ lastScore?.score }}/{{ lastScore?.total }}
        </span>
        <button class="btn-ghost" @click="retake">Retake</button>
      </template>
    </div>
  </div>
</template>
