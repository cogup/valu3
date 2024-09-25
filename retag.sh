#!/bin/bash

# Verifica se o arquivo VERSION.txt existe
if [ ! -f "VERSION.txt" ]; then
    echo "Arquivo VERSION.txt não encontrado."
    exit 1
fi

# Lê a última tag do arquivo VERSION.txt
last_tag=$(<VERSION.txt)

# Verifica se a nova tag foi fornecida como argumento
if [ "$#" -ne 1 ]; then
    echo "Uso: $0 <new_tag>"
    exit 1
fi

new_tag=$1

files=("valu3/Cargo.toml" "valu3_derive/Cargo.toml" "valu3/README.md" "valu3_derive/README.md" "README.md" "VERSION.txt")

# Loop através dos files e realiza o replace
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        if sed -i "s|$last_tag|$new_tag|g" "$file"; then
            echo "Substituído '$last_tag' por '$new_tag' em $file"
        else
            echo "Erro ao substituir em $file"
        fi
    else
        echo "Arquivo $file não encontrado."
    fi
done
