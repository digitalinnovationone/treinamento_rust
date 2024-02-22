use crate::models::cliente::Cliente;

pub fn get_clientes() -> Vec<Cliente> {
    vec![
        Cliente { id: 1, nome: "Cliente 1".to_string(), cpf: "000.000.000-01".to_string() },
        Cliente { id: 2, nome: "Cliente 2".to_string(), cpf: "000.000.000-02".to_string() },
    ]
}