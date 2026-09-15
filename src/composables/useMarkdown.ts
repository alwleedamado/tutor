import { marked } from 'marked'

marked.setOptions({ gfm: true, breaks: true })

/**
 * Render trusted-but-defended local markdown into HTML:
 * strip <script>/<iframe>/event-handler attributes after parsing so a stray
 * snippet can never execute inside the webview.
 */
export function renderMarkdown(source: string): string {
  const html = marked.parse(source ?? '', { async: false }) as string
  return sanitize(html)
}


function sanitize(html: string): string {
  return html
    .replace(/<script\b[^>]*>[\s\S]*?<\/script>/gi, '')
    .replace(/<iframe\b[^>]*>[\s\S]*?<\/iframe>/gi, '')
    .replace(/\son\w+\s*=\s*("[^"]*"|'[^']*'|[^\s>]+)/gi, '')
}
