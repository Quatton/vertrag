gen-axum:
  openapi-generator generate \
    --skip-validate-spec \
    -i ./playground/contract/openapi.json \
    -o ./playground/rust-axum/gen \
    -g rust-axum 

run-axum:
  cargo run --manifest-path ./playground/rust-axum/server/Cargo.toml

gen-fastapi:
    openapi-generator generate \
        --skip-validate-spec \
        -i ./playground/contract/openapi.json \
        -o ./playground/python-fastapi/gen \
        -g python-fastapi

# Make sure you already have a virtualenv created
run-fastapi:
  fastapi dev playground/python-fastapi/src/main.py