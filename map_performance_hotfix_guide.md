# Guia de Hotfix de Performance do Mapa

Projeto: jogo 2D colony builder em Rust com Bevy.  
Contexto: o mapa usa tiles de textura `64x64 px` e atualmente sofre lag quando a câmera está muito afastada, pois muitos tiles ficam visíveis ao mesmo tempo.

Este guia define **3 tarefas isoladas** para estabilizar o projeto antes de uma refatoração maior de renderização por chunks.

## Problema

O modelo atual provavelmente segue a lógica:

```text
1 tile lógico = 1 sprite/entity visual
```

Isso funciona com poucos tiles visíveis, mas não escala quando a câmera mostra grande parte do mapa.

Exemplo:

```text
Mapa: ~75.625 tiles
Zoom distante: grande parte ou todo o mapa visível
Resultado: muitas entidades/sprites processadas por frame
```

## Objetivo deste guia

Corrigir o problema de forma segura em etapas pequenas:

```text
1. Limitar zoom máximo
2. Adicionar métricas de renderização
3. Separar map view da simulação
```

Essas tarefas devem ser feitas em branches separadas e commits isolados.

## Fora do escopo deste guia

Não implementar ainda:

- chunk mesh
- LOD
- geração procedural infinita
- save/load por chunk
- pathfinding por chunk
- refatoração completa do mapa
- mudança estrutural da simulação

A prioridade é estabilizar e preparar o terreno para otimizações futuras.

## Tarefa 1 — Limitar zoom máximo da câmera

### Objetivo

Reduzir o lag imediatamente impedindo que a câmera consiga visualizar o mapa inteiro de uma vez.

### Categoria

```text
presentation / camera / fix
```

### Branch sugerida

```text
fix/limit-camera-zoom
```

### Commit sugerido

```text
fix(camera): limit max zoom to avoid full map rendering
```

### Escopo

Incluído:

- Alterar apenas a lógica de zoom da câmera.
- Reduzir o valor máximo de zoom/scale.
- Impedir que o mapa inteiro fique visível.
- Manter movimentação da câmera funcionando.
- Manter o comportamento atual de renderização dos tiles.

Fora do escopo:

- Não alterar o grid lógico.
- Não alterar geração de mapa.
- Não alterar tiles.
- Não alterar simulação.
- Não criar chunks.
- Não criar LOD.
- Não mexer em save/load.
- Não mexer em pathfinding.

### Valor recomendado

Se existir algo como:

```text
MAX_SCALE = 20.0
```

Reduzir para algo próximo de:

```text
MAX_SCALE = 5.0
```

Se `5.0` ainda permitir ver uma área grande demais, testar:

```text
MAX_SCALE = 4.0
```

### Justificativa

Com tiles de `64x64 px`, a quantidade estimada de tiles visíveis cresce com o zoom.

Em uma janela de `1920x1080`:

```text
scale 1.0  ≈ 30 x 17 tiles  ≈ 510 tiles visíveis
scale 5.0  ≈ 150 x 84 tiles ≈ 12.600 tiles visíveis
scale 20.0 ≈ 600 x 337 tiles, suficiente para cobrir mapas grandes inteiros
```

Reduzir o zoom máximo não resolve a arquitetura final, mas evita o pior caso imediatamente.

### Contratos arquiteturais

A tarefa deve respeitar:

- Simulação independente de FPS.
- Separação entre simulação e apresentação.
- Input/câmera não altera estado lógico do mundo.
- Nenhum manager centralizador deve ser criado.
- Nenhuma regra de gameplay deve ser alterada.

### Critérios de aceite

- O zoom máximo não permite ver o mapa inteiro.
- O lag no zoom máximo é reduzido.
- A câmera continua movendo corretamente.
- A simulação permanece inalterada.
- Nenhum sistema de gameplay é modificado.
- `cargo fmt --check` passa.
- `cargo check` passa.
- `cargo test` passa.
- `cargo clippy` passa.

