FROM rust:1.67 as builder

# install protobuf
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

COPY Cargo.toml build.rs /usr/src/app/
COPY src /usr/src/app/src/
COPY proto /usr/src/app/proto/
WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release --bin calculate-server

FROM gcr.io/distroless/static-debian11 as runner

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/calculate-server /

# EXPOSE 50051

CMD ["/calculate-server"]

