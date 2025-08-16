# 🦀 Tradutor CLI - Rust

Uma ferramenta de linha de comando simples e rápida para traduzir palavras do português para inglês usando HashMap interno.

## 🚀 Como usar

```bash
cargo run -- amarelo
# Output: yellow

cargo run -- casa
# Output: house
```

## 📋 Pré-requisitos

- Rust instalado (https://rustup.rs/)
- Clap dependency configurada

## 🛠️ Instalação

1. Clone o projeto:

```bash
git clone <seu-repositorio>
cd grrs
```

2. Adicione dependência no Cargo.toml:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

3. Execute:

```bash
cargo run -- palavra
```

## 📝 Dicionário interno

O dicionário está integrado no código usando HashMap para máxima performance. Palavras disponíveis:

- amarelo → yellow
- casa → house
- carro → car
- água → water
- livro → book

## 🔧 Como funciona

1. **Input**: Recebe uma palavra em português como argumento
2. **Busca**: Consulta HashMap interno (O(1) - busca instantânea)
3. **Output**: Retorna a tradução em inglês
4. **Erro**: Mostra "Palavra não encontrada" se não existir

## 🎯 Recursos

- ✅ Busca ultra-rápida com HashMap (O(1))
- ✅ Funciona offline
- ✅ Zero dependência de arquivos externos
- ✅ Mensagens de erro claras
- ✅ Sintaxe simples
- ✅ Dicionário integrado no binário

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

- [ ] Adicionar mais palavras ao HashMap interno
- [ ] Suporte para frases
- [ ] Tradução bidirecional (inglês → português)
- [ ] Case-insensitive matching
- [ ] Interface interativa
- [ ] Exportar como binário executável

## 🤝 Contribuindo

1. Adicione novas palavras ao HashMap no código
2. Faça um fork do projeto
3. Crie sua feature branch
4. Commit suas mudanças
5. Abra um Pull Request

## 📄 Licença

Este projeto é open source e está disponível sob a [MIT License](LICENSE).
