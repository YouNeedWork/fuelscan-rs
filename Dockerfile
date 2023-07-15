FROM rust:1.70.0

RUN apt-get update && apt-get install -y cmake clang

WORKDIR /usr/src/fuelscan
COPY . .

RUN cargo build --release


FROM debian:bullseye-slim AS runtime
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y libjemalloc-dev
ENV LD_PRELOAD /usr/lib/x86_64-linux-gnu/libjemalloc.so

COPY --from=builder /usr/src/fuelscan/target/release/fuelscan /usr/local/bin
RUN cp /usr/src/fuelscan/target/release/fuelscan /usr/local/bin/fuelscan
WORKDIR /usr/src/fuelscan
CMD ["fuelscan"]