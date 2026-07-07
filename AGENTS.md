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
cargo test               # 91 testes (unit + integration)
```

Ordem pré-merge: `cargo fmt --check && cargo check && cargo test && cargo clippy`

## Arquitetura (contratos vigentes)

- **Simulação dirigida por ticks fixos** — independente de FPS. Sistemas de gameplay não dependem de frame time.
- **Separação rígida simulação/apresentação** — simulação é serializável, testável sem render. Apresentação apenas lê estado.
- **Sem centralizadores** — proibido criar `GameManager`, `WorldManager`, `ColonyManager` ou equivalentes.
- **Comunicação por eventos** — sistemas não chamam lógica interna de outros domínios. Sem dependências circulares.
- **Componentes pequenos e focados** — preferir vários componentes específicos a um genérico grande.
- **IDs lógicos persistentes** — não usar `Entity` ID da ECS como identidade de gameplay. Usar `Id<T>` (`core::identity`) para save/load/referências. Cada domínio define seu próprio marker type (ex: `Colonist`, `Building`) e usa `type ColonistId = Id<Colonist>`.
- **EntityMap por domínio** — cada domínio mantém seu próprio `EntityMap<T>` para traduzir `Id<T>` → `Entity`. Não existe mapeamento centralizado.
- **Alocação de IDs** — cada domínio usa `IdAllocator<T>` como recurso próprio. IDs são sequenciais dentro de uma gameplay.
- **Conteúdo em definições** — evitar hardcode de gameplay em sistemas centrais. Definições com IDs estáveis e validadas. Carregar de `assets/definitions/` via `ItemRegistry` (e similares futuros). Registry valida duplicatas em loading.
- **Input não altera simulação diretamente** — input/UI cria intenção → sistema valida → simulação muda → evento → UI reage.
- **Validar antes de codificar** — não escrever código sem antes verificar se a mudança respeita os contratos arquiteturais vigentes. O agente atua como auxiliar de estrutura, não como gerador cego de código.
- **Mudanças arquiteturais documentadas** — alterações em contratos entre sistemas, schedules, organização de plugins ou direção de dependências exigem atualização deste arquivo e/ou criação de ADR em `docs/adr/`.
- **Core primeiro** — não implementar domínios específicos de gameplay (colonos, jobs, produção, construção) antes da fundação core: tick, schedule, registro de definições, estrutura de mundo.
- **Eventos para consumo imediato** — eventos (`TickEvent`, etc.) carregam dados prontos para consumo (ex: `tick`, `day`, `tick_of_day`). Nenhum sistema deve extrair campos de um evento para armazenamento persistente.
- **Tick bruto como verdade (`u64`)** — ticks são a unidade de persistência. Dias e horas são derivados de `tick * constantes`. Constantes (`TICKS_PER_DAY`, etc.) são a régua de conversão para exibição. Se a régua mudar, o tick continua válido.
- **Coesão de domínio** — tick gerencia dia (grupo de ticks). Marcos derivados (amanhecer, hora cheia) são detectados por `tick_of_day % X`. Se um marco for usado por ≥ 2 sistemas distintos, vira evento próprio no módulo que define a divisão.
- **Debug isolado e descartável** — `debug/` nunca é importado por nenhum outro módulo. Todo terminal output passa por `DebugSettings` (`dev_mode` desligado = zero output). Remover `DebugPlugin` de `main.rs` não afeta gameplay.
- **Sistemas sob Playing** — todo sistema de gameplay, debug, save, e input de jogo deve usar `.run_if(in_state(GameState::Playing))`. Sistemas que não respeitam isso podem crashar durante Loading por falta de recursos. Exceção: sistemas de loading, startup, ou transição de estado.
- **TDD** — testes primeiro. `cargo test` antes de merge. Testes unitários em `tests/*.rs`. Testes de integração com Bevy `App` mínimo em `tests/app.rs`.

## Idioma

Documentação arquitetural em **português**. Termos técnicos (ECS, Plugin, Component, Tick, Schedule) mantidos em inglês.

## Fluxo de trabalho

Consulte `TASK_FLOW.md` para o fluxo obrigatório de resolução de tarefas.

## Outros arquivos importantes

- `git_repository_rules.md` — branch naming (`tipo/descricao-curta`), commit format (`tipo(escopo): msg`), merge policy, checklist pré-merge.
