import { onBeforeUnmount, onMounted } from 'vue'

export type ShortcutMap = Record<string, (event: KeyboardEvent) => void>

function eventKey(event: KeyboardEvent): string {
  const parts: string[] = []
  if (event.ctrlKey || event.metaKey) parts.push('ctrl')
  if (event.altKey) parts.push('alt')
  if (event.shiftKey) parts.push('shift')
  parts.push(event.key.toLowerCase())
  return parts.join('+')
}

/** Attach window-level keyboard shortcuts; cleaned up on unmount. */
export function useShortcuts(map: ShortcutMap): void {
  const handler = (event: KeyboardEvent) => {
    const combo = eventKey(event)
    const action = map[combo]
    if (action) {
      event.preventDefault()
      action(event)
    }
  }
  onMounted(() => window.addEventListener('keydown', handler))
  onBeforeUnmount(() => window.removeEventListener('keydown', handler))
}
