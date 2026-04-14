FROM rust:1.82-bookworm

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN cargo build --release

EXPOSE 3000

CMD ["cargo", "run", "--release"]