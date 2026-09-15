/**
 * Offline Vue preview engine.
 *
 * Extracts template / <script setup> / style sections from a snippet and
 * builds a complete standalone HTML document that runs it with a locally
 * bundled Vue runtime (public/vendor/vue.global.prod.js — ZERO CDN).
 *
 * The document loads into a sandboxed iframe (allow-scripts, null origin):
 * preview code can never touch the app's DOM, storage, or IPC bridge.
 */

/**
 * Locally bundled Vue runtime served verbatim from /public — identical URL in
 * dev and production. Refresh the copy after upgrading vue:
 *   Copy-Item node_modules\vue\dist\vue.global.prod.js public\vendor\
 */
export const VUE_RUNTIME_PATH = '/vendor/vue.global.prod.js'


export interface ParsedSnippet {
  template: string
  script: string
  style: string
  hadTemplate: boolean
}

const RESERVED_BINDINGS = new Set(['defineProps', 'defineEmits', 'Vue', 'props', 'emit'])

/** Tolerant SFC-lite extraction (single root template block). */
export function parseSnippet(code: string): ParsedSnippet {
  const templateMatch = /<template[^>]*>([\s\S]*?)<\/template>/i.exec(code)
  const scriptMatch =
    /<script[^>]*\bsetup\b[^>]*>([\s\S]*?)<\/script>/i.exec(code) ??
    /<script(?![^>]*\bsrc\b)[^>]*>([\s\S]*?)<\/script>/i.exec(code)
  const styleMatch = /<style[^>]*>([\s\S]*?)<\/style>/i.exec(code)

  let template = templateMatch?.[1]?.trim() ?? ''
  const hadTemplate = Boolean(templateMatch)
  if (!hadTemplate) {
    // Bare-HTML snippets: strip script/style blocks and use what remains.
    template = code
      .replace(/<script[\s\S]*?<\/script>/gi, '')
      .replace(/<style[\s\S]*?<\/style>/gi, '')
      .trim()
  }

  return {
    template: template || '<p class="empty">Nothing to render yet.</p>',
    script: scriptMatch?.[1]?.trim() ?? '',
    style: styleMatch?.[1]?.trim() ?? '',
    hadTemplate
  }
}

/** Top-level bindings a `<script setup>` body exposes to its template. */
export function collectBindings(scriptBody: string): string[] {
  const names = new Set<string>()
  for (const m of scriptBody.matchAll(/\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*[=;]/g)) {
    names.add(m[1])
  }
  for (const m of scriptBody.matchAll(/\bfunction\s+([A-Za-z_$][\w$]*)/g)) {
    names.add(m[1])
  }
  return [...names].filter((n) => !RESERVED_BINDINGS.has(n))
}

/**
 * Light TypeScript relaxation for preview purposes: strips the annotation
 * forms our curriculum exercises use (defineProps/defineEmits generics, `as`
 * casts, simple param/return/var annotations). Anything fancier surfaces a
 * friendly overlay telling the learner to keep preview snippets plain-JS.
 */
export function relaxTypescript(src: string): string {
  let out = src
  out = out.replace(/define(Props|Emits)\s*<[^<>]*>/g, 'define$1')
  out = out.replace(/\s+as\s+[A-Za-z_$][\w$<>[\].|]*/g, '')
  // ): RetType {   |   ): RetType =>
  out = out.replace(/\)\s*:\s*[A-Za-z_$][\w$<>[\].|, ]*?(?=\s*(?:=>|\{))/g, ')')
  // (param: Type,   |   , param: Type)
  out = out.replace(/([(,]\s*[A-Za-z_$][\w$]*)\s*:\s*[A-Za-z_$][\w$<>[\].|]*?(?=\s*[),=])/g, '$1')
  // const x: SomeType =
  out = out.replace(/\b(const|let|var)\s+([A-Za-z_$][\w$]*)\s*:\s*[^=\n]+?=/g, '$1 $2 =')
  return out
}

function jsonForHtml(value: unknown): string {
  return JSON.stringify(value).replace(/<\/script/gi, '<\\/script')
}

/** `new Function` bodies cannot contain import statements — drop them. */
export function stripImportStatements(src: string): string {
  return src.replace(/^\s*import\b[^;\n]*(?:;|(?=\n|$))/gm, '')
}


