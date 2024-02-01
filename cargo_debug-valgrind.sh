####!/bin/bash

##### Valgrind #####
# Valgrind é uma ferramenta poderosa para detecção de vazamentos de memória e erros relacionados. 
# Embora Rust seja conhecido por sua segurança de memória, Valgrind pode ser útil para identificar problemas em código não seguro (unsafe) ou em bibliotecas C/C++ vinculadas.
# Para usar, compile o programa normalmente e execute-o através do Valgrind.

### Instalação: ###

#  MacOS:
#    brew install valgrind # sem suporte

#  Windows (usando Chocolatey):
#    choco install valgrind # sem suporte, instale pelo WSL do linux

#  Linux:
#    Para distribuições baseadas em Debian (como Ubuntu e Debian):
#    sudo apt install valgrind

#    Para distribuições baseadas no Fedora:
#    sudo dnf install valgrind

#    Para distribuições baseadas no Red Hat (como RHEL e CentOS):
#    sudo yum install valgrind

### Verificar instalação: ###
#    valgrind --version

# faz o build
cargo build 

if [ $? -eq 0 ]; then
    echo "Iniciando inspeção Valgrind ..."
    valgrind target/debug/treinamento_rust
else
    echo "A compilação falhou, por favor, verifique os erros."
fi