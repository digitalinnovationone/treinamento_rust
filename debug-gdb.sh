####!/bin/bash

##### GDB (GNU Debugger) #####
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
#   run: Para iniciar a execução do seu programa dentro do GDB.
#   break <arquivo.rs>:<linha>: Para adicionar um ponto de interrupção em um arquivo e linha específicos.
#   next: Para executar a próxima instrução (sem entrar em funções).
#   step: Para executar a próxima instrução, entrando em funções.
#   print <variável>: Para imprimir o valor de uma variável.
#   continue: Para continuar a execução até o próximo ponto de interrupção.

# faz o build
rustc -g src/main.rs -o out/main

# Verificando se o build foi bem-sucedido antes de iniciar o GDB
if [ $? -eq 0 ]; then
    echo "Iniciando o GDB para depuração..."
    gdb out/main # executa o binário com debugger
else
    echo "A compilação falhou, por favor, verifique os erros."
fi