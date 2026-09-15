# Health, Metrics, Resilience & Background Jobs

A service that "works" locally is not production-ready until it can be *operated*: probed, measured, limited, and stopped safely.

## Liveness vs readiness

| Endpoint | Answers | Failing means |
| --- | --- | --- |
| `/health` (liveness) | is the process alive? | restart me |
| `/ready` (readiness) | can I serve traffic now? | stop sending traffic |

Readiness checks the things that matter: DB pool responds, cache warm, migrations applied. During deploys readiness flips *before* shutdown, so load balancers drain you gracefully.

## Metrics that answer real questions

- **RED** per endpoint: Rate, Errors, Duration (p50/p95/p99).
- **Saturation**: pool waits, queue depth, memory.
- Emit durations as histograms — averages hide the pain your users feel at p99.

## Resilience patterns, honestly applied

- **Timeouts everywhere**: every outbound call gets one; a dependency hanging must not hang you.
- **Retries with backoff + jitter** — for *idempotent* operations only, capped (3 tries, exponential + jitter to avoid thundering herds).
- **Circuit breaker**: after N failures, fail fast for a cool-down instead of hammering a dying dependency.

```rust
// bounded retry with backoff — never an infinite loop
loop {
    match try_send() {
        Ok(v) => return Ok(v),
        Err(_) if attempt <= 3 => sleep(backoff(attempt)),
        Err(e) => return Err(e),
    }
}
```

## Background jobs

Jobs fail differently from requests: nobody is watching. So:

- **Bounded queues** — backpressure beats memory death.
- **Cancellation tokens** checked between steps — shutdown must stop jobs cleanly.
- **Idempotent handlers** — jobs retry; the work must tolerate it.
- **Dead-letter queue** for permanent failures, with an alert.

## Best practices

- Alert on symptoms (error rate, p99, queue age), not causes (CPU %).
- Test the failure paths: kill the DB and watch readiness flip, timeouts fire, breaker open.

## Common mistakes

- Infinite retries turning one outage into a self-inflicted DDoS.
- Jobs with no cancellation — deploys leak half-finished work.
- Unbounded channels: the queue that eats the process at 3 a.m.
