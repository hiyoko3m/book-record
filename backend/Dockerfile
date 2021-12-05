FROM rust:1.57.0 AS builder
WORKDIR /usr/src/book-record
# packageのインストールをキャッシュさせる
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() { println!("Hello, world!"); }' > src/main.rs \
    && cargo build --release
COPY . ./
# timestampが前に戻るので、touchしないとcargo buildがし直されない
RUN touch src/main.rs \
    && cargo build --release

FROM debian:buster-slim AS runner
RUN apt-get update \
    && apt-get install libpq-dev -y
WORKDIR /app
COPY --from=builder /usr/src/book-record/target/release/book-record ./
CMD ["./book-record"]
