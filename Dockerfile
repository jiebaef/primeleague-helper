FROM rust:1.75 AS builder

RUN apt-get update -y && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the application source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final build
FROM debian:bookworm-slim

RUN apt-get update -y && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Set up a non-root user
RUN useradd -m appuser
WORKDIR /home/appuser

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/primeleague-helper ./primeleague-helper

# Ensure the binary is executable
RUN chmod +x primeleague-helper

# Expose the required port
EXPOSE 42069

# Run the application as the non-root user
USER appuser
CMD ["./primeleague-helper"]
