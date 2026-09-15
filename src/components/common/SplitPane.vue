<script setup lang="ts">
import { onMounted, ref } from 'vue'

/**
 * Two-pane resizable AND collapsible splitter (VSCode-style):
 * - drag the divider,
 * - arrow keys adjust (Shift = bigger step), Enter resets proportions,
 * - chevron buttons on the divider collapse either side; clicking the
 *   mirrored chevron expands back to its previous size,
 * - double-click on the divider resets proportions,
 * - sizes AND collapse state persist per storage key.
 */
const props = withDefaults(
  defineProps<{
    direction?: 'horizontal' | 'vertical'
    initial?: [number, number]
    storageKey?: string
    minFirst?: number
    minSecond?: number
    collapsible?: boolean
  }>(),
  {
    direction: 'horizontal',
    initial: () => [50, 50],
    minFirst: 12,
    minSecond: 12,
    collapsible: false
  }
)

type Collapsed = 'first' | 'second' | 'none'
const dir = props.direction
const sizes = ref<[number, number]>([...props.initial] as [number, number])
const collapsed = ref<Collapsed>('none')
let expandedSize = props.initial[0]

const container = ref<HTMLDivElement | null>(null)
const dragging = ref(false)

const storageKey = `rm-split:${props.storageKey ?? dir}`

onMounted(() => {
  try {
    const raw = localStorage.getItem(storageKey)
    if (!raw) return
    const parsed = JSON.parse(raw) as unknown
    if (Array.isArray(parsed)) {
      // Legacy shape: plain [number, number]
      if (
        parsed.length === 2 &&
        typeof parsed[0] === 'number' &&
        typeof parsed[1] === 'number'
      ) {
        sizes.value = [parsed[0], parsed[1]]
      }
      return
    }
    const obj = parsed as { sizes?: unknown; collapsed?: unknown }
    if (
      Array.isArray(obj.sizes) &&
      obj.sizes.length === 2 &&
      typeof obj.sizes[0] === 'number' &&
      typeof obj.sizes[1] === 'number'
    ) {
      sizes.value = [obj.sizes[0], obj.sizes[1]]
    }
    if (obj.collapsed === 'first' || obj.collapsed === 'second' || obj.collapsed === 'none') {
      collapsed.value = obj.collapsed
    }
  } catch {
    /* corrupted persistence is non-fatal */
  }
})

function persist(): void {
  try {
    localStorage.setItem(
      storageKey,
      JSON.stringify({ sizes: sizes.value, collapsed: collapsed.value })
    )
  } catch {
    /* storage unavailable — sizes stay session-local */
  }
}

function clamp(first: number): [number, number] {
  const clamped = Math.min(100 - props.minSecond, Math.max(props.minFirst, first))
  return [clamped, 100 - clamped]
}

function rememberExpanded(): void {
  if (sizes.value[0] >= props.minFirst) expandedSize = sizes.value[0]
}

function collapseTo(side: Exclude<Collapsed, 'none'>): void {
  if (collapsed.value === side) return
  rememberExpanded()
  collapsed.value = side
  sizes.value = side === 'first' ? [0, 100] : [100, 0]
  persist()
}

function expand(): void {
  if (collapsed.value === 'none') return
  collapsed.value = 'none'
  const restored = Math.min(100 - props.minSecond, Math.max(props.minFirst, expandedSize))
  sizes.value = [restored, 100 - restored]
  persist()
}

function onChevron(side: Exclude<Collapsed, 'none'>): void {
  if (collapsed.value === side) expand()
  else collapseTo(side)
}

function reset(): void {
  collapsed.value = 'none'
  sizes.value = [...props.initial] as [number, number]
  persist()
}

function onPointerDown(event: PointerEvent): void {
  if (collapsed.value !== 'none') return
  const rect = container.value?.getBoundingClientRect()
  if (!rect) return
  dragging.value = true
  event.preventDefault()
  ;(event.target as HTMLElement).setPointerCapture(event.pointerId)

  const startPos = dir === 'horizontal' ? event.clientX - rect.left : event.clientY - rect.top
  const startSize = sizes.value[0]

  const onMove = (move: PointerEvent): void => {
    const pos = dir === 'horizontal' ? move.clientX - rect.left : move.clientY - rect.top
    const total = Math.max(1, dir === 'horizontal' ? rect.width : rect.height)
    sizes.value = clamp(startSize + ((pos - startPos) / total) * 100)
  }
  const onUp = (): void => {
    dragging.value = false
    persist()
    window.removeEventListener('pointermove', onMove)
    window.removeEventListener('pointerup', onUp)
  }
  window.addEventListener('pointermove', onMove)
  window.addEventListener('pointerup', onUp)
}

