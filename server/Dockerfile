# Set the base image to the official Rust image
FROM rust:latest

# Install protoc
RUN apt-get update && apt-get install -y protobuf-compiler

# Create a new directory for the application
WORKDIR /usr/src/app

# Copy the Cargo.toml file to the application directory
COPY Cargo.toml .

# Copy the requried folders to the application directory
COPY server ./server
COPY scheduler ./scheduler
COPY proto ./proto

# Set the environment variable to enable the flag
ENV EXPIREMENTAL_ALLOW_PROTO3_OPTIONAL=true
ENV LISTEN_PORT=10000

# Build the application using Cargo
RUN cargo build --release

# Expose the port used by the server
EXPOSE 10000

# Run the server when the container starts
CMD ["./target/release/server"]