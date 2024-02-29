// struct Pedido {
//     nome_cliente: String,
//     valor: f32,
//     entregue: bool,
// }


// // === Abordagem Imperativa =======
// fn main() {
//     let pedidos = vec![
//         Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
//         Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
//         Pedido { nome_cliente: String::from("Carol"), valor: 100.0, entregue: true },
//     ];

//     let mut valor_total = 0.0; // pode ter problemas em concorrencia variavel depenendo da condição

//     for pedido in pedidos {
//         if pedido.entregue {
//             valor_total += pedido.valor;
//         }
//     }

//     println!("O valor total dos pedidos entregues é: {:.2}", valor_total);
// }



// fn pedido_entregue(pedido: &Pedido) -> bool {
//     pedido.entregue
// }

// //// === Abordagem funcional =======
// fn main() {
//     let pedidos = vec![
//         Pedido { nome_cliente: String::from("Alice"), valor: 150.0, entregue: true },
//         Pedido { nome_cliente: String::from("Bob"), valor: 250.0, entregue: false },
//         Pedido { nome_cliente: String::from("Carol"), valor: 100.0, entregue: true },
//     ];

//     //// === Conceito de função que retorna função ===
//     // iter() - Cria um iterador sobre a coleção.
//     // filter(|pedido| pedido.entregue) - Filtra os clientes entregues
//     // map(|pedido| pedido.valor) - mapeia os itens retornando um array de f32
//     // sum() - Soma os dados retornados pelo array


//     let valor_total: f32 = pedidos.iter()
//                                    .filter(|&pedido| pedido_entregue(pedido))
//                                    .map(|pedido| pedido.valor)
//                                    .sum();

//     println!("O valor total dos pedidos entregues é: {:.2}", valor_total);
// }


//// === Abordagem funcional Closures =======

/*
Closures são funções anônimas que podem capturar variáveis do escopo onde foram definidas 
para uso posterior. 

Elas são bastante úteis para tarefas como passagem de comportamento como argumento para outras funções,
construção de abstrações de controle e manipulação de dados de coleções.

Características das Closures
Anônimas: Closures geralmente não têm um nome.

Capturam o ambiente: Podem capturar variáveis do contexto onde são definidas, 
seja por valor ou por referência.

Flexíveis: Podem ser armazenadas em variáveis, passadas como argumentos para outras funções, e mais.

Tipagem forte: Assim como outras funções em Rust, as closures são fortemente tipadas, 
mas o compilador de Rust muitas vezes pode inferir seus tipos automaticamente.

*/

// fn main() {
//     let x = 4;

//     let exemplo_closure = |parametro| {
//         // corpo da closure
//         parametro + x // consegue pegar variável do scopo em que ela está inserida
//     };
//     println!("exemplo_closure: {}", exemplo_closure(6));


//     // Uma closure que captura `x` do ambiente circundante
//     let igual_a_x = |z| z == x;

//     let y = 4;

//     println!("O resultado da comparação é: {}", igual_a_x(y));
// }





// //// === Função de Alta Ordem & Closure =======
// fn soma(x: i32, y: i32) -> impl Fn(i32) -> i32 {
//     let r = x + y;
//     move |multiplicador| r * multiplicador // closure
// }

// fn main() {
//     let resultado_soma = soma(5, 3); // 5 + 3 = 8
//     let resultado_final = resultado_soma(2); // 8 * 2 = 16

//     println!("Resultado da multiplicação: {}", resultado_final); // 8 * 2 = 16
// }






//// === Abordagem funcional =======
/*

Em resumo, a escolha entre impl Fn e Box<dyn Fn> depende das necessidades específicas do seu código: 
se você valoriza a eficiência e sabe exatamente o tipo da closure que está retornando, impl Fn pode ser a melhor escolha. 

Se você precisa de flexibilidade para retornar diferentes tipos de closures que implementam o mesmo trait, 
Box<dyn Fn> é a opção apropriada.

*/
// fn soma(x: i32, y: i32) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
//     let r: i32 = x + y;
//     move |multiplicador| {
//         let r_mult = r * multiplicador;
//         Box::new(move |sub| r_mult - sub)
//     }
// }


// fn main() {
//     let multiplica = soma(5, 3); // 5 + 3 = 8
//     let subtrai = multiplica(2); // 8 * 2 = 16
//     let resultado = subtrai(4);  // 16 - 4 = 12

//     println!("O resultado é: {}", resultado); // O resultado é: 12
// }






//// === Abordagem funcional =======

// fn cria_somador(x: i32) -> Box<dyn Fn(i32) -> i32> {
//     Box::new(move |y| x + y)
// }

// fn main() {
//     let somador_10 = cria_somador(10);
//     println!("10 + 5 = {}", somador_10(5));
//     println!("10 + 10 = {}", somador_10(10));
//     println!("10 + 30 = {}", somador_10(30));
//     println!("10 + 20 = {}", somador_10(20));
// }





//// === Abordagem funcional calculo de salário =======

