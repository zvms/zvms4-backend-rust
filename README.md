# ZVMS 4 Backend Implementation with Rust

The first version of backend of ZVMS 4 is a `shit mountain` of code.

Since I am learning `Rust`, I decided to rewrite the backend in `Rust`.

Technologies used:

- `axum`: Web framework
- `tokio`: Async runtime
- `mongodb`: Database

> You should have `Rust` installed in your system to run this project.

> You need to create `src/config.rs` with following code:

```rust
pub const MONGO_URL: &str = "<MONGO_URL>";
```
