/**
 * Generates deterministic app icons without external tools:
 *  - src-tauri/icons/icon.ico (32x32, multi-use)
 *  - src-tauri/icons/icon.png (1024x1024 source for `npx tauri icon`,
 *    which produces the full platform set: pngs, .icns, .ico)
 */
import { mkdirSync, writeFileSync } from 'node:fs'
import { deflateSync } from 'node:zlib'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..')

// ---------- shared design (dark slate base, teal band, amber dot) ----------
function pixelAt(x, y, size) {
  const onBand = x + y >= (size / 32) * 20 && x + y <= (size / 32) * 30
  const dotStart = (size / 32) * 24
  const dotEnd = (size / 32) * 28
  const dotTop = (size / 32) * 4
  const dotBottom = (size / 32) * 8
  const isDot = x >= dotStart && x < dotEnd && y >= dotTop && y < dotBottom
  if (isDot) return [251, 191, 36] // amber-400
  if (onBand) return [45, 212, 191] // teal-400
  return [15, 23, 42] // slate-900 #0f172a
}

// ---------- PNG encoder (RGBA, no filtering) ----------
const CRC_TABLE = (() => {
  const table = new Uint32Array(256)
  for (let n = 0; n < 256; n++) {
    let c = n
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1
    table[n] = c >>> 0
  }
  return table
})()

function crc32(buf) {
  let crc = 0xffffffff
  for (const byte of buf) crc = CRC_TABLE[(crc ^ byte) & 0xff] ^ (crc >>> 8)
  return (crc ^ 0xffffffff) >>> 0
}

function pngChunk(type, data) {
  const out = Buffer.alloc(8 + data.length + 4)
  out.writeUInt32BE(data.length, 0)
  out.write(type, 4, 'ascii')
  data.copy(out, 8)
  out.writeUInt32BE(crc32(Buffer.concat([Buffer.from(type, 'ascii'), data])), 8 + data.length)
  return out
}

function encodePng(size, rgba) {
  const stride = size * 4
  const raw = Buffer.alloc((stride + 1) * size)
  for (let y = 0; y < size; y++) {
    raw[y * (stride + 1)] = 0 // filter: none
    rgba.copy(raw, y * (stride + 1) + 1, y * stride, (y + 1) * stride)
  }
  const ihdr = Buffer.alloc(13)
  ihdr.writeUInt32BE(size, 0)
  ihdr.writeUInt32BE(size, 4)
  ihdr[8] = 8 // bit depth
  ihdr[9] = 6 // color type: RGBA
  return Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    pngChunk('IHDR', ihdr),
    pngChunk('IDAT', deflateSync(raw, { level: 9 })),
    pngChunk('IEND', Buffer.alloc(0))
  ])
}

// ---------- 1024x1024 PNG source ----------
const pngSize = 1024
const pngRgba = Buffer.alloc(pngSize * pngSize * 4)
for (let y = 0; y < pngSize; y++) {
  for (let x = 0; x < pngSize; x++) {
    const [r, g, b] = pixelAt(x, y, pngSize)
    const off = (y * pngSize + x) * 4
    pngRgba[off] = r
    pngRgba[off + 1] = g
    pngRgba[off + 2] = b
    pngRgba[off + 3] = 255
  }
}

const pngOut = resolve(root, 'src-tauri/icons/icon.png')
mkdirSync(dirname(pngOut), { recursive: true })
writeFileSync(pngOut, encodePng(pngSize, pngRgba))
console.log(`Wrote ${pngOut} (${pngSize}x${pngSize} PNG)`)

// ---------- 32x32 ICO ----------
const size = 32
const xorSize = size * size * 4
const andRowBytes = Math.ceil(size / 32) * 4 // 4 bytes per row for 32px
const andSize = andRowBytes * size
const imageSize = 40 + xorSize + andSize

const header = Buffer.alloc(6)
header.writeUInt16LE(0, 0) // reserved
header.writeUInt16LE(1, 2) // type: icon
header.writeUInt16LE(1, 4) // count

const entry = Buffer.alloc(16)
entry.writeUInt8(size, 0)
entry.writeUInt8(size, 1)
entry.writeUInt8(0, 2) // palette
entry.writeUInt8(0, 3) // reserved
entry.writeUInt16LE(1, 4) // planes
entry.writeUInt16LE(32, 6) // bpp
entry.writeUInt32LE(imageSize, 8)
entry.writeUInt32LE(22, 12) // data offset

const bmp = Buffer.alloc(imageSize)
bmp.writeUInt32LE(40, 0) // biSize
bmp.writeInt32LE(size, 4)
bmp.writeInt32LE(size * 2, 8) // double height (XOR + AND)
bmp.writeUInt16LE(1, 12) // planes
bmp.writeUInt16LE(32, 14) // bpp
bmp.writeUInt32LE(xorSize + andSize, 20)

// Paint: dark slate base, teal diagonal band, amber corner dot.
for (let y = 0; y < size; y++) {
  for (let x = 0; x < size; x++) {
    const row = size - 1 - y // bottom-up
    const off = (row * size + x) * 4 + 40
    const [r, g, b] = pixelAt(x, y, size)
    bmp.writeUInt8(b, off)
    bmp.writeUInt8(g, off + 1)
    bmp.writeUInt8(r, off + 2)
    bmp.writeUInt8(255, off + 3)
  }
}

const out = resolve(root, 'src-tauri/icons/icon.ico')
mkdirSync(dirname(out), { recursive: true })
writeFileSync(out, Buffer.concat([header, entry, bmp]))
console.log(`Wrote ${out} (${6 + 16 + imageSize} bytes)`)
