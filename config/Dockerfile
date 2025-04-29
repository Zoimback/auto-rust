FROM alpine:latest

# Instalar dependencias necesarias
RUN apk add --no-cache \
    rustup \
    cargo \
    clang \
    llvm \
    libgcc \
    musl-dev \
    make

# Instalar Rust
RUN rustup-init -y && \
    source $HOME/.cargo/env && \
    rustup default stable && \
    rustup component add clippy rustfmt

# Establecer el directorio de trabajo
WORKDIR /usr/src/app

# Copiar el código fuente al contenedor
COPY . .

# Compilar el código
RUN cargo build --release

# Ejecutar análisis con Clippy
RUN cargo clippy -- -D warnings

# Formatear el código con Rustfmt
RUN cargo fmt --all

# Definir el comando de ejecución
CMD ["./target/release/rust"]
