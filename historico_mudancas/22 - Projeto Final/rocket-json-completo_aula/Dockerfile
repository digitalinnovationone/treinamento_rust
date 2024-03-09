# Use a imagem nightly oficial do Rust como imagem base para compilar o aplicativo
FROM rustlang/rust:nightly as builder

# Crie um novo diretório para o aplicativo e defina-o como diretório de trabalho
WORKDIR /usr/src/app

# Copie os arquivos do seu projeto para o contêiner
COPY . .

# Compila seu aplicativo em modo de release
RUN cargo build --release

# Use uma imagem Ubuntu mais recente como imagem base para o contêiner final
FROM ubuntu:latest

# Instale as dependências necessárias no contêiner final
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    ca-certificates \
    libmariadb3 \
    && rm -rf /var/lib/apt/lists/*

# Defina o diretório de trabalho para o contêiner final
WORKDIR /usr/local/bin

# Copie o binário compilado e arquivos de configuração
COPY --from=builder /usr/src/app/target/release/rocket-json .
COPY --from=builder /usr/src/app/Rocket.toml .
COPY --from=builder /usr/src/app/.env .

# Quando o contêiner inicia, executa o binário
CMD ["./rocket-json"]

