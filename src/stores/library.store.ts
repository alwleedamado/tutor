import { defineStore } from 'pinia'

import { ipc } from '@/services/ipc'
import type { BookmarkDto, NoteDto, NoteInput } from '@/types/domain'

interface LibraryState {
  notes: NoteDto[]
  bookmarks: BookmarkDto[]
  notesLoaded: boolean
  bookmarksLoaded: boolean
}

/** Owns student-authored artifacts: notes and bookmarks. */
export const useLibraryStore = defineStore('library', {
  state: (): LibraryState => ({
    notes: [],
    bookmarks: [],
    notesLoaded: false,
    bookmarksLoaded: false
  }),

  getters: {
    isBookmarked(state): (itemType: string, itemId: string) => boolean {
      return (itemType, itemId) =>
        state.bookmarks.some((b) => b.itemType === itemType && b.itemId === itemId)
    },
    notesForScope(state): (scopeType: string, scopeId: string) => NoteDto[] {
      return (scopeType, scopeId) =>
        state.notes.filter((n) => n.scopeType === scopeType && n.scopeId === scopeId)
    }
  },

  actions: {
    async ensureBookmarks() {
      if (this.bookmarksLoaded) return
      this.bookmarks = await ipc.listBookmarks()
      this.bookmarksLoaded = true
    },

    async ensureNotes(scopeType?: string, scopeId?: string) {
      // Full list powers /notes; scoped fetches refresh that slice.
      this.notes = await ipc.listNotes(scopeType, scopeId)
      this.notesLoaded = true
    },

    async saveNote(input: NoteInput): Promise<NoteDto> {
      const saved = await ipc.saveNote(input)
      const index = this.notes.findIndex((n) => n.id === saved.id)
      if (index >= 0) this.notes.splice(index, 1, saved)
      else this.notes.unshift(saved)
      return saved
    },

    async deleteNote(id: number) {
      if (!(await ipc.deleteNote(id))) return
      this.notes = this.notes.filter((n) => n.id !== id)
    },

    async toggleBookmark(itemType: string, itemId: string) {
      const bookmarked = await ipc.toggleBookmark(itemType, itemId)
      if (bookmarked) {
        this.bookmarks.unshift({ itemType, itemId, createdAt: Math.floor(Date.now() / 1000) })
      } else {
        this.bookmarks = this.bookmarks.filter(
          (b) => !(b.itemType === itemType && b.itemId === itemId)
        )
      }
      return bookmarked
    }
  }
})
