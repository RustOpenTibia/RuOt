# Estrutura organizada do projeto **RuOt**

``` dirs
ruot/                           # Raiz do projeto (Workspace Cargo)
├── .cargo/                     # Configuração do Cargo (opcional)
│   └── config.toml
├── .github/                    # Configuração do GitHub
│   └── workflows/              # GitHub Actions
│       ├── ci.yml              # Workflow de Integração Contínua
│       └── cd.yml              # Workflow de Deploy Contínuo
├── .vscode/                    # Configuração do VSCode (opcional)
│   └── settings.json
├── crates/                     # Pacotes (crates) do workspace RuOt
│   ├── ruot_server/            # Crate principal do servidor de jogo RuOt
│   │   ├── Cargo.toml          # Dependências (bevy, wasmtime, diesel, tokio-tungstenite, etc.)
│   │   ├── build.rs            # Script de build (gerar bindings WIT host)
│   │   ├── Dockerfile          # Build da imagem Docker do servidor de jogo
│   │   └── src/                # Código fonte do servidor de jogo
│   │       ├── main.rs         # Ponto de entrada App Bevy
│   │       ├── config.rs       # Módulo de configuração (TOML)
│   │       ├── database.rs     # Módulo de banco de dados (Diesel, pool)
│   │       ├── ecs/            # Lógica central do ECS Bevy
│   │       │   ├── mod.rs
│   │       │   ├── components.rs # Componentes (Vitals, Position, PlayerInfo, etc.)
│   │       │   ├── resources.rs  # Recursos (GameTime, MapInfo, etc.)
│   │       │   ├── events.rs     # Eventos (PlayerMoveEvent, DamageEvent, etc.)
│   │       │   └── systems/      # Sistemas Bevy (movimento, combate, AI, ações)
│   │       │       └── mod.rs
│   │       ├── map/            # Módulo do mapa (bevy_ecs_tiled, pathfinding)
│   │       │   └── mod.rs
│   │       ├── network/          # Módulo de rede (WebSockets, protocol)
│   │       │   └── mod.rs
│   │       ├── scripting/        # Módulo de scripting WASM/WIT
│   │       │   ├── mod.rs
│   │       │   ├── wasm_manager.rs # Gerenciador wasmtime (engine, módulos)
│   │       │   ├── host_api.rs   # Implementação da trait Host WIT (WasmData)
│   │       │   ├── execution.rs  # Sistemas Bevy que chamam scripts
│   │       │   └── bindings.rs   # Inclui bindings WIT gerados (`include!(...)`)
│   │       ├── localization.rs   # Módulo de integração bevy_fluent
│   │       └── utils.rs
│   │   └── tests/              # Testes (unitários, integração, propriedades)
│   │
│   ├── ruot_chat/              # Crate separada para o servidor de chat RuOt
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   │       ├── main.rs
│   │       └── lib.rs          # Lógica do chat, comunicação com ruot_server
│   │   └── tests/
│   │
│   ├── ruot_shared/            # Crate para código compartilhado (tipos, constantes)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── types.rs        # Ex: EntityId, Position, etc.
│   │
│   ├── wasm_scripts/           # Crate para os scripts WASM (Guest)
│   │   ├── Cargo.toml          # Dependências (wit-bindgen-rt), config para cdylib/wasm32
│   │   ├── build.rs            # Script de build (gerar bindings WIT guest)
│   │   └── src/                # Código fonte dos scripts (Rust -> WASM)
│   │       ├── lib.rs          # Implementa e exporta traits WIT
│   │       ├── items/          # Scripts de itens
│   │       │   └── mod.rs
│   │       ├── npcs/           # Scripts de NPCs
│   │       │   └── mod.rs
│   │       └── bindings.rs     # Inclui bindings WIT gerados (`include!(...)`)
│   │
│   └── ruot_tui/               # (Opcional) Crate para a interface TUI
│       ├── Cargo.toml          # Dependências (ratatui, crossterm, bevy_cli)
│       └── src/
│           └── main.rs         # Lógica da aplicação TUI
│
├── data/                       # Dados estáticos e assets
│   ├── config/                 # Arquivos de configuração exemplo
│   │   ├── ruot_server.toml.example
│   │   └── ruot_chat.toml.example
│   ├── locales/                # Arquivos de tradução (Fluent)
│   │   ├── en-US/
│   │   └── pt-BR/
│   ├── maps/                   # Arquivos de mapa Tiled (.tmx)
│   │   └── world.tmx
│   ├── scripts/                # Onde os .wasm compilados são colocados
│   │   └── item_potion_on_use.wasm
│   └── secrets/                # Ignorado pelo Git (usar .env ou vars de ambiente)
│       └── .gitignore          # -> *
│
├── migrations/                 # Migrações de banco de dados (Diesel para PostgreSQL)
│   └── postgresql/
│       └── YYYY-MM-DD-HHMMSS_migration_name/
│           ├── up.sql
│           └── down.sql
│
├── wit/                        # Definições da Interface WebAssembly (WIT)
│   ├── ruot-guest.wit          # (Renomeado para consistência)
│   ├── ruot-host.wit           # (Renomeado para consistência)
│   └── scripting.wit           # World que conecta guest e host
│
├── target/                     # Diretório de build (ignorado)
│
├── .env.example                # Exemplo de vars de ambiente para `dotenvy`
├── .gitignore                  # Padrão Rust + específicos
├── Cargo.toml                  # Configuração do workspace RuOt
├── diesel.toml                 # Configuração do Diesel CLI
├── LICENSE                     # Licença do projeto
├── README.md                   # Documentação principal do RuOt
├── rust-toolchain.toml         # Versão do toolchain Rust
└── Taskfile.yml / Makefile     # (Opcional) Atalhos para comandos comuns
```
