# Migration Guide: Vuex → Pinia

Worked conversion of `vuex/progress.store.js` → `pinia/progress.store.ts`.

## Step 1 — State stays, gets typed
```diff
- state: () => ({ done: [], total: 10 })
+ state: (): ProgressState => ({ done: [], total: 10 })
```

## Step 2 — Getters convert 1:1 (drop string maps later)
```diff
  getters: {
    percent(state) { … },        // identical body
    isDone: (state) => (id) => … // identical
  }
```
Component change: `...mapGetters(['percent'])` → `const s = useProgressStore()`.

## Step 3 — Mutations dissolve INTO actions
```diff
- mutations: {
-   MARK(state, id) { if (!state.done.includes(id)) state.done.push(id) },
-   UNMARK(state, id) { state.done = state.done.filter(...) }
- },
  actions: {
-   mark({ commit }, id) { commit('MARK', id) },
+   mark(id: string): void {
+     if (!this.done.includes(id)) this.done.push(id)   // direct, typed
+   },
```
Rule of thumb: every `commit('X', payload)` line becomes the mutation body,
inlined into its action. Delete the whole mutations block afterwards.

## Step 4 — Components lose mapping helpers
```diff
- computed: { ...mapGetters(['percent', 'isDone']) }
- methods: { ...mapActions(['mark']) }
+ const progress = useProgressStore()
+ // template: {{ progress.percent }}  @click="progress.mark(id)"
```
TypeScript now checks every call site — impossible with `'cart/ADD'` strings.

## Step 5 — Registration & modules
`app.use(createPinia())` replaces `app.use(store)`; namespaced module paths
collapse into store ids. Multi-module apps become one file per domain store;
cross-store reads are plain function calls inside actions.

## Checklist
- [ ] zero occurrences of `commit(` / `mapGetters` / `mapActions`
- [ ] every action mutates `this.*` directly
- [ ] store ids replace namespace strings
- [ ] tests import real stores with fresh pinia per test
