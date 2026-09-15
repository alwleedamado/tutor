/**
 * One-shot repair: escapes raw control characters (LF/CR/TAB) that illegally
 * appear inside JSON string literals, then validates every content JSON.
 */
import fs from 'node:fs'
import path from 'node:path'

function walk(dir, out = []) {
  for (const entry of fs.readdirSync(dir)) {
    const full = path.join(dir, entry)
    if (fs.statSync(full).isDirectory()) walk(full, out)
    else if (entry.endsWith('.json')) out.push(full)
  }
  return out
}

/** Escape raw LF/CR/TAB occurring inside string literals; leave the rest. */
function escapeRawControls(src) {
  let out = ''
  let inString = false
  for (let i = 0; i < src.length; i++) {
    const ch = src[i]
    if (!inString) {
      if (ch === '"') inString = true
      out += ch
      continue
    }
    if (ch === '\\') {
      const next = src[i + 1] ?? ''
      // Valid JSON escapes pass through untouched.
      if ('"\\/bfnrtu'.includes(next)) {
        out += ch + next
        i++
        continue
      }
      // Authored-intent recovery: a stray "\p" was meant as a newline.
      if (next === 'p') {
        out += '\\n'
        i++
        continue
      }
      // Anything else: drop the stray backslash, keep the character.
      out += next
      i++
      continue
    }

    if (ch === '"') {
      inString = false
      out += ch
      continue
    }
    if (ch === '\n') {
      out += '\\n'
    } else if (ch === '\r') {
      out += '\\r'
    } else if (ch === '\t') {
      out += '\\t'
    } else {
      out += ch
    }
  }
  return out
}

let repaired = 0
for (const file of walk('content')) {
  const raw = fs.readFileSync(file, 'utf8')
  try {
    JSON.parse(raw)
  } catch {
    const fixed = escapeRawControls(raw)
    JSON.parse(fixed) // throws with context if still broken
    fs.writeFileSync(file, fixed)
    repaired++
    console.log('repaired:', file)
  }
}
console.log(`done — ${repaired} file(s) repaired`)
