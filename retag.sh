#!/bin/bash

# Verifica se foram fornecidos exatamente dois argumentos
if [ "$#" -ne 2 ]; then
    echo "Uso: $0 <last_tag> <new_tag>"
    exit 1
fi

last_tag=$1
new_tag=$2

files=("valu3/Cargo.toml" "valu3_derive/Cargo.toml" "valu3/README.md" "valu3_derive/README.md" "README.md" "VERSION.txt")

# Loop através dos files e realiza o replace
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        sed -i "s|$last_tag|$new_tag|g" "$file"
        echo "Substituído '$last_tag' por '$new_tag' em $file"
    else
        echo "file $file not found."
    fi
done