// fn main() {
//     let salario_bruto = 5000.0;

//     // Cálculo do salário líquido
//     fn calcular_salario_liquido(salario_bruto: f64) -> f64 {
//         // Função interna para desconto do plano de saúde
//         fn desconto_plano_saude(salario: f64) -> f64 {
//             salario * 0.10 // Desconto de 10%
//         }

//         // Função interna para desconto do plano dentário
//         fn desconto_plano_dentario(salario: f64) -> f64 {
//             salario * 0.05 // Desconto de 5%
//         }

//         // Função interna para desconto de vale-refeição
//         fn desconto_vale_refeicao(salario: f64) -> f64 {
//             salario * 0.03 // Desconto de 3%
//         }

//         let desconto_saude = desconto_plano_saude(salario_bruto);
//         let desconto_dentario = desconto_plano_dentario(salario_bruto);
//         let desconto_refeicao = desconto_vale_refeicao(salario_bruto);

//         salario_bruto - desconto_saude - desconto_dentario - desconto_refeicao
//     }

//     let salario_liquido = calcular_salario_liquido(salario_bruto);
//     println!("Salário líquido: {:.2}", salario_liquido);
// }






//// === Abordagem funcional calculo salario funções separadas =======

// fn main() {
//     let salario_bruto = 5000.0;

//     // Função interna para desconto do plano de saúde
//     fn desconto_plano_saude(salario: f64) -> f64 {
//         salario * 0.10 // Desconto de 10%
//     }

//     // Função interna para desconto do plano dentário
//     fn desconto_plano_dentario(salario: f64) -> f64 {
//         salario * 0.05 // Desconto de 5%
//     }

//     // Função interna para desconto de vale-refeição
//     fn desconto_vale_refeicao(salario: f64) -> f64 {
//         salario * 0.03 // Desconto de 3%
//     }

//     // Cálculo do salário líquido
//     fn calcular_salario_liquido(salario_bruto: f64) -> f64 {
//         let desconto_saude = desconto_plano_saude(salario_bruto);
//         let desconto_dentario = desconto_plano_dentario(salario_bruto);
//         let desconto_refeicao = desconto_vale_refeicao(salario_bruto);

//         salario_bruto - desconto_saude - desconto_dentario - desconto_refeicao
//     }

//     let salario_liquido = calcular_salario_liquido(salario_bruto);

//     println!("Salário líquido: {:.2}", salario_liquido);
// }



//// === Abordagem funcional calculo salario com closure =======

// fn main() {
//     let salario_bruto = 5000.0;

//     // Cálculo do salário líquido usando closures
//     let calcular_salario_liquido = |salario_bruto: f64| {
//         // Closure para desconto do plano de saúde
//         let desconto_plano_saude = |salario: f64| salario * 0.10;

//         // Closure para desconto do plano dentário
//         let desconto_plano_dentario = |salario: f64| salario * 0.05;

//         // Closure para desconto de vale-refeição
//         let desconto_vale_refeicao = |salario: f64| salario * 0.03;

//         let desconto_saude = desconto_plano_saude(salario_bruto);
//         let desconto_dentario = desconto_plano_dentario(salario_bruto);
//         let desconto_refeicao = desconto_vale_refeicao(salario_bruto);

//         salario_bruto - desconto_saude - desconto_dentario - desconto_refeicao
//     };

//     let salario_liquido = calcular_salario_liquido(salario_bruto);
//     println!("Salário líquido: {:.2}", salario_liquido);
// }







//// === Função de Alta Ordem =======

// fn main() {
//     let salario_bruto = 5000.0;

//     // Função de alta ordem que aplica descontos ao salário
//     fn aplicar_descontos(salario: f64, descontos: Vec<fn(f64) -> f64>) -> f64 {
//         let total_descontos = descontos.iter().fold(0.0, |valor_param, funcao_closure| valor_param + funcao_closure(salario));
//         salario - total_descontos
//     }

//     // Cálculo do salário líquido usando a função de alta ordem
//     let salario_liquido = aplicar_descontos(salario_bruto, vec![
//         |salario: f64| salario * 0.10, // Desconto do plano de saúde: 10%
//         |salario: f64| salario * 0.05, // Desconto do plano dentário: 5%
//         |salario: f64| salario * 0.03, // Desconto de vale-refeição: 3%
//     ]);

//     println!("Salário líquido: {:.2}", salario_liquido);
// }





//// === Função de Alta Ordem =======
// fn desconto_plano_saude(salario: f64) -> f64 {
//     salario * 0.10 // Desconto de 10%
// }

// fn desconto_plano_dentario(salario: f64) -> f64 {
//     salario * 0.05 // Desconto de 5%
// }

// fn desconto_vale_refeicao(salario: f64) -> f64 {
//     salario * 0.03 // Desconto de 3%
// }

