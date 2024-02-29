#!/bin/bash

# Sai em caso de erro
set -e

echo "Compilando o projeto..."
cargo build --release

echo "Executando o projeto..."
./target/release/docs
