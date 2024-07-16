# This stage is only used for building the Rust binary. Afterwards the binary is copied
# into the actual image below
FROM clux/muslrust:1.81.0-nightly-2024-07-16 as builder
WORKDIR /volume

COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build only the dependencies and cache them
RUN set -eux \
  &&  mkdir ./src \
  &&  echo "fn main() {}" > src/main.rs

RUN cargo build --release
RUN rm -rf ./src

COPY src ./src

# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs

RUN cargo build --release

# The actual image
FROM alpine
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/sawit-log .

ENTRYPOINT [ "/sawit-log" ]
