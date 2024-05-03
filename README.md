# ZVMS 4 Backend Implementation with Rust

[![JWT](https://jwt.io/img/badge-compatible.svg)](https://jwt.io/)

![Test](https://github.com/zvms/zvms4-backend-rust/actions/workflows/test.yml/badge.svg) ![Build](https://github.com/zvms/zvms4-backend-rust/actions/workflows/release.yml/badge.svg)

The first version of backend of ZVMS 4 is a `shit mountain` of code.

Since I am learning `Rust`, I decided to rewrite the backend in `Rust`.

Technologies used:

- `axum`: Web framework
- `tokio`: Async runtime
- `mongodb`: Database

> Remember to export the Python DLIB path before running the project.

For example:

```bash
export DYLD_LIBRARY_PATH=/opt/anaconda3/envs/zvms/lib:$DYLD_LIBRARY_PATH
```