export function buildPreviewDocument(
  code: string,
  options: { dark: boolean; vueSrc?: string }
): string {
  const vueSrc = options.vueSrc ?? VUE_RUNTIME_PATH
  const parsed = parseSnippet(code)

  let scriptBody = stripImportStatements(parsed.script)

  const rawParses = (() => {
    try {
      void new Function(scriptBody)
      return true
    } catch {
      return false
    }
  })()
  if (!rawParses) scriptBody = relaxTypescript(scriptBody)
  const bindings = jsonForHtml(rawParses ? collectBindings(parsed.script) : collectBindings(scriptBody))


  const bg = options.dark ? '#0b1220' : '#ffffff'
  const fg = options.dark ? '#e2e8f0' : '#1e293b'
  const muted = options.dark ? '#94a3b8' : '#64748b'
  const border = options.dark ? '#1e293b' : '#e2e8f0'

  return `<!doctype html>
<html><head><meta charset="utf-8">
<style>
  html,body{margin:0;padding:12px;background:${bg};color:${fg};
    font-family:system-ui,"Segoe UI",Roboto,sans-serif;font-size:14px;line-height:1.5}
  button{font:inherit;padding:4px 10px;border-radius:6px;border:1px solid ${border};
    background:transparent;color:inherit;cursor:pointer;margin-right:6px}
  button:hover{border-color:#14b8a6;color:#0d9488}
  input,select,output{font:inherit;padding:4px 8px;border-radius:6px;
    border:1px solid ${border};background:transparent;color:inherit}
  ${parsed.style}
  #__err{white-space:pre-wrap;font:12px/1.5 ui-monospace,Consolas,monospace;color:#fecaca;
    background:#450a0a;border:1px solid #7f1d1d;border-radius:8px;padding:10px;margin:0 0 10px}
  #__hint{color:${muted};font-size:12px;font-style:italic}
  #__events{position:fixed;left:0;right:0;bottom:0;padding:4px 12px;font:11px ui-monospace,monospace;
    color:${muted};background:${options.dark ? '#101a2eee' : '#f1f5f9ee'};border-top:1px solid ${border}}
</style></head>
<body>
<div id="app"></div>
<pre id="__err" hidden></pre>
<p id="__hint" hidden>Add a &lt;template&gt; section to visualize this snippet.</p>
<div id="__events" hidden></div>
<script src="${vueSrc}" onerror="document.getElementById('__err').hidden=false;document.getElementById('__err').textContent='Vue runtime failed to load from ${vueSrc}'"><\/script>
<script>
(function () {
  if (!window.Vue) {
    var errEl = document.getElementById('__err');
    errEl.hidden = false;
    errEl.textContent = 'Vue runtime did not load from ${vueSrc}';
    return;
  }

  var errEl = document.getElementById('__err');
  var hintEl = document.getElementById('__hint');
  var eventsEl = document.getElementById('__events');
  function fail(message) { errEl.hidden = false; errEl.textContent = String(message); }
  window.addEventListener('error', function (e) { fail(e.message); });

  var emitted = [];
  function renderEvents() {
    if (!emitted.length) return;
    eventsEl.hidden = false;
    eventsEl.textContent = 'emitted \\u2192 ' + emitted.map(function (entry) {
      return entry.event + (entry.payload === undefined ? '' : ' ' + JSON.stringify(entry.payload));
    }).join(', ');
  }

  var template = ${jsonForHtml(parsed.template)};
  var scriptBody = ${jsonForHtml(scriptBody)};
  var bindings = ${bindings};

  if (!${parsed.hadTemplate ? 'true' : 'false'}) hintEl.hidden = false;

  var shims = {
    defineProps: function () { return {}; },
    defineEmits: function () {
      return function (event, payload) {
        emitted.push({ event: event, payload: payload });
        renderEvents();
      };
    }
  };

  try {
    var factory = new Function('Vue', '__shims',
      '"use strict";' +
      'var ref=Vue.ref,reactive=Vue.reactive,computed=Vue.computed,watch=Vue.watch,' +
      'watchEffect=Vue.watchEffect,onMounted=Vue.onMounted,onUnmounted=Vue.onUnmounted,' +
      'nextTick=Vue.nextTick,provide=Vue.provide,inject=Vue.inject;' +
      'var defineProps=__shims.defineProps,defineEmits=__shims.defineEmits;' +
      scriptBody + ';return {' + bindings.join(',') + '};'
    );
    var setupResult = factory(window.Vue, shims) || {};
    var app = Vue.createApp({ template: template, setup: function () { return setupResult; } });
    app.config.errorHandler = function (err) { fail(err && (err.message || err)); };
    app.mount('#app');
  } catch (err) {
    fail((err && (err.message || err)) +
      '\\n\\nPreview tip: keep preview snippets plain JavaScript inside <template>/<script setup>.');
  }
})();
<\/script>
</body></html>`
}


