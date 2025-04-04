"Com base no plano detalhado e nas orientações fornecidas no documento em anexo ["Conversão de MMORPG para Rust"], você pode refinar e expandir as seguintes áreas, visando a conversão do projeto de MMORPG 'Canary' (escrito em C++, disponível em `https://github.com/opentibiabr/canary` e no projeto em anexo) para a linguagem Rust, resultando em um novo projeto chamado **RuOt**?

O objetivo central é realizar uma conversão idiomática, mantendo a funcionalidade essencial do Canary enquanto modernizamos a stack tecnológica para aproveitar os benefícios do Rust e do ecossistema Bevy. O projeto **RuOt** deverá utilizar as seguintes tecnologias Rust e Bevy (em modo headless):

* **Engine / Arquitetura:** Bevy (paradigma ECS, headless para lógica do servidor).
* **Scripting (Lógica Dinâmica):** WebAssembly (WASM) com `wasmtime` (runtime) e `wit-bindgen` (interfaces seguras).
* **Banco de Dados:** PostgreSQL com `Diesel` (ORM/Query Builder) e `diesel_migrations` (versionamento de schema).
* **Rede:** WebSockets seguros (`tokio-tungstenite` com `rustls`), serialização com MessagePack (`rmp-serde`), e integração PROXY protocol v2 (`ppp`).
* **Proxy Layer:** **Uma camada de proxy distinta do servidor do jogo e parte integrante do projeto RuOt.** Esta camada consiste em um reverse proxy customizado (`pingora-proxy`) que deve ser implementado e forward proxies HAproxy. O reverse e forward proxy se comunicam com mTLS. O servidor do jogo deve saber o ip do client e do forward proxy.
* **Servidor de Login:** Um servidor dedicado para autenticação de jogadores e gerenciamento de contas, com foco em **alta performance**.
* **Mapa do Mundo:** Carregamento e gerenciamento com `bevy_ecs_tiled` (formato `.tmx`).
* **Métricas:** `metrics` facade com integração `tracing` (`metrics-tracing-context`), exportação via Prometheus.
* **Testes:** Framework nativo Rust (`#[test]`) e testes baseados em propriedades (`proptest`).
* **Configuração:** Arquivos TOML (`bevy_common_assets`, `serde`, `duration-str`).
* **Segredos:** `dotenvy` (dev), variáveis de ambiente/vaults (prod).
* **Localização:** Suporte a múltiplos idiomas (`bevy_fluent`).
* **CLI / TUI (Opcional):** `bevy_cli` (CLI) e `ratatui`/`crossterm` (TUI opcional).
* **Tratamento de Erros:** `Result`, `Option`, `anyhow`, `thiserror`.

**Ênfases da Conversão:**

* Arquitetura modular e baseada em ECS (Bevy).
* Segurança de memória e comunicação de rede (Rust, TLS).
* **Performance:** Buscar performance igual ou superior à versão C++ no servidor de jogo e garantir **alta performance** no servidor de login. Utilizar as ferramentas de profiling e métricas (`metrics`, `tracing`) para otimização contínua em ambos.
* Código idiomático, bem documentado, testado e manutenível (TDD, testes de propriedade).

**Solicito um guia detalhado abordando os seguintes pontos, complementando as informações do documento anexado:**

