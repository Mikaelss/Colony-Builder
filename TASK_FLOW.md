# Fluxo de Resolução de Tarefas

Este documento define o fluxo obrigatório para agentes ao receberem uma tarefa neste projeto.

## Papéis do Time

Considere estas responsabilidades em toda resposta (sem necessidade de personificação explícita):

- **Arquiteto** — validar contratos, direção de dependências, isolamento entre domínios.
- **Tech Lead Rust/Bevy** — schedule, plugins, ECS idiomático, compatibilidade com a engine.
- **Gameplay Designer** — fluxo de simulação, fases de tick, definições de conteúdo.
- **QA/Test Engineer** — testabilidade, cobertura, comandos de validação.
- **DevOps/Git** — branch, commits, merge, CI.
- **Documentation** — atualização de `AGENTS.md`, ADRs, arquivos de referência.

---

## Fluxo Obrigatório de Resolução

Para cada tarefa, percorra as etapas abaixo. Só implemente após a etapa 8.

### 1. Entendimento da Tarefa

Explique em poucas linhas: objetivo, parte do projeto afetada, categoria (core, simulação, apresentação, tooling, docs, gameplay), dependências de fundação ausentes.

Não implemente nada nesta etapa.

### 2. Classificação

```text
Categoria: [core|time|schedule|state|events|identity|definitions|world|simulation|presentation|ui|debug|save|tooling|docs|refactor|test]
Branch sugerida: [tipo/descricao-curta]
Commit principal esperado: [tipo(escopo): mensagem]
```

### 3. Análise Arquitetural

Antes de propor código, verifique contratos do `AGENTS.md`:

- A tarefa respeita os contratos arquiteturais vigentes?
- Quais sistemas ou módulos são afetados?
- Risco de acoplamento indevido? Risco de manager centralizador?
- Deve rodar por frame, por tick ou sob demanda?
- Exige evento, recurso, componente, plugin ou schedule?
- Afeta save/load? Afeta modabilidade futura?
- Exige atualização de documentação?

Se houver ambiguidade, faça perguntas objetivas antes de continuar.

### 4. Decisão de Escopo

```text
Incluído:
- ...

Fora do escopo:
- ...
```

Evite expandir a tarefa além do necessário. Se for grande demais, divida em subtarefas e recomende ordem.

### 5. Plano Técnico

Sem escrever código. Inclua:

- Arquivos prováveis (criar ou alterar)
- Módulos e plugins envolvidos
- Componentes, recursos, eventos conceituais
- Fases de schedule
- Dependências permitidas e proibidas
- Riscos técnicos
- Critérios de aceite

### 6. Fluxo ECS

```text
Entrada:
Processamento:
Saída:
Eventos emitidos:
Eventos consumidos:
Frequência: [Frame | Tick | Sob demanda]
```

Se não envolver ECS, explique por quê.

### 7. Estratégia de Testes

```text
Obrigatórios: cargo check, cargo fmt --check, cargo clippy, cargo test
Recomendados: [testes unitários, integração, comportamento, validação manual]
```

### 8. Estratégia de Git

```text
Branch:
Commits sugeridos:
Checklist pré-merge:
- cargo fmt --check
- cargo check
- cargo test
- cargo clippy
- documentação atualizada se necessário
```

### 9. Execução

Implemente mudanças pequenas e focadas. Respeite o escopo definido. Não misture refactor amplo com feature. Não introduza abstrações prematuras, dependências circulares, ou `Entity` como identidade persistente.

### 10. Revisão Interna

Verifique:

- **Arquitetura**: contratos do `AGENTS.md` mantidos? Separação sim/apresentação? Sem centralizadores? Eventos/contratos usados? Componentes pequenos?
- **Rust/Bevy**: schedule explícito? Plugins com responsabilidade clara? Recursos e componentes coerentes?
- **Performance**: sem sistemas globais desnecessários? Lógica de simulação não roda por frame?
- **Testes**: validação suficiente? Passa em check/fmt/clippy/test?
- **Git**: branch e commits seguem as regras? Mudança pequena e revisável?
- **Documentação**: `AGENTS.md`, ADR ou README precisam de atualização?

### 11. Resultado Final

```text
Resumo:
Arquivos alterados/criados:
Comandos para validar:
Commits sugeridos:
Próximo passo recomendado:
```

Se bloqueado:

```text
Bloqueios:
Perguntas para desbloquear:
```

---

## Comportamento Esperado

- Pense como arquiteto antes de codar.
- Planeje como tech lead.
- Valide como QA.
- Organize como maintainer.
- Documente como colaborador de longo prazo.

Não implemente sem plano. Não resolva em um commit gigante. Não ignore `AGENTS.md` ou `git_repository_rules.md`. Não misture simulação e apresentação. Se a tarefa for ambígua ou arriscada, primeiro proponha decomposição segura e pergunte.
