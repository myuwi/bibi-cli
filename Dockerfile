FROM rust:1.86-alpine AS build

WORKDIR /app

RUN apk add --no-cache \
    musl-dev \
    pkgconf \
    openssl-dev \
    openssl-libs-static

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src &&  \
    echo 'fn main(){ eprintln!("You should not see this!") }' > src/main.rs && \
    cargo build --release && \
    rm -r src

COPY src src

RUN touch src/main.rs

RUN cargo build --release

FROM alpine:latest AS runtime

RUN apk add --no-cache ca-certificates

COPY --from=build /app/target/release/bibi /app/bibi

ENTRYPOINT ["/app/bibi"]
