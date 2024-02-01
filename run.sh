####!/bin/bash

# rustc -g src/main.rs -o out/main # faz o build em modo debugger
rustc src/main.rs -o out/main # faz o build em modo release
./out/main # executa o binário