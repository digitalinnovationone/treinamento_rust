/*

=== Motivos ===
Gerenciamento Automático de Memória:
O .NET utiliza um coletor de lixo (Garbage Collector - GC) para 
gerenciar a memória automaticamente. Isso significa que a memória para 
objetos é alocada na heap gerenciada, e o GC se encarrega de desalocar 
objetos quando eles não são mais acessíveis. 
Não há necessidade de desalocar manualmente a memória, evitando 
assim muitos dos problemas comuns associados a dangling pointers.

Tipos de Referência vs. Tipos de Valor: 
No .NET, strings são tipos de referência. Quando você retorna local da 
função tentativa_de_dangling_pointer, você está retornando uma referência 
para a string alocada na heap gerenciada. 
A propriedade (ownership) dessa string é passada para o chamador, 
e a string permanece acessível enquanto houver uma referência a ela. 
O coletor de lixo se encarrega de desalocá-la quando não houver mais referências.

Strings são Imutáveis: 
Em C#, strings são imutáveis; quando você cria uma string, ela não pode ser alterada. 
Se você precisa modificar uma string, uma nova string é criada. 
Isso significa que retornar uma string de uma função simplesmente passa a 
referência para o objeto string existente, sem a necessidade de 
cópias ou preocupações com a validade da referência.
*/

using System;

class Program {
    static string TentativaDeDanglingPointer() {
        string local = "isso é um teste";
        return local;
    }

    static void Main() {
        var resultado = TentativaDeDanglingPointer();
        Console.WriteLine(resultado);
    }
}
