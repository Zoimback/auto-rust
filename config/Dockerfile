
# Usa la imagen oficial de Rust como base
FROM rust:latest

# Establece el directorio de trabajo dentro del contenedor
WORKDIR /usr/src/myapp

# Copia el resto de los archivos del proyecto al contenedor desde la ruta rust
COPY ./rust ./

# Compila el script de Rust
RUN cargo build --release

# Ejecuta el script compilado
CMD ["./target/release/rust"]


