import { describe, expect, it } from 'vitest'
import { renderMarkdown } from '../useMarkdown'

describe('renderMarkdown', () => {
  it('renders headings and code blocks', () => {
    const html = renderMarkdown('# Title\n\n```rust\nfn main() {}\n```')
    expect(html).toContain('<h1>Title</h1>')
    expect(html).toContain('<code')

  })

  it('strips script tags even when embedded in trusted content', () => {
    const html = renderMarkdown('hello <script>alert(1)</script> world')
    expect(html).not.toContain('<script')
    expect(html).not.toContain('alert')
  })

  it('removes inline event handlers', () => {
    const html = renderMarkdown('<img src="x.png" onerror="pwn()">')
    expect(html).not.toContain('onerror')
    expect(html).toContain('src="x.png"')
  })

  it('drops iframes entirely', () => {
    const html = renderMarkdown('<iframe src="https://evil.example"></iframe>')
    expect(html).not.toContain('<iframe')
  })
})
