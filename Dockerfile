# Use an official Rust image as the build stage
FROM rust:1.73 as builder

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo manifest and source files to the container
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .shuttle ./

# Build the project in release mode
RUN cargo build --release

# Use a smaller base image for the final container
FROM debian:buster-slim

# Install required system dependencies
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/starlit-ecommerce-api /usr/local/bin/app

# Expose the application's port
EXPOSE 8080

# Command to run the application
CMD ["app"]
