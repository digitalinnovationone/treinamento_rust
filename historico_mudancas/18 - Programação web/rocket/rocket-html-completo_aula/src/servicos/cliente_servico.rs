use crate::models::cliente::Cliente;

pub fn get_clientes() -> Vec<Cliente> {
    vec![
        Cliente { id: 1, nome: "Sheila".to_string(), cpf: "123456764321".to_string() },
        Cliente { id: 2, nome: "Lana".to_string(), cpf: "23456543".to_string() },
        Cliente { id: 2, nome: "Liah".to_string(), cpf: "34565432".to_string() },
        Cliente { id: 3, nome: "Maria".to_string(), cpf: "34565432".to_string() },
        Cliente { id: 4, nome: "Luiza".to_string(), cpf: "34565432".to_string() },
        Cliente { id: 5, nome: "Madalena".to_string(), cpf: "34565432".to_string() },
    ]
}

pub fn criar_cliente(nome:String, cpf:String) -> bool {

    // salvar os dados no banco de dados
    println!("Nome do cliente {}", nome);
    println!("Cpf do cliente {}", cpf);

    true // false
}

pub fn alterar(id:u32, nome:String, cpf:String) -> bool {

    // salvar os dados no banco de dados
    println!("Id do cliente {}", id);
    println!("Nome do cliente {}", nome);
    println!("Cpf do cliente {}", cpf);

    false
}


pub fn excluir_por_id(id:u32) -> bool {

    // simula exclusão de dados
    println!("Id do cliente {}", id);

    true
}


pub fn get_cliente_por_id(_id: u32) -> Cliente {
    // Simulação de retornando um dado no banco de dados de cliente
    println!("ID: {}", _id);
    Cliente { id: _id, nome: "Cliente 1".to_string(), cpf: "000.000.000-01".to_string() }
}