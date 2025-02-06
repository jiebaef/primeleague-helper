FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /primeleague-helper

# Prepare dependency and build caching
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /primeleague-helper/recipe.json recipe.json
# Build and cache dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin primeleague-helper

# Run the app
FROM debian:bookworm-slim AS runtime
RUN apt-get update -y && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /primeleague-helper
COPY --from=builder /primeleague-helper/target/release/primeleague-helper /usr/local/bin
ENTRYPOINT ["/usr/local/bin/primeleague-helper"]
EXPOSE 42069
