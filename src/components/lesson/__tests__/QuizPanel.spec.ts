import { beforeEach, describe, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia } from 'pinia'

import type { Quiz } from '@/types/domain'
import { recordQuizAttempt } from './helpers'
import QuizPanel from '../QuizPanel.vue'

const quiz: Quiz = {
  id: 'quiz-1',
  passScore: 70,
  questions: [
    { id: 'q1', prompt: 'Which owns x after move?', options: ['a', 'b'], answerIndex: 1, explanation: 'moves transfer ownership' },
    { id: 'q2', prompt: 'Pinia needs mutations?', options: ['yes', 'no'], answerIndex: 1, explanation: 'actions mutate directly' }
  ]
}

function mountQuiz() {
  return mount(QuizPanel, {
    props: { quiz, trackId: 'rust', moduleId: 'rust-01', lessonId: 'rust-01-l01' },
    global: { plugins: [createPinia()] }
  })
}

beforeEach(() => {
  recordQuizAttempt.mockClear()
})

describe('<QuizPanel>', () => {
  it('scores a perfect run as passed and records the attempt', async () => {
    const wrapper = mountQuiz()

    await wrapper.findAll('input[type="radio"]')[1].setValue()
    await wrapper.findAll('input[type="radio"]')[3].setValue()
    await wrapper.find('[data-testid="quiz-submit"]').trigger('click')
    await Promise.resolve()

    expect(wrapper.find('[data-testid="quiz-result"]').text()).toContain('Passed')
    expect(recordQuizAttempt).toHaveBeenCalledWith('rust', 'rust-01', 'rust-01-l01', 'quiz-1', 2, 2)
    wrapper.unmount()
  })

  it('marks an all-wrong run as failed and reveals correct answers', async () => {
    const wrapper = mountQuiz()

    await wrapper.findAll('input[type="radio"]')[0].setValue() // wrong
    await wrapper.findAll('input[type="radio"]')[2].setValue() // wrong
    await wrapper.find('[data-testid="quiz-submit"]').trigger('click')
    await Promise.resolve()

    expect(wrapper.find('[data-testid="quiz-result"]').text()).toContain('Keep practicing')
    expect(wrapper.text()).toContain('Correct answer:')
    expect(recordQuizAttempt).toHaveBeenCalledWith('rust', 'rust-01', 'rust-01-l01', 'quiz-1', 0, 2)
    wrapper.unmount()
  })

  it('disables submit until every question is answered', async () => {
    const wrapper = mountQuiz()

    const submit = wrapper.find('[data-testid="quiz-submit"]')
    expect((submit.element as HTMLButtonElement).disabled).toBe(true)

    await wrapper.findAll('input[type="radio"]')[0].setValue()
    await wrapper.findAll('input[type="radio"]')[2].setValue()
    expect((submit.element as HTMLButtonElement).disabled).toBe(false)
    wrapper.unmount()
  })
})

