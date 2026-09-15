# Streams, Process Lifecycle & Graceful Shutdown

Two production survival skills: move large data through streams instead of memory, and shut down without dropping requests.

## Streams: constant memory for any size

```js
// ❌ loads the whole file (2GB?) into memory
res.send(await fs.promises.readFile(bigPath))

// ✅ chunks flow through with backpressure
const stream = fs.createReadStream(bigPath)
stream.pipe(res)
```

Backpressure: if the client is slow, the stream pauses reads — memory stays flat no matter the file size. The same applies to request bodies, CSV transforms (`stream.pipeline` with transform streams), and DB cursors.

## Configuration: 12-factor style

```text
code holds defaults  →  config file  →  environment wins
```

`process.env.PORT`, `DATABASE_URL`, `LOG_LEVEL` — read at boot, validated (types/ranges), missing-critical = refuse to start. Fail in 50ms at boot, not on the first request at 3 a.m.

## Graceful shutdown

```js
const server = app.listen(3000)

process.on('SIGTERM', () => {
  server.close(() => process.exit(0))     // stop accepting; finish in-flight
  setTimeout(() => process.exit(1), 30_000).unref()  // hard cap: don't hang forever
})
```

The sequence: SIGTERM arrives → stop accepting new connections → in-flight requests finish (bounded) → exit. With a readiness endpoint flipping to 503 first, load balancers drain you and users notice nothing.

Also handle `unhandledRejection`/`uncaughtException`: log fatally and exit non-zero — a half-broken process is worse than a restarted one.

## Best practices

- `stream.pipeline` (not raw `.pipe`) for proper error propagation and cleanup.
- Exit codes matter: 0 for clean shutdown, non-zero for failures — orchestrators act on them.

## Common mistakes

- Buffering "small" uploads that turn out not to be small.
- Catching SIGTERM with `process.exit(0)` immediately — in-flight uploads die on every deploy.
- Environment variables read deep in the codebase instead of validated once at boot.
