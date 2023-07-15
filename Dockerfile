FROM rust:1.70

WORKDIR /usr/src/fuelscan
COPY . .

RUN cargo install --path .

CMD ["fuelscan"]