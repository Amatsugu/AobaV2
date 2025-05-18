# Client Side build - prep deps
FROM rust:1 AS chef
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
WORKDIR /app/AobaClient
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS client-builder
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

# Server Build
# This stage is used when running from VS in fast mode (Default for Debug configuration)
FROM mcr.microsoft.com/dotnet/aspnet:9.0 AS base
USER $APP_UID
WORKDIR /app
EXPOSE 8080
EXPOSE 8081

# This stage is used to build the service project
FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src
COPY ["AobaServer/AobaServer.csproj", "AobaServer/"]
RUN dotnet restore "./AobaServer/AobaServer.csproj"
COPY . .
# Copy Built bundle from client builder
COPY --from=client-builder /app/AobaClient/target/dx/aoba-client/release/web/public /src/AobaServer/wwwroot
WORKDIR "/src/AobaServer"
RUN dotnet build "./AobaServer.csproj" -c $BUILD_CONFIGURATION -o /app/build

# This stage is used to publish the service project to be copied to the final stage
FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "./AobaServer.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

# This stage is used in production or when running from VS in regular mode (Default when not using the Debug configuration)
FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "AobaServer.dll"]
