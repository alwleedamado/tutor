# The Node Runtime & Event Loop

Node runs your JavaScript on a single thread with an event loop; concurrency comes from async I/O, not from threads. Master this and every Express performance question becomes answerable.

## The model

```text
one main thread:  runs your JS, one callback at a time
event loop:       picks pending callbacks/timers in phases
libuv threads:    offload file I/O, DNS, crypto (behind the scenes)
```

`await fetch(...)`, `await db.query(...)` — the loop is free while waiting. The loop is NOT free during synchronous work.

## What blocks the loop

```js
// ❌ every request on the server stalls behind this
const huge = JSON.parse(fs.readFileSync('10mb.json', 'utf8'))

// ❌ same problem, CPU flavor
crypto.createHash('sha256').update(hugeBuffer).digest()

// ✅ async I/O — loop serves others while waiting
await fs.promises.readFile('10mb.json', 'utf8')

// ✅ CPU-bound work off the loop entirely
new Worker('./hash-worker.js', { workerData: payload })
```

A 2-second sync hash in one handler delays **every** request for 2 seconds. Classic blockers: big `JSON.parse`, sync fs calls, heavy crypto, image processing, regex on huge strings.

## The production reflexes

- Any I/O in a handler → `await` the async version.
- Any CPU-heavy transform → worker thread or a dedicated service.
- Watch for loops whose *latency* degrades under load — that's the loop blocking, visible as high "event loop lag" in metrics.

## Best practices

- Keep handlers async end-to-end; a stray `readFileSync` is a latency bomb.
- `fastify`/`express` make no difference here — the loop is the runtime's, not the framework's.

## Common mistakes

- Adding servers/replicas to fix blocking CPU work (scaling the symptom).
- `atob`/JSON on giant payloads in a request path.
- Believing "async function" makes code non-blocking — only awaited *I/O* yields; sync CPU work inside async functions still stalls the loop.
