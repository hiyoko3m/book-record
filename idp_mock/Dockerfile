FROM rust:1.57.0 AS builder
WORKDIR /usr/src/idp_mock
# packageのインストールをキャッシュさせる
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() { println!("Hello, world!"); }' > src/main.rs \
    && cargo build --release
COPY . ./
# timestampが前に戻るので、touchしないとcargo buildがし直されない
RUN touch src/main.rs \
    && cargo build --release

FROM gcr.io/distroless/cc-debian10 AS runner
WORKDIR /app
COPY --from=builder /usr/src/idp_mock/target/release/idp_mock ./
CMD ["./idp_mock"]
