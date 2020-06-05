FROM rust:1-stretch as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:stretch-slim
COPY --from=builder /usr/src/app/target/release/actix-mongodb-example /bin/
CMD actix-mongodb-example
