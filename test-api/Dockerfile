FROM rust:latest as builder

ADD . ./

RUN cargo build --release

FROM gcr.io/distroless/cc:latest

COPY --from=builder ./target/release/test-api ./test-api

CMD ["./test-api"]