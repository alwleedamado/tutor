# SOLID in Non-OOP Practice

SOLID was coined for OO classes, but the *ideas* transfer to Rust traits, Vue composables and Express middleware — if you translate them instead of reciting them.

## Single Responsibility (SRP)

One reason to change per unit. In Rust, a module that formats **and** stores **and** validates invoices has three reasons to change:

```rust
struct InvoiceRepo { /* storage */ }
struct InvoiceValidator;          // rules
struct InvoiceFormatter;          // presentation
```

In Vue: a component that fetches, transforms and renders data has three change reasons — split into composable (fetch), store/service (transform), component (render).

## Open/Closed (OCP)

Extend behavior by **adding**, not editing. Trait implementations are the Rust-native move:

```rust
trait Notifier { fn send(&self, msg: &str); }

struct EmailNotifier;
impl Notifier for EmailNotifier { fn send(&self, msg: &str) { /* ... */ } }

// adding SMS later = a new impl, zero edits to existing code
```

## Dependency Inversion (DIP)

High-level policy depends on an **abstraction**, low-level details plug in:

```rust
struct Checkout<N: Notifier> { notifier: N }
```

In Vue, a composable accepting an injected `api` (via provide/inject or a plain parameter) instead of importing a singleton directly is the same idea — and it makes testing trivial.

## Composition over inheritance

Neither Rust nor idiomatic Vue has implementation inheritance, and that is not a loss: compose small traits and composables. `useFetch` + `useAuth` beats an `AuthedFetchBase` class.

## Best practices

- Translate each principle to a *mechanism you actually have*: traits, generics, composables, middleware.
- Apply SRP at the **change-reason** level, not the file-size level.
- Reach for DIP at true boundaries (I/O, time, randomness) — not everywhere.

## Common mistakes

- **Ceremony**: five layers of traits for one implementation "because SOLID". Abstraction without a second implementation is a guess.
- **Anemic SRP**: splitting until every function is its own "responsibility" — now cohesion is destroyed.
- Confusing DIP with "always use a DI framework"; in Rust and Vue, plain parameters often *are* the inversion.
