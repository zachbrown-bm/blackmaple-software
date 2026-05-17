# syntax=docker/dockerfile:1
# 1. Build the Rust / WASM application
FROM rust:latest AS builder
WORKDIR /app

# Install wasm32 target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk
RUN wget -qO- https://github.com | tar -xzf- -C /usr/local/bin

# Copy the rest of your app and build
COPY . .
RUN trunk build --release

# 2. Serve the static files with Nginx
FROM nginx:alpine

# Copy built files to Nginx web directory
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy a custom Nginx configuration if needed (optional)
# COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
