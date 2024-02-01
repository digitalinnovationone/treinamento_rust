####!/bin/bash

##### GDB (GNU Debugger) #####
# GDB é amplamente usado para debugar programas em Rust. 
# É particularmente útil para inspecionar o estado do programa em tempo de execução, como variáveis, pilhas de chamadas e threads.
# Para usar o GDB com Rust, é recomendável compilar o programa com símbolos de debug (rustc -g seu_programa.rs) para obter informações detalhadas durante a sessão de debug.

### Instalação: ###

#  MacOS:
#    brew install gdb

#  Windows (usando Chocolatey):
#    choco install gdb

#  Linux:
#    Para distribuições baseadas em Debian (como Ubuntu e Debian):
#    sudo apt install gdb

#    Para distribuições baseadas no Fedora:
#    sudo dnf install gdb

#    Para distribuições baseadas no Red Hat (como RHEL e CentOS):
#    sudo yum install gdb

### Verificar instalação: ###
#    gdb --version


### Comandos Básicos do GDB: ###
#   b(break) <arquivo>:<linha>: Para adicionar um ponto de interrupção em um arquivo e linha específicos.
#   b src/main.rs:2 Para adicionar um ponto de interrupção em um arquivo e linha específicos.

#   r(run): Para iniciar a execução do seu programa dentro do LLVM.
#   n(next): Para executar a próxima instrução (sem entrar em funções).
#   s(step): Para executar a próxima instrução, entrando em funções.
#   p(print) <variável>: Para imprimir o valor de uma variável.
#   l(list): Mostra em que local você está
#   c(continue): Para continuar a execução até o próximo ponto de interrupção.
#   q(quit): para sair pedindo confirmação

# faz o build
cargo build 

# Verificando se o build foi bem-sucedido antes de iniciar o GDB
if [ $? -eq 0 ]; then
    echo "Iniciando o GDB para depuração..."
    gdb target/debug/treinamento_rust # executa o binário com debugger
else
    echo "A compilação falhou, por favor, verifique os erros."
fi