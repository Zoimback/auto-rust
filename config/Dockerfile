# Usa la imagen oficial de Rust como base
FROM rust:latest

# Establece el directorio de trabajo dentro del contenedor
WORKDIR /usr/src/myapp

# Copia el resto de los archivos del proyecto al contenedor desde la ruta rust
COPY ./rust ./ 

# CMD por defecto para mantener el contenedor vivo (útil para docker exec)
CMD ["tail", "-f", "/dev/null"]


