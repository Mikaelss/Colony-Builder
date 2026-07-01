# AGENTS.md

Projeto: jogo 2D colony builder em Rust. Atualmente é apenas um esqueleto (`src/main.rs` de 3 linhas, sem dependências).

## Setup

- **Rust edition 2024** — requer Rust ≥ 1.85 (`rustc 1.96` em uso). Não há `rust-toolchain.toml`.
- **Dependências**: `bevy = "0.19"` adicionado. Consulte `Cargo.toml` para a lista completa.
- **Git**: regras detalhadas de branch, commit e merge em `git_repository_rules.md`. Leia antes de commitar.

## Comandos

```bash
cargo check              # compila sem gerar binário
cargo fmt --check        # formatação obrigatória antes de merge
cargo clippy             # lint (rodar antes de merge)
cargo test               # testes (ainda não existem)
```

Ordem pré-merge: `cargo fmt --check && cargo check && cargo test && cargo clippy`

## Arquitetura (contratos vigentes)

- **Simulação dirigida por ticks fixos** — independente de FPS. Sistemas de gameplay não dependem de frame time.
- **Separação rígida simulação/apresentação** — simulação é serializável, testável sem render. Apresentação apenas lê estado.
- **Sem centralizadores** — proibido criar `GameManager`, `WorldManager`, `ColonyManager` ou equivalentes.
- **Comunicação por eventos** — sistemas não chamam lógica interna de outros domínios. Sem dependências circulares.
- **Componentes pequenos e focados** — preferir vários componentes específicos a um genérico grande.
- **IDs lógicos persistentes** — não usar `Entity` ID da ECS como identidade de gameplay. Usar IDs estáveis para save/load/referências.
- **Conteúdo em definições** — evitar hardcode de gameplay em sistemas centrais. Definições com IDs estáveis e validadas.
- **Input não altera simulação diretamente** — input/UI cria intenção → sistema valida → simulação muda → evento → UI reage.
- **Validar antes de codificar** — não escrever código sem antes verificar se a mudança respeita os contratos arquiteturais vigentes. O agente atua como auxiliar de estrutura, não como gerador cego de código.
- **Mudanças arquiteturais documentadas** — alterações em contratos entre sistemas, schedules, organização de plugins ou direção de dependências exigem atualização deste arquivo e/ou criação de ADR em `docs/adr/`.
- **Core primeiro** — não implementar domínios específicos de gameplay (colonos, jobs, produção, construção) antes da fundação core: tick, schedule, registro de definições, estrutura de mundo.

## Idioma

Documentação arquitetural em **português**. Termos técnicos (ECS, Plugin, Component, Tick, Schedule) mantidos em inglês.

## Fluxo de trabalho

Consulte `TASK_FLOW.md` para o fluxo obrigatório de resolução de tarefas.

## Outros arquivos importantes

- `git_repository_rules.md` — branch naming (`tipo/descricao-curta`), commit format (`tipo(escopo): msg`), merge policy, checklist pré-merge.
