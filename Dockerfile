# ====================
# Stage 1
# Build the API
# ====================
FROM rustlang/rust:nightly AS api_builder
WORKDIR /app
COPY . /app/
RUN cargo build --bin website --release

# ====================
# Stage Final
# Bundle API and Client into a single container
# ====================
FROM ubuntu:latest AS api_server
WORKDIR /app
COPY --from=api_builder /app/target/release/website .
# update for future dep install
RUN apt update
# Install libssl as dependency
RUN apt install libssl-dev -y
ENTRYPOINT ["./website"]
EXPOSE 8000/tcp
# CMD ["./website"]

# ====================
# Stage 2
# ====================

# FROM node:alpine3.11 AS node_builder

# RUN apk update
# RUN apk add git
# RUN git clone https://github.com/mezeipetister/gnstore_client

# WORKDIR /gnstore_client

# RUN npm install -g @angular/cli
# RUN npm install
# RUN ng build --prod