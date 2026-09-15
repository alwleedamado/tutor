// EDUCATIONAL ARTIFACT — Vuex 4 legacy equivalent. NEVER imported by this app.
// To execute: copy outside this repo into a scratch Vue project with `vuex@4`.
import { createStore } from 'vuex'

interface State {
  done: string[]
  total: number
}

export default createStore({
  state: (): State => ({
    done: [],
    total: 10
  }),

  getters: {
    percent(state): number {
      return state.total === 0 ? 0 : Math.round((state.done.length * 100) / state.total)
    },
    isDone: (state) => (lessonId: string) => state.done.includes(lessonId)
  },

  mutations: {
    MARK(state, lessonId: string) {
      if (!state.done.includes(lessonId)) state.done.push(lessonId)
    },
    UNMARK(state, lessonId: string) {
      state.done = state.done.filter((id) => id !== lessonId)
    }
  },

  actions: {
    mark({ commit }, lessonId: string) {
      commit('MARK', lessonId)
    },
    unmark({ commit }, lessonId: string) {
      commit('UNMARK', lessonId)
    }
  }
})

// Component usage (Options API era):
//   computed: { ...mapGetters(['percent']) },
//   methods: { ...mapActions(['mark', 'unmark']) }

/* Observations for the reader:
   1. Two extra layers (mutations) exist purely for devtool audit trails.
   2. String action/mutation names are untyped.
   3. mapGetters/mapActions obscure where data actually flows.        */
