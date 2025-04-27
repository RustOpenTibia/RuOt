# RUOT Decisions & Context (Dump da Conversa - 2025-04-25)

Este documento registra as decisões técnicas e arquiteturais tomadas durante as discussões sobre o desenvolvimento do projeto RuOt.

## Build System (Cargo)

### Perfis de Build

* **Perfil `perf` para Profiling:**
  * Definido em `Cargo.toml`.
  * Herda (`inherits`) do perfil `release` para manter otimizações (`opt-level = 3`, etc.).
  * Habilita símbolos de depuração completos (`debug = true`) essenciais para ferramentas como `perf` e `cargo-flamegraph`.
  * Uso: `cargo build/run/test --profile perf`.
  * Artefatos gerados em `target/perf/`.

    ```toml
    # Cargo.toml (exemplo)
    [profile.perf]
    inherits = "release"
    debug = true
    # Opcional: strip = "debuginfo" ou "none" se necessário, mas debug=true geralmente é suficiente.
    ```

### Features Condicionais

* **Bevy Dynamic Linking (`bevy/dynamic_linking`):**
  * **Objetivo:** Acelerar compilações incrementais *apenas* durante o desenvolvimento.
  * **Método:** Não usar ativação automática baseada no perfil `dev`. Em vez disso, definir uma feature customizada no `Cargo.toml` do crate/workspace que depende de Bevy.
  * **Implementação:**
    * Adicionar uma feature (ex: `dev-dynamic`) na seção `[features]`.
    * Esta feature ativa a feature do Bevy: `dev-dynamic = ["bevy/dynamic_linking"]`.
    * Ativar manualmente em comandos de desenvolvimento: `cargo run --features dev-dynamic`, `cargo build --features dev-dynamic`.
    * **NÃO** ativar esta feature para builds `release` ou `perf`.

    ```toml
    # Cargo.toml (exemplo)
    [dependencies]
    bevy = { version = "...", default-features = false, features = ["..."] } # Ajustar features base

    [features]
    default = []
    dev-dynamic = ["bevy/dynamic_linking"]
    ```

## Simulação de Ambiente (Bevy ECS)

* **Estrutura:** Utilizar um `EnvironmentPlugin` para encapsular recursos e sistemas relacionados ao ambiente.
* **Abordagem:** Usar `Resources` para estado global/base e `Components` + `Systems` para lógica e estado local/específico da entidade. Carregar parâmetros de configuração (tempo, clima, zonas) de arquivos externos (ex: TMX).

### Tempo de Jogo

* **Recurso:** `GameTime { elapsed_seconds: f64, time_scale: f64, second: u8, minute: u8, hour: u8, day_of_year: u16 }`.
* **Sistema:** `update_game_time(ResMut<GameTime>, Res<Time>)` - Atualiza `elapsed_seconds` baseado em `Time::delta_seconds_f64()` e `time_scale`, e recalcula os outros campos (hora, dia, etc.).
* **Configuração:** `time_scale` e calendário (segundos por dia/ano) configuráveis.

### Ciclo Dia/Noite e Estações

* **Recursos:** Enums `WorldPhase { Dawn, Day, Dusk, Night }` e `Season { Spring, Summer, Autumn, Winter }`, inseridos como recursos Bevy (`Res<WorldPhase>`, `Res<Season>`).
* **Sistemas:**
  * `update_world_phase(Res<GameTime>, ResMut<WorldPhase>)` - Determina a fase baseada em `GameTime.hour` e horários de transição configuráveis.
  * `update_season(Res<GameTime>, ResMut<Season>)` - Determina a estação baseada em `GameTime.day_of_year` e datas de início configuráveis.
* **Efeitos:** Outros sistemas leem `Res<WorldPhase>` e `Res<Season>` para modificar comportamento (NPC AI, spawns, etc.).

### Clima e Zonas Climáticas

* **Zonas:**
  * **Definição:** Enum `ClimateZoneType { Temperate, Arctic, Desert, Tropical, ... }`.
  * **Mapeamento:** Usar um `Resource` `ClimateZoneMap(HashMap<ChunkCoords, ClimateZoneType>)` para mapear coordenadas de chunks/regiões do mapa a um tipo de clima.
  * **Carregamento:** Um sistema `Startup` carrega os dados do mapa para popular `ClimateZoneMap`. Helper `position_to_chunk_coords` para converter posição de entidade em chave do mapa.
* **Configuração do Clima:** Recurso `WeatherConfig` contendo regras complexas:
  * Probabilidades de transição de clima por `(Season, ClimateZoneType, CurrentWeatherType)`.
  * Duração e intensidade min/max por `WeatherType`.
  * Regras de modificação/impossibilidade de clima por `(Season, ClimateZoneType)`.
* **Estado Global/Base do Clima:** Manter um `Resource` `CurrentWeather { weather_type: WeatherType, intensity: f32, remaining_duration_secs: f64 }` representando condições em larga escala ou base.
* **Sistema Global `update_weather`:** Atualiza o `Resource CurrentWeather` (reduz duração, determina *próximo* estado global/base usando `WeatherConfig`, `Season`, RNG e talvez uma zona de influência padrão).
* **Estado Local do Ambiente:**
  * **Componente:** Adicionar `LocalEnvironmentState { climate_zone: ClimateZoneType, current_weather: WeatherType, weather_intensity: f32, temperature: f32 }` a entidades (Players, NPCs) que precisam reagir ao ambiente local. Requer também um componente `Position`.
  * **Sistema:** `update_local_environment(Query<(&Position, &mut LocalEnvironmentState)>, Res<ClimateZoneMap>, Res<CurrentWeather>, Res<Season>, ...)` - Para cada entidade:
        1. Encontra a `ClimateZoneType` baseada na `Position` e `ClimateZoneMap`.
        2. Determina o clima *efetivo* e intensidade local, começando com o `CurrentWeather` global e aplicando modificações baseadas na `ClimateZoneType`, `Season` e regras da `WeatherConfig`.
        3. Calcula outros efeitos locais (ex: temperatura).
        4. Atualiza o componente `LocalEnvironmentState` da entidade.
