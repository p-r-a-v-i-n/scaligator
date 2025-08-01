FROM rust:1.79-slim-bookworm AS builder

ARG APP_NAME=scaligator
WORKDIR /app

# This can help with some git-based dependencies in CI/CD environments.
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# --- Dependency Caching Layer ---
# This is the most important part for build performance.

# 1. Copy only the manifest files to start.
COPY Cargo.toml Cargo.lock ./

# 2. Create a dummy project and build it.
# This step compiles and caches *all* your dependencies without needing your actual source code.
# Docker will cache this layer heavily. It will only be invalidated if
# your Cargo.toml or Cargo.lock file changes.
RUN mkdir src && \
    echo "fn main() {println!(\"Building dependencies...\");}" > src/main.rs && \
    cargo build --release

# --- Application Build Layer ---
# The cache is primed. Now we build the actual application.

# 3. Remove the dummy source file.
RUN rm -f src/main.rs

# 4. Copy your actual application source code.
# Changes to your application code will only invalidate the Docker cache from this point onward.
COPY src ./src

# 5. Build your application.
# This will be very fast because all dependencies are already compiled and cached.
# Using `--locked` is a good practice to ensure reproducible builds.
RUN cargo build --release --locked


FROM debian:bookworm-slim AS final

# Create a dedicated, non-root user for running the application.
RUN useradd --create-home --shell /bin/bash appuser

# Copy the compiled application binary from the 'builder' stage.
COPY --from=builder /app/target/release/${APP_NAME} /home/appuser/${APP_NAME}

# Ensure the new user owns the application binary.
RUN chown appuser:appuser /home/appuser/${APP_NAME}

# Switch to the non-root user. From this point on, all commands run as 'appuser'.
USER appuser
WORKDIR /home/appuser

# Define the command to run when the container starts.
CMD ["./${APP_NAME}"]
