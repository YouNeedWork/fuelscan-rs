FROM rust:1.70.0 as builder

RUN apt-get update && apt-get install -y cmake clang

WORKDIR /usr/src/fuelscan
COPY . .

RUN cargo build --release


FROM debian:bullseye-slim AS runtime
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates libpq-dev
RUN update-ca-certificates

COPY --from=builder /usr/src/fuelscan/target/release/fuelscan-core /usr/local/bin

WORKDIR /usr/src/fuelscan
CMD ["fuelscan-core"]