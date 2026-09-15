import { describe, expect, it } from 'vitest'

import {
  buildPreviewDocument,
  collectBindings,
  parseSnippet,
  relaxTypescript
} from '../preview'

const SFC = `<template>
  <button @click="count++">{{ count }}</button>
</template>

<script setup>
import { ref } from 'vue'
const count = ref(0)
function bump() { count.value++ }
</script>

<style>
button { color: teal }
</style>`

describe('parseSnippet', () => {
  it('extracts template, setup body and style', () => {
    const parsed = parseSnippet(SFC)
    expect(parsed.hadTemplate).toBe(true)
    expect(parsed.template).toContain('@click="count++"')
    expect(parsed.script).toContain('const count = ref(0)')
    expect(parsed.style).toContain('color: teal')
  })

  it('falls back to bare HTML when no template block exists', () => {
    const parsed = parseSnippet('<p>hello</p><script>var x = 1</script>')
    expect(parsed.hadTemplate).toBe(false)
    expect(parsed.template).toBe('<p>hello</p>')
  })
})


describe('collectBindings', () => {
  it('returns top-level bindings and skips reserved shims', () => {
    const names = collectBindings('const count = ref(0)\nfunction bump() {}\nconst defineProps = x')
    expect(names).toContain('count')
    expect(names).toContain('bump')
    expect(names).not.toContain('defineProps')
  })
})

describe('relaxTypescript', () => {
  it('strips the annotation forms used by curriculum exercises', () => {
    const relaxed = relaxTypescript(`const props = defineProps<{ modelValue: number }>()
const emit = defineEmits<{ (e: 'update:modelValue', value: number): void }>()
function toggle(initial = false): boolean { return initial as boolean }
`)
    expect(relaxed).toContain('defineProps(')
    expect(relaxed).toContain('defineEmits(')
    expect(relaxed).not.toMatch(/define(Props|Emits)\s*</)
    expect(relaxed).not.toMatch(/\):\s*[A-Za-z_$]/)
    expect(relaxed).not.toContain('as boolean')
    expect(relaxed).toContain('function toggle(initial = false)')
  })
})


describe('buildPreviewDocument', () => {
  const doc = buildPreviewDocument(SFC, { dark: true })

  it('embeds the local Vue asset from public/vendor (no CDN)', () => {
    expect(doc).toContain('<script src="/vendor/vue.global.prod.js"')
    expect(doc).not.toMatch(/https?:\/\//)
    // Load failures must surface instead of rendering a silent white frame.
    expect(doc).toContain('Vue runtime did not load')
  })

  it('strips import statements and exposes bindings via the setup factory', () => {
    expect(doc).not.toContain('import {')
    expect(doc).toContain('var ref=Vue.ref')
    expect(doc).toContain('"count","bump"')
  })

  it('injects template/script safely (closing tags escaped)', () => {
    expect(doc).toContain('@click=')
    expect(doc).toContain('const count = ref(0)')
  })

  it('honors theme colors', () => {
    const light = buildPreviewDocument('<template><p/></template>', { dark: false })
    expect(light).toContain('#ffffff')
  })
})

