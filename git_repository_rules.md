# Regras de Repositório Git

## Objetivo

Este repositório deve manter um histórico limpo, rastreável e seguro para a evolução de longo prazo do jogo.

As regras de Git existem para garantir:

- Clareza no histórico.
- Facilidade de revisão.
- Separação entre mudanças arquiteturais e mudanças de gameplay.
- Segurança contra commits acidentais de arquivos gerados.
- Facilidade para voltar versões.
- Organização para futuras contribuições e mods.
- Estabilidade da branch principal.

---

## Branch Principal

A branch principal do projeto deve ser:

```text
master
```

A branch `master` deve representar sempre um estado estável do projeto.

Regra principal:

```text
master sempre deve compilar.
```

Não devem ser feitos commits diretos na `master`, exceto em casos pequenos e controlados no início do projeto.

Antes de integrar mudanças relevantes, sempre que possível executar:

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy
```

---

## Branches de Trabalho

Toda mudança relevante deve ser feita em uma branch separada.

Formato recomendado:

```text
tipo/descricao-curta
```

Exemplos:

```text
core/tick-foundation
core/plugin-architecture
core/game-states
core/definition-registry
world/grid-foundation
debug/tick-inspector
refactor/module-boundaries
docs/update-agents-rules
```

Tipos recomendados:

```text
core/
world/
sim/
render/
ui/
debug/
docs/
refactor/
fix/
test/
tooling/
prototype/
```

### Significado dos tipos

#### `core/`

Mudanças na fundação do projeto.

Exemplos:

- arquitetura de plugins
- schedules
- estados globais
- tick simulation
- registry base
- identidade persistente

#### `world/`

Mudanças relacionadas à estrutura do mundo.

Exemplos:

- grid
- chunks
- tiles
- coordenadas
- ocupação de mapa

#### `sim/`

Mudanças em sistemas de simulação.

Exemplos:

- jobs
- colonos
- necessidades
- produção
- construção
- recursos

#### `render/`

Mudanças de apresentação visual.

Exemplos:

- sprites
- tilemap rendering
- câmera
- animações
- sincronização visual

#### `ui/`

Mudanças de interface e input do jogador.

Exemplos:

- menus
- seleção
- comandos
- HUD
- overlays

#### `debug/`

Ferramentas internas de desenvolvimento.

Exemplos:

- painéis de debug
- logs estruturados
- visualização de ticks
- visualização de entidades

#### `docs/`

Documentação.

Exemplos:

- `README.md`
- `AGENTS.md`
- decisões arquiteturais
- guias internos

#### `refactor/`

Mudanças internas que não deveriam alterar comportamento esperado.

#### `fix/`

Correções de bug.

#### `test/`

Adição ou ajuste de testes.

#### `tooling/`

Ferramentas de desenvolvimento.

Exemplos:

- CI
- scripts
- configuração de lint
- configuração de formatação

#### `prototype/`

Explorações temporárias ou provas de conceito.

Código de protótipo não deve entrar silenciosamente na `master` como código final.

---

## Commits

Commits devem ser pequenos, claros e focados.

Evitar commits gigantes misturando várias intenções.

Cada commit deve responder:

```text
O que mudou?
Por que mudou?
Qual parte do projeto foi afetada?
```

---

## Convenção de Mensagens de Commit

Usar uma variação simples de Conventional Commits.

Formato:

```text
tipo(escopo): descrição curta
```

Exemplos:

```text
docs(agents): add repository git rules
core(time): add tick simulation foundation
core(schedule): define simulation phases
world(grid): add initial tile coordinate model
debug(tick): add tick counter diagnostics
refactor(core): split plugin registration
fix(world): correct tile bounds validation
test(time): add tick rate validation tests
```

### Tipos permitidos

```text
feat
fix
docs
refactor
test
chore
perf
debug
tooling
```

### Quando usar cada tipo

#### `feat`

Nova funcionalidade.

```text
feat(core): add game state setup
```

#### `fix`

Correção de bug.

```text
fix(world): prevent invalid tile lookup
```

#### `docs`

Mudança apenas em documentação.

```text
docs(agents): clarify ECS boundaries
```

#### `refactor`

Mudança estrutural sem alterar comportamento esperado.

```text
refactor(core): separate simulation schedules
```

#### `test`

Adição ou alteração de testes.

```text
test(time): validate fixed tick accumulation
```

#### `chore`

Tarefas auxiliares sem impacto direto na arquitetura ou gameplay.

```text
chore(repo): update gitignore
```

#### `perf`

Mudança focada em performance.

```text
perf(world): reduce tile lookup allocations
```

#### `debug`

Ferramentas ou informações de debug.

```text
debug(core): expose active simulation phase
```

#### `tooling`

Mudanças em ferramentas de desenvolvimento.

```text
tooling(ci): add cargo check workflow
```

---

## Escopos de Commit

Escopos devem representar o domínio afetado.

Escopos recomendados:

```text
repo
core
time
schedule
state
events
identity
definitions
world
grid
chunks
sim
render
ui
debug
save
tests
docs
tooling
```

Evitar escopos vagos como:

```text
misc
stuff
general
manager
logic
```

---

## Commits Atômicos

Um commit deve conter uma mudança lógica.

Bom:

```text
feat(core): add simulation tick resource
```

Bom:

```text
docs(repo): document branch naming rules
```

Ruim:

```text
feat: add ticks, map, rendering, jobs and fix docs
```

Se uma mudança envolve várias áreas, separar em commits diferentes.

Exemplo:

```text
feat(core): add simulation tick resource
feat(core): add fixed tick schedule phases
debug(time): add tick counter logging
docs(time): document tick execution model
```

---

## Pull Requests / Merge Requests

Mesmo que o projeto seja individual no início, cada branch deve representar uma unidade revisável.

Antes de mergear:

- A branch deve compilar.
- O escopo deve estar claro.
- Mudanças temporárias devem ser removidas.
- Logs de debug excessivos devem ser removidos ou protegidos por configuração.
- O código deve estar formatado.
- Arquivos gerados indevidos não devem estar no commit.
- Documentação deve ser atualizada se a arquitetura mudou.

Checklist recomendado:

```text
- [ ] A mudança respeita isolamento entre sistemas.
- [ ] A simulação continua separada da apresentação.
- [ ] Não foi criado manager centralizador.
- [ ] Não há dependência circular nova.
- [ ] Não há hardcode desnecessário de conteúdo.
- [ ] IDs ECS não foram usados como identidade persistente.
- [ ] O projeto compila com cargo check.
- [ ] O código foi formatado com cargo fmt.
- [ ] Clippy foi considerado.
- [ ] Testes relevantes foram adicionados ou atualizados.
- [ ] Documentação foi atualizada quando necessário.
```

---

## Política de Merge

Enquanto o projeto estiver no início, pode ser usado merge simples.

Recomendação para médio prazo:

```text
Squash merge para branches pequenas e focadas.
Merge commit para branches grandes de arquitetura.
```

### Squash merge

Usar quando:

- A branch tem commits intermediários pequenos.
- O histórico individual não é importante.
- A mudança representa uma unidade lógica única.

### Merge commit

Usar quando:

- A branch representa uma fase arquitetural importante.
- Os commits internos contam uma história útil.
- A sequência de mudanças deve ser preservada.

### Rebase

Rebase é aceitável em branches locais antes do merge.

Evitar rebase em branches compartilhadas.

---

## Tags e Versões

Tags devem ser usadas para marcos importantes.

Formato recomendado:

```text
v0.1.0
v0.2.0
v0.3.0
```

Enquanto o jogo estiver em fundação, também podem existir tags de milestone:

```text
foundation-0
foundation-1
prototype-0
prototype-1
```

Recomendação inicial:

```text
v0.1.0 = fundação core mínima
v0.2.0 = mapa 2D mínimo
v0.3.0 = entidade móvel com pathfinding
v0.4.0 = jobs básicos
v0.5.0 = estoque e itens
v0.6.0 = construção básica
```

---

## Arquivos que Não Devem Ser Commitados

Não commitar:

```text
target/
.DS_Store
*.log
*.tmp
*.bak
*.swp
*.swo
.env
.env.*
.idea/
```

Exceção:

Configurações úteis e compartilháveis de editor podem ser commitadas se forem intencionais.

Exemplo aceitável:

```text
.vscode/extensions.json
.vscode/settings.json
```

Desde que não contenham caminhos pessoais ou configurações privadas.

---

## Arquivos Gerados

Arquivos gerados devem ser evitados no Git, salvo quando forem parte intencional do projeto.

Normalmente não commitar:

- builds compilados
- caches
- logs
- artefatos temporários
- arquivos exportados automaticamente
- dados locais de debug
- saves locais

Pode commitar, se fizer sentido:

- assets fonte
- dados vanilla
- arquivos de configuração
- documentação revisada
- snapshots de teste explicitamente necessários

---

## Cargo.lock

Para este projeto, `Cargo.lock` deve ser commitado.

Motivo:

Este é um jogo/aplicação, não apenas uma biblioteca Rust.

Manter `Cargo.lock` no repositório ajuda a garantir builds reproduzíveis.

---

## `.gitignore` Recomendado

Usar um `.gitignore` inicial semelhante a:

```gitignore
# Rust
/target/

