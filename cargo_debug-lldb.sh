####!/bin/bash

##### LLDB (LLVM Debugger) #####
# LLDB é parte do projeto LLVM e funciona bem com Rust, especialmente em plataformas onde GDB não é a opção ideal, como macOS.
# Assim como o GDB, é recomendado compilar com símbolos de debug para um melhor resultado.

### Instalação: ###

#  MacOS:
#    brew install lldb

#  Windows (usando Chocolatey):
#    choco install lldb

#  Linux:
#    Para distribuições baseadas em Debian (como Ubuntu e Debian):
#    sudo apt install lldb

#    Para distribuições baseadas no Fedora:
#    sudo dnf install lldb

#    Para distribuições baseadas no Red Hat (como RHEL e CentOS):
#    sudo yum install lldb

### Verificar instalação: ###
#    lldb --version


### Comandos Básicos do LLDB: ###
#   b(break) <arquivo.rs>:<linha>: Para adicionar um ponto de interrupção em um arquivo e linha específicos.
#   b main.rs:2 Para adicionar um ponto de interrupção em um arquivo e linha específicos.

#   r(run): Para iniciar a execução do seu programa dentro do LLVM.
#   n(next): Para executar a próxima instrução (sem entrar em funções).
#   s(step): Para executar a próxima instrução, entrando em funções.
#   p(print) <variável>: Para imprimir o valor de uma variável.
#   l(list): Mostra em que local você está
#   c(continue): Para continuar a execução até o próximo ponto de interrupção.
#   exit: para sair
#   q(quit): para sair pedindo confirmação

# faz o build
cargo build 

# Verificando se o build foi bem-sucedido antes de iniciar o LLVM
if [ $? -eq 0 ]; then
    echo "Iniciando o LLVM para depuração..."
    lldb target/debug/treinamento_rust # executa o binário com debugger
    # lldb out/main # executa o binário com debugger
else
    echo "A compilação falhou, por favor, verifique os erros."
fi