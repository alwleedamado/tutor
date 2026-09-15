import { defineStore } from 'pinia'

import { ipc } from '@/services/ipc'

export interface EditorDocument {
  scopeId: string
  language: string
  code: string
  cursorOffset: number
  dirty: boolean
}

interface EditorState {
  documents: Record<string, EditorDocument>
  activeScopeId: string | null
}

/**
 * Owns open editor documents keyed by scope
 * (`lesson:<track>:<module>:<lesson>` or `exercise:<key>:<exerciseId>`).
 * Snapshots persist through the editor-state service.
 */
export const useEditorStore = defineStore('editor', {
  state: (): EditorState => ({
    documents: {},
    activeScopeId: null
  }),

  getters: {
    activeDoc(state): EditorDocument | null {
      return state.activeScopeId ? state.documents[state.activeScopeId] ?? null : null
    }
  },

  actions: {
    /**
     * Open (or focus) a document. Initial content resolution order:
     * already-open buffer → persisted snapshot → provided fallback.
     */
    async open(scopeId: string, language: string, fallbackCode: string): Promise<EditorDocument> {
      this.activeScopeId = scopeId
      const existing = this.documents[scopeId]
      if (existing) return existing

      const snapshot = await ipc.getEditorState(scopeId).catch(() => null)
      const doc: EditorDocument = {
        scopeId,
        language,
        code: snapshot?.code ?? fallbackCode,
        cursorOffset: snapshot?.cursorOffset ?? 0,
        dirty: false
      }
      this.documents[scopeId] = doc
      return doc
    },

    setCode(scopeId: string, code: string) {
      const doc = this.documents[scopeId]
      if (!doc) return
      doc.code = code
      doc.dirty = true
    },

    setCursor(scopeId: string, cursorOffset: number) {
      const doc = this.documents[scopeId]
      if (doc && doc.cursorOffset !== cursorOffset) doc.cursorOffset = cursorOffset
    },

    async persist(scopeId?: string) {
      const id = scopeId ?? this.activeScopeId
      if (!id) return
      const doc = this.documents[id]
      if (!doc) return
      await ipc.saveEditorState({
        scopeId: doc.scopeId,
        language: doc.language,
        code: doc.code,
        cursorOffset: doc.cursorOffset
      })
      doc.dirty = false
    },

    close(scopeId: string) {
      delete this.documents[scopeId]
      if (this.activeScopeId === scopeId) this.activeScopeId = null
    }
  }
})