// // Função de alta ordem que aplica descontos ao salário
// fn aplicar_descontos(salario: f64, descontos: Vec<fn(f64) -> f64>) -> f64 {
//     let total_descontos = descontos.iter().fold(0.0, |valor_param, funcao_closure| valor_param + funcao_closure(salario));
//     salario - total_descontos
// }

// fn main() {
//     let salario_bruto = 5000.0;

//     // Cálculo do salário líquido usando a função de alta ordem
//     let salario_liquido = aplicar_descontos(salario_bruto, vec![
//         desconto_plano_saude,
//         desconto_plano_dentario,
//         desconto_vale_refeicao,
//     ]);

//     println!("Salário líquido: {:.2}", salario_liquido);
// }






//// === Abordagem com struct (Com abordagem em Orientação a Objetos, pois usa conceito de mutabilidade) =======

// struct CalculadoraSalario {
//     salario_bruto: f64,
//     total_descontos: f64,
// }

// impl CalculadoraSalario {
//     fn new(salario_bruto: f64) -> Self {
//         CalculadoraSalario {
//             salario_bruto: salario_bruto,
//             total_descontos: 0.0,
//         }
//     }

//     fn desconto_plano_saude(mut self) -> Self {
//         self.total_descontos += self.salario_bruto * 0.10; // Desconto de 10%
//         self
//     }

//     fn desconto_plano_dentario(mut self) -> Self {
//         self.total_descontos += self.salario_bruto * 0.05; // Desconto de 5%
//         self
//     }

//     fn desconto_vale_refeicao(mut self) -> Self {
//         self.total_descontos += self.salario_bruto * 0.03; // Desconto de 3%
//         self
//     }

//     fn valor(self) -> f64 {
//         self.salario_bruto - self.total_descontos
//     }
// }

// fn main() {
//     let salario_bruto = 5000.0;
//     let salario_liquido = CalculadoraSalario::new(salario_bruto)
//         .desconto_plano_saude()
//         .desconto_plano_dentario()
//         .desconto_vale_refeicao()
//         .valor();

//     println!("Salário líquido: {:.2}", salario_liquido);
// }







//// === Exemplo de recursão =======
// fn soma_recursiva(n: i32) -> i32 {
//     if n == 0 {
//         0
//     } else {
//         n + soma_recursiva(n - 1)
//     }
// }

// fn main() {
//     let n = 5;
//     println!("A soma recursiva dos números até {} é: {}", n, soma_recursiva(n));
// }


// //// === Exemplo imperativa =======
// fn soma_loop(n: i32) -> i32 {
//     let mut soma = 0;
//     for i in 1..=n {
//         soma += i;
//     }
//     soma
// }

// fn main() {
//     let n = 5;
//     println!("A soma com loop dos números até {} é: {}", n, soma_loop(n));
// }










// -------------------Pattern Matching-------------------------
enum Semafaro {
    Verde,
    Amarelo,
    Vermelho,
}

fn acao(semafaro: Semafaro) {
    match semafaro {
        Semafaro::Verde => println!("Siga"),
        Semafaro::Amarelo => println!("Atenção"),
        Semafaro::Vermelho => println!("Pare"),
    }
}

fn main() {
    acao(Semafaro::Verde);
    acao(Semafaro::Amarelo);
    acao(Semafaro::Vermelho);
}















//// ======= Conclusão ========

/*

### 1. **Entender os Conceitos Fundamentais**

- **Imutabilidade:** Dados imutáveis não podem ser modificados após sua criação. Isso ajuda a evitar efeitos colaterais e torna o código mais previsível.
- **Funções Puras:** São funções que, para os mesmos inputs, sempre retornarão o mesmo output e não têm efeitos colaterais (como modificar uma variável externa ou realizar I/O).
- **Expressões sobre Instruções:** Em programação funcional, você expressa o "o quê" ao invés do "como", focando em expressões que calculam valores em vez de instruções que alteram o estado do programa.

### 2. **Utilizar Recursos e Ferramentas da Linguagem**

- **Funções de Alta Ordem:** São funções que podem receber outras funções como argumentos ou retorná-las como resultado. Elas são fundamentais para criar abstrações poderosas e reutilizáveis.
- **Closures:** Funções que podem capturar e utilizar variáveis do escopo onde foram criadas.
- **Estruturas de Dados Imutáveis:** Preferir o uso de estruturas de dados que não podem ser alteradas após sua criação.
- **Pattern Matching:** Uma forma de verificar e destrinchar valores de acordo com um padrão. É particularmente útil para trabalhar com tipos algébricos de dados, como enums em Rust.


### 3. **Praticar Técnicas Comuns da Programação Funcional**

- **Recursão:** Muitas vezes, em programação funcional, loops são expressos através de recursão. É importante entender como implementar recursão de forma eficiente, especialmente a recursão de cauda (tail recursion), que é otimizada em muitas linguagens para evitar estouro de pilha.
- **Map, Filter, e Reduce:** São operações fundamentais para trabalhar com coleções de dados de maneira declarativa.
- **Composição de Funções:** A capacidade de combinar duas ou mais funções para criar uma nova função.

*/