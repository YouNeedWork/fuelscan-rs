FROM rust:1.70-alpine

RUN apk update && \
    apk add zlib-dev bzip2-dev lz4-dev snappy-dev zstd-dev gflags-dev && \
    apk add build-base linux-headers git bash perl
RUN apk add clang \ 
    clang-libs \ 
    llvm \ 
    cmake \ 
    protobuf-c-dev \ 
    protobuf-dev \ 
    musl-dev \ 
    build-base \ 
    gcc \ 
    libc-dev \ 
    python3-dev 

WORKDIR /usr/src/fuelscan
COPY . .

RUN cargo install --path .

CMD ["fuelscan"]