* **Sincronização Cliente:** O servidor envia o `LocalEnvironmentState` (ou dados derivados relevantes) para o cliente do jogador para renderização local dos efeitos ambientais.

## Decisões e Diretrizes sobre Inteligência Artificial (IA)

### IA - Implementação de Lógica de Combate (Bevy ECS)

* **Decisão:** Adotar uma arquitetura flexível baseada em Bevy ECS para implementar múltiplos algoritmos de IA tradicionais (FSM, Behavior Trees, GOAP, Lógica Customizada em Rust, WASM).
* **Arquitetura Aprovada:**
    1. **Componente Seletor:** Um enum `AiLogicRequest` adicionado às entidades de Mobs para definir qual tipo de lógica usar (e.g., `AiLogicRequest::FiniteStateMachine`, `AiLogicRequest::BehaviorTree`, `AiLogicRequest::WasmScript(String)`).
    2. **Componentes de Estado Dedicados:** Componentes Bevy separados para armazenar o estado interno de cada tipo de IA (e.g., `FsmState`, `BtState`, `GoapState`, `WasmAiState`).
    3. **Definições de IA:** Estruturas de FSM, árvores BT, goals/actions GOAP, etc., gerenciadas como `Assets` ou `Resources` do Bevy para permitir carregamento e potencial hot-reloading.
    4. **Sistema de Inicialização:** Um sistema Bevy que observa `Added<AiLogicRequest>` para adicionar o componente de *estado* correto (e remover quaisquer estados antigos) à entidade quando a lógica de IA é atribuída ou alterada.
    5. **Sistemas de Execução Dedicados:** Sistemas Bevy separados para cada tipo de IA (e.g., `run_fsm_ai_system`, `run_bt_ai_system`).
    6. **Run Conditions (`.run_if()`):** Cada sistema de execução de IA usará uma condição de execução, como `.run_if(any_with_component::<FsmState>())`, para garantir que ele só opere em entidades que possuem o *estado* relevante e que o sistema não execute desnecessariamente se nenhuma entidade com aquele estado existir no momento.
* **Alinhamento com WASM:** Esta arquitetura suporta explicitamente a integração de lógica de IA implementada em **WebAssembly** (via `AiLogicRequest::WasmScript` e `WasmAiState`), alinhando-se com a estratégia definida em RuOt v4 para usar WASM/WIT para scripting de lógica de jogo complexa e dinâmica.
* **Benefícios:** Modularidade, separação de responsabilidades, performance (através de ECS e `run_if`), extensibilidade para adicionar novos tipos de IA, e alinhamento com padrões Bevy e a arquitetura geral do RuOt.

## Sistema de Itens

### Dados de Instância de Itens (Atributos Variáveis)

* **Análise de NBT (Minecraft):**
  * *Prós:* Alta flexibilidade (dados arbitrários), extensibilidade (não quebra saves antigos), formato comprovado, existem crates Rust.
  * *Contras:* Complexidade de implementação/parsing, potencial overkill para necessidades do RuOt, possível sobrecarga de performance vs. structs nativas, não idiomático Rust/Bevy, forte associação com Minecraft.
* **Análise de Structs Rust + `serde` + `bevy_reflect`:**
  * *Prós:* Idiomático Rust/Bevy, segurança de tipos em compilação, aproveita ecossistema (`serde`), boa performance para estruturas conhecidas, `bevy_reflect` oferece flexibilidade em runtime (inspeção/modificação), integração limpa com ECS.
  * *Contras:* Menos flexível para dados *totalmente* arbitrários adicionados pós-compilação comparado a NBT (mas `bevy_reflect` mitiga isso).
* **Decisão:** **Evitar NBT.** Adotar a abordagem idiomática Rust/Bevy usando **structs Rust** para definir os dados de instância (ex: `ItemInstanceData { base_id: u16, durability: Option<u16>, charges: Option<u8>, ... }`). Utilizar **`serde`** (com formato eficiente como `bincode` ou `messagepack`) para serialização (DB, rede, componentes). Utilizar **`bevy_reflect`** (`#[derive(Reflect)]`) nessas structs se flexibilidade em runtime for necessária (ex: scripting WASM).

### Agrupamento de Itens (Conceito de "Tags" do Minecraft)

* **Análise do Conceito de Tags:**
  * *Prós:* Excelente para agrupar itens semanticamente (ex: `#swords`, `#healing_potions`), simplifica lógica de jogo (crafting, skills, loot), pode ser data-driven (definido em configs), extensível.
  * *Contras:* Sistema de tags do Minecraft é específico (JSONs, namespaces) e **não serve para dados de instância** (como NBT ou a abordagem de structs).
* **Decisão:** **Implementar um sistema de agrupamento customizado para RuOt, inspirado no *conceito* de tags, mas não na sua implementação.**
* **Implementação Recomendada:**
  * Definir grupos em arquivos de configuração externos (ex: `data/item_groups.toml`).
  * Carregar esses grupos na inicialização em um `Resource` Bevy.
  * Estrutura do Recurso: Exemplo `ItemGroups(HashMap<String, HashSet<u16>>)`, mapeando um nome de grupo (String) para um conjunto de IDs de itens (`HashSet<u16>`).
  * A lógica do jogo consultará este `Resource` para verificar pertencimento a grupos.
