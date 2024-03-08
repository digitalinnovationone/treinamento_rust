CREATE TABLE clientes (
    id INTEGER PRIMARY KEY,
    nome varchar(150) NOT NULL,
    telefone varchar(15) NOT NULL
);

CREATE TABLE produtos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nome VARCHAR(100) NOT NULL,
    descricao VARCHAR(255),
    imagem VARCHAR(1000),
    preco REAL NOT NULL
);

CREATE TABLE pedidos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    valor_total REAL NOT NULL,
    cliente_id INTEGER NOT NULL,
    data DATETIME NOT NULL,
    pago INTEGER NOT NULL CHECK (pago IN (0, 1))
);

CREATE TABLE pedido_produtos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pedido_id INTEGER NOT NULL,
    produto_id INTEGER NOT NULL,
    quantidade INTEGER NOT NULL,
    FOREIGN KEY (pedido_id) REFERENCES pedidos(id),
    FOREIGN KEY (produto_id) REFERENCES produtos(id)
);


INSERT INTO clientes (nome, telefone) VALUES ('João das Camisetas', '(11) 98765-4321');


INSERT INTO produtos (nome, descricao, imagem, preco) VALUES 
('Camiseta Azul', 'Camiseta azul royal, 100% algodão, ideal para sublimação.', 'https://loja.comerciomix.com.br/media/catalog/product/cache/fb4f878514d02efd710032ded901d118/c/a/camiseta-azul-royal-para-sublima_o-tradicional_1.jpg', 50.00),
('Camiseta Verde', 'Camiseta verde bandeira, em algodão, toque macio e confortável.', 'https://img.irroba.com.br/fit-in/600x600/filters:fill(fff):quality(80)/jrconfec/catalog/camiseta-masc-algodao/mockups-camisetas-lisas-jr-verde.jpg', 45.00),
('Camiseta Preta', 'Camiseta preta básica, poliéster e algodão, perfeita para o dia a dia.', 'https://cdn.shoppub.io/cdn-cgi/image/w=1000,h=1000,q=80,f=auto/rota34/media/uploads/produtos/foto/d7e6ca26b80a1_0101_camada-51.jpg', 55.00),
('Camiseta Bordo', 'Camiseta bordo, elegante e versátil, ideal para eventos casuais.', 'https://lojausereserva.vteximg.com.br/arquivos/ids/8358420-500-auto/0088103568_01.jpg?v=638343932992500000', 60.00);


INSERT INTO pedidos (valor_total, cliente_id, data, pago) VALUES (210.00, 1, '2024-03-07', 1);


INSERT INTO pedido_produtos (pedido_id, produto_id, quantidade) VALUES 
(1, 1, 1),
(1, 2, 1),
(1, 3, 1),
(1, 4, 1);
