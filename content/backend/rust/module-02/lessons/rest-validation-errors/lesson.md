# REST, JSON, Validation & Structured Errors

The API layer's contract: untrusted JSON in, validated domain values down, structured errors out.

## Handler → service → repository

```rust
// transport
async fn create_order(State(st): State<AppState>, Json(dto): Json<CreateOrderDto>)
    -> Result<Json<Order>, AppError>
{
    let input = dto.into_domain()?;              // validate at the edge
    let order = st.orders.create(input).await?;  // use-case
    Ok(Json(order.to_dto()))                     // map out
}
```

## Validate once, at the boundary

`into_domain()` is the single checkpoint: required fields, lengths, formats. Everything below the boundary receives trusted, typed values — no re-checking five layers deep.

## Structured errors beat `Result<T, String>`

```rust
#[derive(Debug)]
pub enum AppError {
    NotFound,
    Validation(String),
    Conflict,
}

impl AppError {
    pub fn status(&self) -> u16 {
        match self {
            AppError::NotFound => 404,
            AppError::Validation(_) => 422,
            AppError::Conflict => 409,
        }
    }
}
```

One `From<AppError> → Response` conversion centralizes status mapping and body shape (`{ "error": { "code", "message" } }`). Callers match on kinds in tests; the frontend renders on codes.

## Best practices

- One error enum per service, converted at the transport edge.
- 422 for syntactically-valid-but-invalid; 409 for state conflicts; never 200-with-error-body.
- Error messages for humans AND machines: stable `code` + readable `message`.

## Common mistakes

- `unwrap()` on untrusted JSON — a client can crash your service with one malformed field.
- Mapping every failure to 500, or worse, 200.
- String-matching error texts instead of matching error kinds.
