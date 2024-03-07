#!/bin/bash

# Defina suas credenciais e informações do banco de dados
DB_USER="root"
DB_PASS="root"
DB_HOST="localhost"
DB_NAME="rust_actix_api_projeto_final"

# Caminho para o arquivo SQL
SQL_FILE="restore.sql"

echo "Iniciando o processo de restauração do banco de dados..."

# Conecte-se ao MySQL/MariaDB e execute o script SQL
mysql -u"$DB_USER" -p"$DB_PASS" -h "$DB_HOST" < "$SQL_FILE"

if [ $? -eq 0 ]; then
    echo "Restauração concluída com sucesso."
else
    echo "Falha ao restaurar o banco de dados."
fi
