# Stage 1: builder
FROM rust:1.89 AS builder
WORKDIR /usr/src/hll

# Copy only Cargo.toml and Cargo.lock first
COPY Cargo.toml Cargo.lock ./

# Create empty src folder to satisfy cargo
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (cached unless Cargo.toml changes)
RUN cargo build --release
RUN rm -rf src

# Now copy actual source code
COPY . .

# Build the final binary
RUN cargo install --path .

# Stage 2: minimal runtime image
FROM debian:bookworm-slim
WORKDIR /usr/src/hll
# RUN apt-get update && apt-get install -y 
COPY ./spellfix1.so /usr/src/hll/spellfix1.so
COPY ./migrations /usr/src/hll/migrations
COPY --from=builder /usr/local/cargo/bin/hll /usr/local/bin/hll
CMD ["hll"]
