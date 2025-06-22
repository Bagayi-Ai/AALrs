FROM rust:1.87.0-slim AS build

WORKDIR /app

# Copy workspace files
COPY . .

# Cache target directory to avoid rebuilding dependencies
# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies only
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

# # Build the project
# RUN cargo build

# # Set up development server
# EXPOSE 8080
# WORKDIR /app/standalone_connect4

# CMD ["dx", "serve", "--address", "0.0.0.0"]
