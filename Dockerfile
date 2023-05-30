FROM rustlang/rust:nightly-slim as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/airborn /usr/local/bin/airborn
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["airborn"]