1.  **Plano de Conversão Estratégico:** O documento anexo descreve um plano de conversão faseado. Você poderia fornecer mais detalhes sobre a priorização de funcionalidades dentro dessas fases e sugerir um cronograma aproximado para cada etapa?
2.  **Superando Desafios C++ -> Rust:** O documento lista os principais desafios. Com base em sua análise do projeto 'Canary' (repositório e texto anexo), quais desafios específicos você antecipa e quais estratégias detalhadas podemos usar para mitigá-los?
3.  **Mapeamento Funcional e Tecnológico Detalhado:** O documento sugere um mapeamento de funcionalidades do 'Canary' para tecnologias Rust/Bevy. Você poderia revisar este mapeamento e sugerir alternativas ou complementos, especialmente para as áreas de `src/lua`, `src/database`, `src/map`, `src/creatures` e `src/items`?
4.  **Arquitetura RuOt com Bevy ECS:** O documento propõe uma arquitetura baseada em plugins Bevy. Você poderia detalhar a estrutura e as responsabilidades de cada plugin sugerido (CorePlugin, NetworkPlugin, etc.) e como eles interagiriam dentro do sistema ECS para garantir modularidade?
5.  **Melhores Práticas em Rust para RuOt:** O documento lista várias melhores práticas. Quais dessas práticas você considera mais críticas para o sucesso do projeto 'RuOt', e como podemos garantir sua adoção consistente pela equipe de desenvolvimento?
6.  **Estrutura de Diretórios e Arquivos Sugerida:** A estrutura de diretórios detalhada no documento parece abrangente. Você teria alguma sugestão de otimização ou considerações adicionais para a organização dos crates (`ruot_server`, `ruot_chat`, `ruot_shared`, `wasm_scripts`, `server_tui`, `ruot_login`, `ruot_proxy`)?
7.  **Exemplos de `Cargo.toml`:** Os exemplos de `Cargo.toml` no documento cobrem as dependências principais. Você poderia fornecer exemplos mais detalhados, incluindo features específicas e dependências para o servidor de login e o proxy reverso?
8.  **Documentação e Diagramação da Arquitetura:** O documento recomenda o uso de Rustdoc e diagramas UML/C4/ERD. Você poderia sugerir ferramentas específicas e um fluxo de trabalho para gerar e manter essa documentação ao longo do projeto?
9.  **Implementação do Versionamento de Banco de Dados:** O documento explica como usar `diesel_migrations`. Você poderia fornecer um exemplo prático de como uma migration para criar a tabela de contas de usuário do servidor de login seria estruturada?
10. **Estratégia para o Servidor de Chat:** O documento discute estratégias de comunicação entre servidores. Com base nas tecnologias escolhidas, qual protocolo (WebSockets, gRPC, etc.) você recomendaria para a comunicação entre `ruot_server`, `ruot_chat` e `ruot_login`, e por quê?
11. **Automação com GitHub Actions:** O documento descreve os workflows de CI/CD. Você poderia fornecer exemplos básicos dos arquivos `ci.yml` e `cd.yml` adaptados para a estrutura de crates do 'RuOt', incluindo build, teste, linting e deploy (considerando Docker para os diferentes componentes)?
12. **Integração Detalhada de WASM e WIT:** O documento explica a integração. Você poderia fornecer um exemplo de como definir uma interface WIT simples para a lógica de um item e como o código Rust (host) e o script WASM (guest) interagiriam usando `wasmtime` e `wit-bindgen`?
13. **Implementação de TUI e Localização:** O documento detalha a integração de `bevy_fluent` e `ratatui`/`bevy_cli`. Você poderia fornecer um exemplo de como carregar traduções com `bevy_fluent` e como criar um widget simples com `ratatui` para exibir informações do servidor?
14. **Exemplos de Tradução de Código:** O documento fornece exemplos básicos. Você poderia fornecer exemplos mais complexos de como uma lógica específica do 'Canary' (por exemplo, o sistema de combate ou a lógica de movimento de um NPC) implementada em C++ poderia ser traduzida para Rust/Bevy (componentes e sistemas)?
15. **Implementação do Servidor de Login:** O documento solicita um guia. Você poderia detalhar a arquitetura do servidor de login, incluindo as tecnologias Rust recomendadas para autenticação (e.g., hashing de senhas), gerenciamento de sessões e comunicação segura com o cliente e o servidor de jogo?
16. **Implementação do Reverse Proxy:** O documento pede detalhes. Você poderia fornecer um exemplo de configuração básica do `pingora-proxy` para o projeto 'RuOt', incluindo como configurar o mTLS com os forward proxies e como passar as informações do IP do cliente para o servidor de jogo usando o PROXY protocol?
17. **Infraestrutura de Rede:** O documento solicita uma explicação. Você poderia detalhar o fluxo completo de conexão de um cliente, desde a conexão inicial até a autenticação no servidor de login, a seleção de personagem (se aplicável) e a entrada no mundo do jogo através do reverse proxy e do servidor de jogo, incluindo a interação com o servidor de chat?
18. **Sistemas do Servidor de Jogo:** Poderia listar e descrever detalhadamente os sistemas que seriam implementados dentro do crate `ruot_server` (utilizando Bevy ECS), explicando a responsabilidade de cada sistema e como eles interagem para implementar a lógica do jogo (por exemplo, sistemas de movimentação, combate, gerenciamento de itens, IA de NPCs, etc.)?

Agradeço imensamente sua ajuda na elaboração deste plano de conversão!"