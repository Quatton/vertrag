gen-axum:
  openapi-generator generate \
    --skip-validate-spec \
    -i ./playground/contract/openapi.json \
    -o ./playground/rust-axum/gen \
    -g rust-axum 

run-axum:
  cargo run --manifest-path ./playground/rust-axum/server/Cargo.toml