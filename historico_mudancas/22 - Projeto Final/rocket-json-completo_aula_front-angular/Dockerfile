# Use a imagem oficial do Node.js como imagem base para compilar o aplicativo
FROM node:21.6.2 as build-stage

# Defina o diretório de trabalho no contêiner
WORKDIR /app

# Copie o package.json e o package-lock.json (ou yarn.lock)
COPY package*.json ./

# Atualize o npm para a versão mais recente
RUN npm install -g npm@10.5.0

# Aumentar o tempo limite do npm e usar um registro alternativo se necessário
RUN npm config set fetch-timeout 120000
RUN npm config set registry "https://registry.npm.taobao.org"

# Instale as dependências
RUN npm install

# Copie o restante dos arquivos do projeto
COPY . .

# Compila o aplicativo Angular
RUN npm run build

# Use uma imagem Nginx para servir o aplicativo
FROM nginx:stable-alpine as production-stage

# Copie os arquivos construídos do Angular para o diretório de conteúdo do Nginx
COPY --from=build-stage /app/dist/ /usr/share/nginx/html

# Exponha a porta 80 (ou a porta que você deseja expor)
EXPOSE 80

# Inicie o Nginx quando o contêiner iniciar
CMD ["nginx", "-g", "daemon off;"]
