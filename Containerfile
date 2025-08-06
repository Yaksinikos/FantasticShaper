
# Stage 1: Build
FROM rust:alpine as builder
RUN apk add --no-cache musl-dev deno
WORKDIR /app
COPY . .
RUN deno task tauri build --no-bundle

# Stage 2: Final image
FROM scratch
COPY --from=builder /app/scr-tauri/target/release/x86_64-unknown-linux-musl/release/fantasticshaper /app/
ENTRYPOINT ["/app/fantasticshaper"]