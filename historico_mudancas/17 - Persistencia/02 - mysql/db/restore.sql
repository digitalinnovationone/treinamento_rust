CREATE DATABASE IF NOT EXISTS clientes_rust_db;

USE clientes_rust_db;

CREATE TABLE IF NOT EXISTS clientes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nome VARCHAR(255),
    telefone VARCHAR(20)
);

INSERT INTO clientes (nome, telefone) VALUES
('Danilo', '11 9999-9991'),
('Sheila', '11 2222-2222'),
('xxx', 'xxx');