### Checklist de execução

```bash
git checkout -b fix/limit-camera-zoom
```

Após alterar:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy
```

Commit:

```bash
git add .
git commit -m "fix(camera): limit max zoom to avoid full map rendering"
```

## Tarefa 2 — Adicionar métricas simples de renderização

### Objetivo

Criar visibilidade mínima sobre o problema de performance antes de refatorações maiores.

Esta tarefa serve para medir:

- zoom atual
- quantidade estimada de tiles visíveis
- número aproximado de entidades visuais de tiles, se viável
- posição da câmera
- FPS ou frame time, se já houver estrutura disponível

### Categoria

```text
debug / render / tooling
```

### Branch sugerida

```text
debug/render-metrics
```

### Commit sugerido

```text
debug(render): add tile rendering metrics
```

### Escopo

Incluído:

- Adicionar métricas simples para depuração.
- Mostrar ou logar zoom atual da câmera.
- Estimar quantidade de tiles visíveis com base em:
  - tamanho da janela
  - zoom/scale da câmera
  - tamanho do tile `64x64`
- Se simples, exibir FPS/frame time.
- Se simples, contar entidades visuais de tiles.

Fora do escopo:

- Não otimizar renderização ainda.
- Não criar chunking.
- Não criar LOD.
- Não trocar sprites por mesh.
- Não alterar simulação.
- Não criar sistema complexo de telemetry.
- Não criar UI final de debug se ainda não existir necessidade.

### Fórmula base

Considerando:

```text
TILE_SIZE = 64.0
```

Estimativa:

```text
tiles_visiveis_x = largura_da_janela * camera_scale / TILE_SIZE
tiles_visiveis_y = altura_da_janela * camera_scale / TILE_SIZE
tiles_visiveis_total = tiles_visiveis_x * tiles_visiveis_y
```

Exemplo:

```text
1920x1080, scale 5.0:

tiles_visiveis_x = 1920 * 5 / 64 ≈ 150
tiles_visiveis_y = 1080 * 5 / 64 ≈ 84
tiles_visiveis_total ≈ 12.600
```

### Forma aceitável de saída

Qualquer uma das opções abaixo é aceitável para esta fase:

```text
1. Log no console
2. Texto debug simples na tela
3. Bevy diagnostics, se já estiver configurado
```

Preferir a opção mais simples e menos invasiva.

### Contratos arquiteturais

A tarefa deve respeitar:

- Debug não altera simulação.
- Debug não deve introduzir dependência de gameplay.
- Debug deve poder ser removido ou desativado facilmente.
- Não criar centralizadores.
- Não misturar lógica visual com regras do mundo.

### Critérios de aceite

- É possível observar o zoom atual.
- É possível estimar tiles visíveis.
- As métricas ajudam a comparar antes/depois.
- O sistema não altera gameplay.
- O sistema não altera dados do mapa.
- `cargo fmt --check` passa.
- `cargo check` passa.
- `cargo test` passa.
- `cargo clippy` passa.

### Checklist de execução

```bash
git checkout -b debug/render-metrics
```

Após alterar:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy
```

Commit:

```bash
git add .
git commit -m "debug(render): add tile rendering metrics"
```

## Tarefa 3 — Separar map view da simulação

### Objetivo

Preparar o projeto para futuras otimizações de renderização sem alterar a lógica do mundo.

A ideia é deixar claro que:

```text
Tile lógico != representação visual do tile
```

A simulação deve possuir os dados do mapa.  
A apresentação deve decidir como desenhar esses dados.

### Categoria

```text
render / presentation / refactor
```

### Branch sugerida

```text
render/separate-map-view
```

### Commit sugerido

```text
refactor(render): separate logical tiles from map presentation
```

### Escopo

Incluído:

- Isolar código de visualização do mapa em uma área de apresentação.
- Manter dados lógicos do mapa fora da camada de render.
- Garantir que sistemas de apresentação apenas leiam o estado do mundo.
- Preparar a estrutura para chunking visual futuro.
- Manter o visual atual funcionando como antes.

