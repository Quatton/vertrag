gen-spec:
  bun run gen:openapi

gen-axum: gen-spec
  openapi-generator generate \
    --skip-validate-spec \
    -i ./playground/contract/openapi.json \
    -o ./playground/rust-axum/gen \
    -g rust-axum 

run-axum:
  cargo run --manifest-path ./playground/rust-axum/server/Cargo.toml

gen-fastapi: gen-spec
    openapi-generator generate \
        --skip-validate-spec \
        -i ./playground/contract/openapi.json \
        -o ./playground/python-fastapi \
        --additional-properties packageName=openapi \
        -g python-fastapi

# Make sure you already have a virtualenv created
run-fastapi:
  fastapi dev playground/python-fastapi/src/main.py