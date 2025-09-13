FROM rust:1.70-alpine

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Expose the port (though UDP doesn't need explicit exposure)
EXPOSE 3000/udp

CMD ["./target/release/balloon-software"]