Fora do escopo:

- Não implementar chunk mesh.
- Não implementar chunks visuais ainda, exceto se for apenas nome/estrutura mínima sem comportamento.
- Não criar LOD.
- Não alterar formato lógico dos tiles.
- Não alterar regras de simulação.
- Não reescrever o sistema de mapa inteiro.
- Não mexer em save/load.
- Não mexer em pathfinding.

### Organização sugerida

A estrutura exata depende do projeto atual, mas a separação conceitual deve ficar parecida com:

```text
src/
  world/
    mod.rs
    tile.rs
    map.rs

  presentation/
    mod.rs
    camera.rs
    map_view/
      mod.rs
      tile_view.rs
```

Ou, se a estrutura atual já for diferente, adaptar mantendo a ideia:

```text
world = dados e regras lógicas
presentation/map_view = representação visual do mapa
```

### Regra principal

A simulação não deve depender de:

- `Sprite`
- câmera
- material visual
- textura
- atlas visual
- input de câmera
- componentes apenas de renderização

A apresentação pode ler:

- mapa lógico
- coordenadas dos tiles
- tipo de terreno
- estado necessário para desenhar

Mas não deve modificar regras de gameplay.

### Estado esperado após esta tarefa

Ainda pode existir:

```text
1 tile = 1 sprite visual
```

Isso é aceitável nesta tarefa.

O objetivo aqui não é resolver tudo, mas preparar a troca futura para:

```text
1 chunk = 1 mesh
```

### Contratos arquiteturais

A tarefa deve respeitar:

- Separação rígida entre simulação e apresentação.
- Simulação serializável e testável sem render.
- Apresentação apenas lê estado.
- Nenhum manager centralizador.
- Sem dependências circulares.
- Sem mudança de comportamento de gameplay.
- Sem mudança estrutural desnecessária.

### Critérios de aceite

- O código de visualização do mapa fica isolado na camada de apresentação.
- O mundo lógico não depende de tipos visuais da Bevy.
- O visual atual do mapa continua funcionando.
- A tarefa prepara o projeto para chunking visual.
- Nenhuma regra de simulação é alterada.
- `cargo fmt --check` passa.
- `cargo check` passa.
- `cargo test` passa.
- `cargo clippy` passa.

### Checklist de execução

```bash
git checkout -b render/separate-map-view
```

Após alterar:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy
```

Commit:

```bash
git add .
git commit -m "refactor(render): separate logical tiles from map presentation"
```

## Ordem obrigatória recomendada

Executar na seguinte ordem:

```text
1. fix/limit-camera-zoom
2. debug/render-metrics
3. render/separate-map-view
```

### Por quê?

#### 1. Primeiro limitar zoom

Resolve o lag imediatamente com baixo risco.

#### 2. Depois medir

Permite comparar o impacto das próximas mudanças.

#### 3. Depois separar map view

Prepara a arquitetura para chunking, mesh por chunk e LOD sem misturar renderização com simulação.

## Próxima etapa após essas 3 tarefas

Depois que essas três tarefas estiverem concluídas, a próxima frente estrutural será:

```text
render/visual-map-chunks
```

Objetivo futuro:

```text
Dividir o mapa visual em chunks de 32x32 tiles.
```

Valores recomendados:

```text
TILE_SIZE = 64.0
CHUNK_SIZE_TILES = 32
CHUNK_SIZE_WORLD = 2048.0
```

Depois disso, a evolução natural será:

```text
1. Criar chunks visuais
2. Renderizar apenas chunks visíveis
3. Trocar sprites individuais por mesh por chunk
4. Marcar chunks alterados como dirty
5. Adicionar LOD por zoom
```

## Regra final

Estas três tarefas são estabilização e preparação.

Não tentar resolver tudo nelas.

O objetivo é:

```text
reduzir lag agora
+ medir o problema
+ preparar arquitetura para correção definitiva
```
