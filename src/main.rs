#[derive(Debug)]
struct Cliente {
    id: u32,
    nome: String,
    cpf: String,
    salario: f32
}

impl Cliente {
    fn cpf_valido(&self) -> bool {
        if self.cpf.is_empty() {
            return false;
        }

        true
    }

    fn adiciona_sobrenome(&mut self){
        self.nome += " da Silva";
    }

    fn aumento(&mut self, valor:f32){
        self.salario += valor;
    }

    fn aumento2(&self, valor:f32) -> f32{
        self.salario + valor
    }
}

fn main() {
    let mut cliente = Cliente { 
        id: 1,
        nome: String::from("Leadro"),
        cpf: String::from("222.555.553-00"),
        salario: 5000.0
    };

    cliente.adiciona_sobrenome();

    cliente.aumento(1000.0);

    let valido = if cliente.cpf_valido()  {
        "Verdadeiro"
    } else {
        "Falso"
    };

    println!(
        "O CPF do cliente({}):  {} é {} \n{:#?}",
        cliente.id,
        cliente.nome,
        valido,
        cliente
    );
}
