use crate::models::Cliente;

pub fn obter_lista_clientes() -> Vec<Cliente> {
    let cliente1 = Cliente {
        id: 1,
        nome: String::from("Cliente 1"),
        cpf: String::from("111.111.111-11"),
    };
    
    let cliente2 = Cliente {
        id: 2,
        nome: String::from("Cliente 2"),
        cpf: String::from("222.222.222-22"),
    };
    
    let mut lista_clientes = Vec::new();
    lista_clientes.push(cliente1);
    lista_clientes.push(cliente2);
    
    lista_clientes
}
