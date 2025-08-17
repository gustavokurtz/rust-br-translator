# 🦀 Tradutor CLI - Rust

Uma ferramenta de linha de comando simples e rápida para traduzir palavras e frases do português para inglês usando a API LibreTranslate.

## 🚀 Como usar

```bash
# Palavra única
cargo run -- olá
# Output:
# Tradução: hi
# Alternativas: ["hello", "hey"]

# Frases (com aspas)
cargo run -- "como você está?"
# Output:
# Tradução: how are you?
# Alternativas: ["how are you doing?", "how do you do?"]
```

## 📦 Instalação

### Via Cargo (Recomendado)

```bash
cargo install rust-br-translator
rust-br-translator olá mundo
```

### Via Código Fonte

1. Clone o projeto:

```bash
git clone https://github.com/gustavokurtz/rust-br-translator.git
cd rust-br-translator
```

2. Instale dependências do sistema (Linux):

```bash
# Ubuntu/Debian
sudo apt install pkg-config libssl-dev

# Fedora/CentOS
sudo dnf install pkg-config openssl-devel
```

3. Execute:

```bash
cargo run -- "palavra ou frase"
```

## 📋 Pré-requisitos

- Rust instalado (https://rustup.rs/)
- Conexão com internet (usa API online)
- No Linux: `pkg-config` e `libssl-dev` instalados

## 🔧 Como funciona

1. **Input**: Recebe palavra/frase em português como argumento
2. **API**: Consulta LibreTranslate API (gustavodev.tech/translate)
3. **Output**: Retorna tradução principal + alternativas
4. **Offline**: Não funciona offline (requer internet)

## 🎯 Recursos

- ✅ Tradução em tempo real via API
- ✅ Múltiplas alternativas de tradução
- ✅ Suporte a palavras e frases completas
- ✅ API gratuita (sem necessidade de chave)
- ✅ Mensagens de erro claras
- ✅ Interface CLI simples
- ✅ Assíncrono e rápido

## 📚 Exemplos

```bash
# Palavras simples
cargo run -- casa          # house
cargo run -- amarelo       # yellow
cargo run -- computador    # computer

# Frases (sempre usar aspas)
cargo run -- "bom dia"           # good morning
cargo run -- "como vai?"         # how are you?
cargo run -- "eu gosto de café"  # I like coffee

# Casos de erro
cargo run -- xywz123      # Erro de tradução ou palavra inexistente
```

## 🔄 Mudanças da v0.1.0 → v0.2.0

### ✅ Adicionado

- Integração com API LibreTranslate
- Suporte a frases completas
- Múltiplas alternativas de tradução
- Tradução online em tempo real
- Tratamento de erros de rede

### ❌ Removido

- HashMap interno (não era escalável)
- Funcionamento offline
- Dicionário limitado integrado

### 🔧 Melhorado

- Qualidade das traduções (API vs dicionário fixo)
- Cobertura de vocabulário (ilimitada)
- Flexibilidade (palavras + frases)

## 🤝 Contribuindo

0. Olhe as issues
1. Faça um fork do projeto
2. Crie sua feature branch (`git checkout -b feature/nova-funcionalidade`)
3. Commit suas mudanças (`git commit -m 'feat: adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/nova-funcionalidade`)
5. Abra um Pull Request

## 📄 Licença

Este projeto é open source e está disponível sob a [MIT License](LICENSE).

## 🙏 Agradecimentos

- [LibreTranslate](https://libretranslate.com/) - API de tradução gratuita
- [gustavodev.tech](https://gustavodev.tech/) - Endpoint da API utilizada
