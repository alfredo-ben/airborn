FROM rustlang/rust:nightly-slim as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

RUN apt update && \
    apt install -y libpq-dev && \
    cargo install diesel_cli --no-default-features --features postgres && \
    diesel setup

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/airborn /usr/local/bin/airborn
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["airborn"]
