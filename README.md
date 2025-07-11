# Prompt Builder GUI

Uma aplicação desktop em Rust para construção de prompts estruturados para IA, usando [Slint](https://slint.rs/) como framework para interface gráfica.

## Sobre o Projeto

Este projeto ajuda você a criar prompts bem estruturados para interações com IA, organizando diferentes seções como:

- **Few-Shot**: Exemplos de entrada e saída esperada
- **Context**: Contexto ou persona da IA
- **Main Content**: Instruções principais
- **Auxiliary Content**: Conteúdo auxiliar e exemplos
- **Limitations**: Limitações e restrições
- **Refactoring**: Instruções de refatoração (para código)
- **Guidance**: Orientações de tom e estilo
- **Tests**: Requisitos de teste
- **Output Format**: Formato de saída desejado

## Estrutura do Projeto

```
src/
├── main.rs              # Ponto de entrada da aplicação
├── models/              # Modelos de dados para seções do prompt
│   ├── few_shot.rs
│   ├── context.rs
│   ├── main_content.rs
│   ├── auxiliary_content.rs
│   ├── limitations.rs
│   ├── refactoring.rs
│   ├── guidance.rs
│   ├── tests.rs
│   ├── output_format.rs
│   └── mod.rs
└── services/            # Serviços para processamento e arquivos
    ├── prompt_generator.rs  # Geração de prompts
    ├── file_service.rs      # Salvamento de arquivos
    └── mod.rs
ui/
└── app-window.slint     # Interface gráfica
```

## Como Usar

1. Instale o Rust seguindo o [guia oficial](https://www.rust-lang.org/learn/get-started)
2. Clone este repositório
3. Execute o projeto:
   ```bash
   cargo run
   ```

## Desenvolvimento

Para desenvolver este projeto, recomendamos usar VS Code com a [extensão Slint](https://marketplace.visualstudio.com/items?itemName=Slint.slint) para suporte a arquivos `.slint`.

## Próximos Passos

- [ ] Conectar interface gráfica com a lógica Rust
- [ ] Implementar callbacks da UI
- [ ] Adicionar funcionalidade de preview em tempo real
- [ ] Implementar salvamento e carregamento de prompts
- [ ] Adicionar mais seções personalizáveis
- [ ] Melhorar o design da interface

## Licença

MIT License - veja o arquivo [LICENSE](LICENSE) para detalhes.
