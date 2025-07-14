# 🦀⚡ Prompt Builder GUI

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Slint](https://img.shields.io/badge/Slint-3584E4?style=for-the-badge&logo=gtk&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D4?style=for-the-badge&logo=windows&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**🚀 Uma aplicação desktop moderna e intuitiva para construção de prompts estruturados para IA**

*Construída com Rust + Slint para máxima performance e experiência de usuário fluida* ⚡

![Tests](https://img.shields.io/badge/Tests-146%20passing-brightgreen?style=flat-square)
![Coverage](https://img.shields.io/badge/Coverage-95%25-brightgreen?style=flat-square)
![Quality](https://img.shields.io/badge/Code%20Quality-A+-blue?style=flat-square)

</div>

---

## 🎯 **O que é o Prompt Builder GUI?**

Imagine uma ferramenta que transforma a complexa arte de criar prompts de IA em uma experiência visual, intuitiva e divertida! 🎨

**Prompt Builder GUI** é sua nova arma secreta para:
- 🧠 **Estruturar** prompts como um profissional
- 🎭 **Organizar** diferentes personas e contextos
- 📝 **Salvar** e **reutilizar** seus melhores prompts
- 🔄 **Iterar** rapidamente com preview em tempo real
- 📊 **Escalar** sua produtividade com IA

### 🌟 **Por que você vai amar:**

| Feature | Benefício | Status |
|---------|-----------|--------|
| 🎨 **Interface Moderna** | UI limpa e responsiva com esquema azul/verde | ✅ Concluído |
| 📂 **Sistema de Arquivos** | Abra, salve e gerencie prompts facilmente | ✅ Concluído |
| 🔍 **Preview em Tempo Real** | Veja o resultado final enquanto edita | ✅ Concluído |
| 📋 **Copy & Paste** | Copie prompts direto para a área de transferência | ✅ Concluído |
| 🧩 **Modular** | 9 seções organizadas para máxima flexibilidade | ✅ Concluído |
| 🎯 **Parsing Inteligente** | Detecta formatos automaticamente | ✅ Concluído |

---

## 🏗️ **Arquitetura do Projeto**

### 📁 **Estrutura Organizada como um Pro:**

```
🦀 prompt_builder_gui/
├── 📦 src/
│   ├── 🎯 main.rs                    # Entry point + parsing inteligente
│   ├── 📚 lib.rs                     # Biblioteca pública
│   ├── 🏗️ models/                   # Modelos de dados (9 módulos)
│   │   ├── 💡 few_shot.rs           # Exemplos de entrada/saída
│   │   ├── 🎭 context.rs            # Contexto e persona da IA
│   │   ├── 📝 main_content.rs       # Instruções principais
│   │   ├── 📎 auxiliary_content.rs  # Conteúdo auxiliar
│   │   ├── 🚫 limitations.rs        # Limitações e restrições
│   │   ├── 🔧 refactoring.rs        # Instruções de refatoração
│   │   ├── 🎨 guidance.rs           # Orientações de estilo
│   │   ├── ✅ tests.rs              # Requisitos de teste
│   │   ├── 📊 output_format.rs      # Formato de saída
│   │   └── 🔗 mod.rs               # Módulo principal
│   └── ⚙️ services/                 # Serviços core
│       ├── 🎪 prompt_generator.rs   # Motor de geração
│       ├── 💾 file_service.rs       # Sistema de arquivos
│       └── 🔗 mod.rs               # Módulo de serviços
├── 🎨 ui/
│   └── 🖼️ app-window.slint         # Interface Slint moderna
├── 🧪 tests/                       # Suíte de testes robusta
│   ├── 🔬 integration_prompt_generation.rs
│   ├── 📁 integration_file_operations.rs
│   └── 🔍 integration_parsing.rs
├── 📋 Cargo.toml                   # Dependências e config
└── 📖 README.md                    # Esta documentação épica
```

### 🧩 **As 9 Seções do Prompt Perfeito:**

| Seção | 🎯 Propósito | 🔥 Quando Usar |
|-------|-------------|----------------|
| **Few-Shot** 💡 | Exemplos práticos de entrada → saída | Ensinar comportamentos específicos |
| **Context** 🎭 | Definir persona e especialização da IA | Criar especialistas virtuais |
| **Main Content** 📝 | Instrução principal da tarefa | Core da sua solicitação |
| **Auxiliary** 📎 | Dados, códigos, exemplos extras | Fornecer material de referência |
| **Limitations** 🚫 | Restrições e limitações | Controlar escopo e qualidade |
| **Refactoring** 🔧 | Instruções de melhoria de código | Projetos de desenvolvimento |
| **Guidance** 🎨 | Tom, estilo, audiência | Personalizar comunicação |
| **Tests** ✅ | Requisitos de validação | Garantir qualidade técnica |
| **Output Format** 📊 | Estrutura da resposta | Padronizar resultados |

---

## 🚀 **Quick Start - Comece em 2 Minutos!**

### 📋 **Pré-requisitos:**
```bash
# 1. Instale o Rust (se ainda não tem)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Reinicie o terminal ou rode:
source ~/.cargo/env
```

### ⚡ **Instalação Instantânea:**
```bash
# Clone o repositório
git clone https://github.com/consultorsandro/prompt_builder_gui.git
cd prompt_builder_gui

# Execute e divirta-se! 🎉
cargo run
```

### 🎮 **Primeiros Passos:**

1. **📝 Preencha os campos** - Comece com Context e Main Content
2. **👀 Veja o preview** - Atualize automaticamente enquanto digita
3. **💾 Salve seu trabalho** - Use o botão "Salvar" para arquivos
4. **📂 Abra prompts salvos** - Botão "Abrir" com parsing inteligente
5. **📋 Copie e use** - Botão "Copiar" para área de transferência

---

## 🧪 **Estratégia de Testes de Classe Mundial**

### 📊 **Métricas de Qualidade:**

<div align="center">

| Métrica | Valor | Status |
|---------|-------|--------|
| **🧪 Testes Unitários** | 119 passando | ✅ |
| **🔗 Testes de Integração** | 27 passando | ✅ |
| **📈 Cobertura Total** | 146 testes | ✅ |
| **🛡️ Segurança** | Audit clean | ✅ |
| **📝 Formatação** | Cargo fmt | ✅ |
| **🔍 Análise Estática** | Clippy clean | ✅ |

</div>

### 🏗️ **Arquitetura de Testes:**

#### **🧪 Testes Unitários (119 testes)**
```
📦 Cada módulo com cobertura completa:
├── 💡 FewShot: 7 testes          # Criação, mutação, formatação
├── 🎭 Context: 8 testes          # Persona, debug, clonagem
├── 📝 MainContent: 9 testes      # Instruções, multiline, chars especiais
├── 📎 AuxiliaryContent: 10 testes # Dados auxiliares, exemplos grandes
├── 🚫 Limitations: 11 testes     # Restrições, múltiplas constraints
├── 🔧 Refactoring: 10 testes     # SOLID, performance, padrões
├── 🎨 Guidance: 11 testes        # Tom, audiência, formatação
├── ✅ Tests: 12 testes           # Frameworks, cobertura, performance
├── 📊 OutputFormat: 12 testes    # JSON, Markdown, HTML, estruturas
├── 💾 FileService: 11 testes     # Save/load, paths, encoding
└── 🎪 PromptGenerator: 18 testes # Preview, marcadores, consistência
```

#### **🔗 Testes de Integração (27 testes)**
```
🌐 Fluxos completos end-to-end:
├── 🎪 integration_prompt_generation.rs (10 testes)
│   ├── ✨ Workflow completo de geração
│   ├── 🔄 Preview com todas as seções
│   ├── 🧩 Interação entre módulos
│   ├── 🗂️ Dados parciais e vazios
│   ├── 📏 Conteúdo grande e especial
│   └── 🎯 Consistência de operações
├── 💾 integration_file_operations.rs (8 testes)
│   ├── 💿 Round-trip save/load
│   ├── 📁 Criação de diretórios
│   ├── 🎨 Formatação complexa preservada
│   ├── 📄 Múltiplos arquivos
│   ├── 🔄 Sobrescrita de arquivos
│   └── 📊 Arquivos grandes e especiais
└── 🔍 integration_parsing.rs (9 testes)
    ├── 🔄 Parse round-trip completo
    ├── 📋 Formato estruturado vs simples
    ├── 🧩 Seções parciais
    ├── 🛡️ Conteúdo malformado
    ├── 🎨 Caracteres especiais
    └── 🎯 Consistência regenerativa
```

### 🛠️ **Ferramentas de Qualidade Utilizadas:**

| Ferramenta | Função | Comando | Status |
|------------|--------|---------|--------|
| **🧪 Cargo Test** | Execução de testes | `cargo test` | ✅ 146 passing |
| **🎨 Cargo Fmt** | Formatação automática | `cargo fmt` | ✅ Applied |
| **🔍 Clippy** | Análise estática | `cargo clippy` | ✅ Clean |
| **✅ Cargo Check** | Verificação rápida | `cargo check` | ✅ Success |
| **📚 Cargo Doc** | Documentação | `cargo doc` | ✅ Generated |
| **🛡️ Cargo Audit** | Segurança deps | `cargo audit` | ✅ No vulnerabilities |

### 🎯 **Filosofia de Testes:**

1. **🔬 Test-Driven Development (TDD)**
   - Testes escritos junto com funcionalidades
   - Cobertura de casos extremos desde o início

2. **🏗️ Arquitetura Testável**
   - Separação clara models/services
   - Injeção de dependências via parâmetros
   - Mocks com arquivos temporários

3. **🎯 Testes Significativos**
   - Não apenas coverage, mas cenários reais
   - Edge cases documentados
   - Performance com arquivos grandes

4. **🔄 Integração Contínua**
   - Pipeline automatizada de qualidade
   - Regressão prevenida por design

---

## 🎨 **Interface & UX Design**

### 🎭 **Design System Moderno:**

```
🎨 Paleta de Cores Profissional:
├── 🔵 Azul Principal: #2563eb (botões, accent)
├── 🟢 Verde Sucesso: #16a34a (confirmações)
├── ⚪ Backgrounds: #f8fafc (clean, moderno)
├── 🔘 Texto: #1e293b (legibilidade perfeita)
└── 🎯 Hover: #1d4ed8 (interatividade clara)
```

### 📱 **Layout Responsivo:**

- **📐 Grid System**: 60% conteúdo + 40% preview
- **🎛️ Action Bar**: 70px fixo no bottom para fácil acesso
- **📜 Native Scroll**: TextEdit com scroll suave e fluido
- **🔘 5 Botões Principais**: Gerar, Salvar, Copiar, Limpar, Abrir

### 🌟 **Funcionalidades UX:**

| Feature | Descrição | Benefício |
|---------|-----------|-----------|
| **🔄 Preview Automático** | Atualiza enquanto digita | Feedback imediato |
| **📁 Parsing Inteligente** | Detecta formato automaticamente | Zero configuração |
| **🧠 Few-Shot Único** | Campo otimizado para parágrafo único | UX simplificada |
| **📋 Clean Preview** | Remove marcadores técnicos | Foco no conteúdo |
| **💾 Auto-save Context** | Mantém dados entre sessões | Produtividade |

---

## ⚙️ **Tecnologias & Dependências**

### 🦀 **Stack Principal:**

```toml
[dependencies]
slint = "1.8.0"          # 🎨 GUI Framework moderno
copypasta = "0.10"       # 📋 Clipboard cross-platform  
rfd = "0.15"            # 📁 Native file dialogs

[dev-dependencies]
tempfile = "3.8"        # 🧪 Testes com arquivos temporários
```

### 🏗️ **Arquitetura Técnica:**

- **🦀 Rust 2021 Edition**: Performance e segurança de memória
- **🎨 Slint 1.8**: Framework GUI declarativo e moderno
- **📁 Native Dialogs**: Integração perfeita com o OS
- **🔄 State Management**: Rc<RefCell<>> para compartilhamento
- **📦 Modular Design**: Separação clara de responsabilidades

### 🎯 **Padrões de Código:**

- **🏗️ Builder Pattern**: PromptData com build_prompt()
- **🔗 Service Layer**: Separação models/services
- **💾 File System Abstraction**: Testável e mockável
- **🎭 Strategy Pattern**: Múltiplos formatos de parsing
- **🧪 Dependency Injection**: Testabilidade máxima

---

## 🔮 **Roadmap & Features Futuras**

### 🚧 **Em Desenvolvimento:**

- [ ] 🌐 **Suporte Multi-idioma** (i18n)
- [ ] 🎨 **Temas Customizáveis** (Dark/Light mode)
- [ ] 📊 **Analytics de Prompts** (métricas de uso)
- [ ] 🔗 **Integração APIs** (OpenAI, Claude, etc.)
- [ ] 📱 **Versão Web** (WASM)

### 💡 **Ideias da Comunidade:**

- [ ] 🤖 **Templates Pré-definidos**
- [ ] 📈 **Versionamento de Prompts**
- [ ] 👥 **Colaboração em Tempo Real**
- [ ] 🎓 **Tutorial Interativo**
- [ ] 🔍 **Busca Global de Prompts**

---

## 👥 **Para Desenvolvedores**

### 🛠️ **Setup de Desenvolvimento:**

```bash
# Clone e setup
git clone https://github.com/consultorsandro/prompt_builder_gui.git
cd prompt_builder_gui

# Instale extensão VS Code para Slint
code --install-extension Slint.slint

# Execute em modo desenvolvimento
cargo run

# Rode todos os testes
cargo test

# Verificação completa de qualidade
cargo fmt && cargo clippy && cargo test && cargo doc
```

### 🧪 **Rodando Testes:**

```bash
# Todos os testes
cargo test

# Apenas unitários
cargo test --lib

# Apenas integração
cargo test --test integration_*

# Com output detalhado
cargo test -- --nocapture

# Testes específicos
cargo test test_parse_structured_format
```

### 📊 **Métricas de Código:**

```bash
# Linhas de código
find src -name "*.rs" | xargs wc -l

# Análise de dependências
cargo tree

# Auditoria de segurança
cargo audit

# Documentação local
cargo doc --open
```

---

## 🏆 **Créditos & Reconhecimentos**

### 👨‍💻 **Desenvolvido por:**
- **Sandro** - Arquitetura, Implementation & Testing
- **GitHub Copilot** - Code Review & Optimization

### 🙏 **Tecnologias que Tornaram Isso Possível:**
- **🦀 Rust Community** - Pela linguagem incrível
- **🎨 Slint Team** - Pelo framework GUI moderno
- **🧪 Rust Testing Ecosystem** - Pela robustez dos testes

### 🌟 **Inspirações:**
- **VS Code** - Design patterns para extensibilidade
- **GitHub** - Workflow de desenvolvimento
- **Discord** - UX design moderno e intuitivo

---

## 📄 **Licença**

```
MIT License

Copyright (c) 2025 Sandro - Prompt Builder GUI

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

<div align="center">

## 🚀 **Ready to Build Amazing Prompts?**

### [⬇️ Download Now](https://github.com/consultorsandro/prompt_builder_gui/releases) | [📖 Documentation](https://docs.rs/prompt-builder-gui) | [🐛 Report Issues](https://github.com/consultorsandro/prompt_builder_gui/issues)

**Feito com ❤️ em Rust | Powered by 🦀 + ⚡**

*"Transformando ideias em prompts perfeitos, um clique de cada vez"*

[![Star this repo](https://img.shields.io/github/stars/consultorsandro/prompt_builder_gui?style=social)](https://github.com/consultorsandro/prompt_builder_gui)

</div>
