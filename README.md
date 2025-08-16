# 🦀 Tradutor CLI - Rust

Uma ferramenta de linha de comando simples e rápida para traduzir palavras do português para inglês.

## 🚀 Como usar

```bash
cargo run -- amarelo
# Output: yellow

cargo run -- casa
# Output: house
```

## 📋 Pré-requisitos

- Rust instalado (https://rustup.rs/)
- Arquivo `dicionario.txt` na raiz do projeto

## 🛠️ Instalação

1. Clone o projeto:

```bash
git clone <seu-repositorio>
cd grrs
```

2. Crie o arquivo de dicionário:

```bash
echo "amarelo=yellow
casa=house
carro=car
água=water
livro=book" > dicionario.txt
```

3. Execute:

```bash
cargo run -- palavra
```

## 📝 Formato do dicionário

O arquivo `dicionario.txt` deve seguir o formato:

```
palavra_portuguesa=english_word
```

Exemplo:

```
amarelo=yellow
casa=house
carro=car
```

## 🔧 Como funciona

1. **Input**: Recebe uma palavra em português como argumento
2. **Busca**: Procura no arquivo `dicionario.txt`
3. **Output**: Retorna a tradução em inglês
4. **Erro**: Mostra "Palavra não encontrada" se não existir

## 🎯 Recursos

- ✅ Busca rápida em arquivo local
- ✅ Funciona offline
- ✅ Case-insensitive (funciona com maiúsculas/minúsculas)
- ✅ Mensagens de erro claras
- ✅ Sintaxe simples

## 📚 Exemplos

```bash
# Traduzir palavras
cargo run -- amarelo    # yellow
cargo run -- CASA       # house
cargo run -- Carro      # car

# Palavra não encontrada
cargo run -- elefante   # Palavra não encontrada
```

## 🚧 TODO

- [ ] Adicionar mais palavras ao dicionário
- [ ] Suporte para frases
- [ ] Tradução bidirecional (inglês → português)
- [ ] Interface interativa
- [ ] Exportar como binário executável

## 🤝 Contribuindo

1. Adicione novas palavras ao `dicionario.txt`
2. Faça um fork do projeto
3. Crie sua feature branch
4. Commit suas mudanças
5. Abra um Pull Request

## 📄 Licença

Este projeto é open source e está disponível sob a [MIT License](LICENSE).