function onKeydown(event: KeyboardEvent): void {
  const step = event.shiftKey ? 5 : 2
  if (event.key === 'ArrowLeft' || event.key === 'ArrowUp') {
    sizes.value = collapsed.value === 'none' ? clamp(sizes.value[0] - step) : sizes.value
    event.preventDefault()
    persist()
  } else if (event.key === 'ArrowRight' || event.key === 'ArrowDown') {
    sizes.value = collapsed.value === 'none' ? clamp(sizes.value[0] + step) : sizes.value
    event.preventDefault()
    persist()
  } else if (event.key === 'Enter') {
    reset()
  }
}
</script>

<style scoped>
.pane-btn {
  display: flex;
  height: 16px;
  width: 16px;
  align-items: center;
  justify-content: center;
  padding: 0;
  border-radius: 4px;
  border: 1px solid rgba(100, 116, 139, 0.55);
  background: rgba(226, 232, 240, 0.95);
  color: #334155;
  font-size: 10px;
  font-weight: 700;
  line-height: 1;
  cursor: pointer;
  box-shadow: 0 1px 2px rgb(0 0 0 / 0.15);
}
.pane-btn:hover {
  background: #14b8a6;
  border-color: #14b8a6;
  color: #ffffff;
}
:global(.dark) .pane-btn {
  background: rgba(51, 65, 85, 0.95);
  border-color: rgba(71, 85, 105, 0.8);
  color: #e2e8f0;
}
:global(.dark) .pane-btn:hover {
  background: #14b8a6;
  color: #ffffff;
}
</style>

<template>
  <div
    ref="container"
    class="flex min-h-0 min-w-0"
    :class="dir === 'horizontal' ? 'flex-row' : 'flex-col'"
  >
    <div
      class="min-h-0 min-w-0 overflow-hidden"
      :style="dir === 'horizontal' ? { width: `${sizes[0]}%` } : { height: `${sizes[0]}%` }"
    >
      <slot name="first" />
    </div>

    <div
      role="separator"
      :aria-orientation="dir === 'horizontal' ? 'vertical' : 'horizontal'"
      :aria-valuenow="Math.round(sizes[0])"
      aria-label="Resize panels; arrow keys adjust, Enter resets, chevrons collapse"
      tabindex="0"
      class="group relative shrink-0 transition-colors focus-visible:outline-none focus-visible:bg-teal-500/70 hover:bg-teal-500/60"
      :class="[
        dir === 'horizontal'
          ? 'w-1.5 cursor-col-resize'
          : 'h-1.5 cursor-row-resize',
        dragging ? 'bg-teal-500' : 'bg-slate-200 dark:bg-slate-700'
      ]"
      title="Drag to resize · double-click resets"
      @pointerdown="onPointerDown"
      @keydown="onKeydown"
      @dblclick="reset"
    >
      <div
        v-if="collapsible"
        class="absolute z-10 flex gap-0.5"
        :class="
          dir === 'horizontal'
            ? 'left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 flex-col items-center'
            : 'left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 flex-row items-center justify-center'
        "
      >
        <button
          v-if="collapsed !== 'second'"
          type="button"
          class="pane-btn"
          :title="
            collapsed === 'first'
              ? 'Expand panel'
              : `Collapse ${dir === 'horizontal' ? 'left' : 'top'} panel`
          "
          :aria-label="collapsed === 'first' ? 'Expand collapsed panel' : 'Collapse first panel'"
          @pointerdown.stop
          @dblclick.stop
          @click.stop="onChevron('first')"
        >
          {{ dir === 'horizontal'
              ? (collapsed === 'first' ? '»' : '‹')
              : (collapsed === 'first' ? '⌄' : '⌃') }}
        </button>
        <button
          v-if="collapsed !== 'first'"
          type="button"
          class="pane-btn"
          :title="
            collapsed === 'second'
              ? 'Expand panel'
              : `Collapse ${dir === 'horizontal' ? 'right' : 'bottom'} panel`
          "
          :aria-label="collapsed === 'second' ? 'Expand collapsed panel' : 'Collapse second panel'"
          @pointerdown.stop
          @dblclick.stop
          @click.stop="onChevron('second')"
        >
          {{ dir === 'horizontal'
              ? (collapsed === 'second' ? '«' : '›')
              : (collapsed === 'second' ? '⌃' : '⌄') }}
        </button>
      </div>
    </div>

    <div class="min-h-0 min-w-0 flex-1 overflow-hidden">
      <slot name="second" />
    </div>
  </div>
</template>