# Logs
*.log

# Temporary files
*.tmp
*.bak
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Environment files
.env
.env.*

# Local saves/debug data
/saves/
/debug_output/
/profiling/

# IDEs
.idea/

# VS Code
.vscode/*
!.vscode/extensions.json
!.vscode/settings.json

# Local cache
/.cache/
```

Se o projeto decidir manter saves de exemplo ou profiling controlado, criar pastas específicas como:

```text
examples/saves/
benchmarks/data/
```

e não usar as pastas locais ignoradas.

---

## Branches Protegidas

Quando o projeto estiver mais maduro, proteger a branch `master`.

Regras recomendadas:

- Bloquear push direto.
- Exigir pull request.
- Exigir `cargo check`.
- Exigir `cargo fmt --check`.
- Exigir testes relevantes.
- Exigir revisão para mudanças grandes.

No início, isso pode ser manual.

---

## Fluxo de Trabalho Recomendado

Para nova tarefa:

```bash
git checkout master
git pull
git checkout -b core/tick-foundation
```

Durante desenvolvimento:

```bash
cargo fmt
cargo check
cargo test
```

Antes do merge:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy
```

Finalizar com commits claros:

```bash
git add .
git commit -m "feat(core): add tick simulation foundation"
```

Mergear quando estiver estável.

---

## Regra para Mudanças Arquiteturais

Mudanças arquiteturais devem ser feitas com cuidado.

Uma mudança é arquitetural quando altera:

- organização de plugins
- schedules
- fases de tick
- contratos entre sistemas
- formato de definições
- identidade persistente
- save/load
- fronteiras entre módulos
- direção de dependências

Para essas mudanças, preferir:

- branch própria
- commit claro
- documentação curta da decisão
- atualização do `AGENTS.md`, se afetar regra geral
- atualização de ADR, se o projeto usar Architecture Decision Records

---

## Architecture Decision Records

Para decisões importantes, usar ADRs.

Local recomendado:

```text
docs/adr/
```

Formato de arquivo:

```text
0001-use-fixed-tick-simulation.md
0002-separate-simulation-and-presentation.md
0003-use-definition-registry-for-content.md
```

Cada ADR deve conter:

```text
# Título

## Contexto

## Decisão

## Consequências
```

ADRs devem ser curtos.

Não usar ADR para toda pequena decisão.

Usar ADR para decisões difíceis de reverter.

---

## Commits de Documentação

Documentação deve ser tratada como parte real do projeto.

Mudanças em arquitetura devem atualizar documentação no mesmo branch.

Exemplos:

```text
docs(agents): add repository rules
docs(adr): document fixed tick simulation
docs(core): describe plugin boundaries
```

---

## Commits de Protótipo

Protótipos são permitidos, mas devem ser claramente marcados.

Branches de protótipo podem usar:

```text
prototype/world-grid
prototype/pathfinding-test
prototype/tile-rendering
```

Código de protótipo não deve ser misturado silenciosamente com código final.

Antes de mergear na `master`, o protótipo deve ser:

- removido
- isolado em exemplo
- transformado em implementação limpa
- documentado como temporário

---

## Commits Temporários

Evitar commits como:

```text
wip
teste
arrumando
coisas
final
final2
```

Se forem usados localmente, devem ser ajustados antes do merge por squash ou rebase.

Mensagens aceitáveis apenas localmente:

```text
wip: testing tick phases
wip: exploring grid structure
```

Mas não devem permanecer no histórico principal.

---

## Regra para Assets

Assets devem seguir organização clara.

Evitar adicionar assets sem contexto.

Cada asset commitado deve ter finalidade conhecida.

Não commitar assets pesados desnecessariamente.

Se o projeto crescer muito em assets, avaliar uso de Git LFS.

Possíveis candidatos a Git LFS no futuro:

- imagens grandes
- áudio
- vídeos
- arquivos fonte de arte muito pesados

Não usar Git LFS prematuramente sem necessidade.

---

## Segurança

Nunca commitar:

- tokens
- chaves privadas
- credenciais
- arquivos `.env`
- dados pessoais
- paths locais sensíveis
- configurações privadas de máquina

Se algo sensível for commitado por acidente, deve ser removido do histórico adequadamente.

---

## Padrão de Revisão

Ao revisar uma mudança, verificar principalmente:

```text
A mudança preserva a arquitetura ECS?
A mudança mantém sistemas isolados?
A mudança respeita tick simulation?
A mudança evita acoplamento com UI/render?
A mudança prepara ou preserva save/load?
A mudança evita hardcode desnecessário?
A mudança mantém performance razoável?
A mudança é pequena o suficiente para entender?
```

---

## Regra de Ouro do Repositório

A branch `master` deve contar a história limpa da evolução do projeto.

Cada commit deve aproximar o jogo de uma base mais estável, modular e testável.

Se uma mudança bagunça o histórico, mistura responsabilidades ou quebra isolamento arquitetural, ela deve ser separada antes do merge.
