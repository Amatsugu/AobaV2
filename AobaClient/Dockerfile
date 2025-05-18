FROM rust:1 AS chef
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
WORKDIR /app/AobaClient
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app/AobaClient
COPY --from=planner /app/AobaClient/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY /AobaClient /app/AobaClient
COPY /AobaServer/Proto /app/AobaServer/Proto

# Install Protobuf
RUN apt update
RUN apt install -y protobuf-compiler libprotobuf-dev

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN dx bundle --platform web
ENTRYPOINT [ "sleep", "infinity" ]


# FROM chef AS runtime
# COPY --from=builder /app/AobaClient/target/dx/aoba-client/release/web/ /usr/local/app

# # set our port and make sure to listen for all connections
# ENV PORT=8616
# ENV IP=0.0.0.0

# # expose the port 8080
# EXPOSE 8616

# WORKDIR /usr/local/app
# ENTRYPOINT [ "/usr/local/app/server" ]

