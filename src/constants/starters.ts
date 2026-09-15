/** Default editor buffers shared by the lesson workspace and the playground. */

export const DEFAULT_STARTERS: Record<string, string> = {
  rust: 'fn main() {\n    println!("Hello, Rust!");\n}\n',
  // NOTE: "</script" is escaped (<\/script>) — the SFC tokenizer closes
  // blocks at the first literal occurrence, even inside JS strings.
  vue: '<template>\n  <button @click="count++">Clicked {{ count }}×</button>\n</template>\n\n<script setup>\nimport { ref } from "vue"\nconst count = ref(0)\n<\/script>\n',
  tauri: 'fn main() {\n    let app = "tauri";\n    println!("Hello from {app} land!");\n}\n',
  javascript:
    '<template>\n  <pre>{{ output }}</pre>\n</template>\n\n<script setup>\nimport { ref } from "vue"\nconst output = ref("Edit me and press Run")\n<\/script>\n'
}

export function starterFor(track: string): string {
  if (DEFAULT_STARTERS[track]) return DEFAULT_STARTERS[track]
  // Tracks without a dedicated starter get a language-appropriate buffer.
  return trackUsesRustSandbox(track) ? DEFAULT_STARTERS.rust : DEFAULT_STARTERS.javascript
}

/**
 * Tracks whose editor docs compile through the local rustc sandbox. Every
 * OTHER track runs in the sandboxed webview preview — never rustc. This set
 * mirrors the content registry's compiled-language tracks (see
 * docs/curriculum-architecture.md); new Rust-backed tracks must be added here.
 */
const RUST_SANDBOX_TRACKS = new Set(['rust', 'tauri', 'backend-rust'])

export function trackUsesRustSandbox(track: string): boolean {
  return RUST_SANDBOX_TRACKS.has(track)
}

/** Convenience predicate for the webview-preview runner. */
export function trackUsesWebPreview(track: string): boolean {
  return !trackUsesRustSandbox(track)
}

export function editorLanguageFor(track: string): string {
  if (trackUsesRustSandbox(track)) return 'rust'
  if (track === 'vue' || track === 'web-accessibility') return 'html'
  return 'javascript'
}
