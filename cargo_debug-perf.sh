####!/bin/bash

##### Perf #####
# Perf é uma ferramenta de profiling de desempenho no nível do sistema. 
# É útil para analisar o desempenho de aplicações Rust em termos de uso de CPU, ciclos de instrução, cache misses, e outros.
# Compile o programa com símbolos de debug para melhores resultados.

### Instalação: ###

#  MacOS:
#    brew install perf # sem suporte

#  Windows (usando Chocolatey):
#    choco install perf # sem suporte, instale pelo WSL do linux

#  Linux:
#    Para distribuições baseadas em Debian (como Ubuntu e Debian):
#    sudo apt-get install linux-tools-common linux-tools-generic linux-tools-`uname -r`

#    Para distribuições baseadas no Fedora:
#    sudo dnf install perf

#    Para distribuições baseadas no Red Hat (como RHEL e CentOS):
#    sudo yum install perf

### Verificar instalação: ###
#    perf --version

# faz o build
cargo build 

if [ $? -eq 0 ]; then
    echo "Iniciando inspeção Perf ..."
    perf record target/debug/treinamento_rust
    perf report
else
    echo "A compilação falhou, por favor, verifique os erros."
fi