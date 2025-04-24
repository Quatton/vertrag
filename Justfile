gen-axum:
  openapi-generator generate -i ./playground/contract/openapi.json -g rust-axum -o ./playground/rust-axum/gen

run-axum:
  cargo run --manifest-path ./playground/rust-axum/server/Cargo.toml