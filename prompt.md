## Plano Detalhado e Orientações para a Conversão do Projeto MMORPG Canary para Rust (RuOt)

Com base nos planos detalhados e nas orientações fornecidas nos documentos anexados ["Conversão de MMORPG para Rust"] **e** ["RuOt v2"] **(priorize as informações destes documentos)** e considerando que o código-fonte do projeto C++ "Canary" também estará disponível em anexo **(analise este código para identificar desafios específicos e exemplos de funcionalidades)**, você pode refinar e expandir as orientações, visando a conversão do projeto de MMORPG "Canary" (escrito em C++, disponível em `https://github.com/opentibiabr/canary`) para a linguagem Rust, resultando em um novo projeto chamado **RuOt**?

**Objetivo:**

Seu objetivo é refinar e expandir as orientações iniciais, fornecendo um guia **extremamente detalhado e prático** para a equipe de desenvolvimento realizar a conversão de forma eficiente e idiomática. O guia deve maximizar o aproveitamento dos recursos e benefícios do Gemini 2.5 Pro, considerando as tecnologias Rust e Bevy especificadas abaixo e o conhecimento que você pode extrair dos documentos anexados.


O objetivo central é realizar uma conversão idiomática, mantendo a funcionalidade essencial do Canary enquanto modernizamos a stack tecnológica para aproveitar os benefícios do Rust e do ecossistema Bevy. O projeto **RuOt** deverá utilizar as seguintes tecnologias Rust e Bevy (em modo headless):

### **Tecnologias-Chave do Projeto RuOt:**

* **Engine / Arquitetura:** Bevy (paradigma ECS, headless para lógica do servidor).
* **Scripting (Lógica Dinâmica):** WebAssembly (WASM) com `wasmtime` (runtime) e `wit-bindgen` (interfaces seguras).
* **Banco de Dados:** PostgreSQL com `Diesel` (ORM/Query Builder) e `diesel_migrations` (versionamento de schema), **com possibilidade de uso de Stored Procedures.**
* **Rede:** WebSockets seguros (`tokio-tungstenite` com `rustls`), serialização com MessagePack (`rmp-serde`), e integração PROXY protocol v2 (`ppp`).
* **Proxy Layer:** Camada de proxy customizada (`pingora-proxy`) e forward proxies HAproxy com comunicação mTLS.
* **Servidor de Login:** Servidor dedicado de alta performance com autenticação, gerenciamento de contas, **Two-Factor Authentication (2FA)** e emissão de tokens OAuth.
* **Mapa do Mundo:** `bevy_ecs_tiled` (formato `.tmx`).
* **Métricas:** `metrics` facade com `tracing` (`metrics-tracing-context`), exportação Prometheus.
* **Testes:** Framework nativo Rust (`#[test]`) e testes de propriedade (`proptest`).
* **Configuração:** Arquivos TOML (`bevy_common_assets`, `serde`, `duration-str`).
* **Segredos:** `dotenvy` (dev), variáveis de ambiente/vaults (prod).
* **Localização:** Suporte a múltiplos idiomas com (`bevy_fluent`).
* **CLI / TUI (Opcional):** `bevy_cli` (CLI) e `ratatui`/`crossterm` (TUI opcional).
* **Tratamento de Erros:** `Result`, `Option`, `anyhow`, `thiserror`.

## **Ênfases da Conversão:**

* Arquitetura modular e baseada em ECS (Bevy).
* Segurança (Rust, TLS, 2FA, OAuth, mTLS).
* Performance (servidor de jogo e login).
* Código idiomático, documentado, testado e manutenível.

**Solicito um Guia Detalhado (fornecido no formato de seções claramente demarcadas para cada ponto) abordando os seguintes pontos, com o máximo de especificidade e exemplos práticos possíveis (forneça snippets de código Rust ilustrativos, a menos que exemplos completos sejam mais apropriados), considerando o conteúdo dos arquivos anexados. O guia deve ser o mais prático e acionável possível para a equipe de desenvolvimento.**

**Por favor, comece sua resposta com um breve resumo ou visão geral do plano de conversão que será detalhado nos pontos seguintes.**

### **1. Plano de Conversão Estratégico Aprimorado:**

* Baseie-se nas fases de conversão detalhadas no documento "Conversão de MMORPG para Rust" e expanda cada uma delas, especificando as funcionalidades do "Canary" que devem ser priorizadas em cada fase. Considere a abordagem incremental e por funcionalidades mencionada em "RuOt v2".
* Sugira um cronograma realista para cada fase, levando em consideração a complexidade do projeto (conforme detalhado em "RuOt v2") e a necessidade de testes contínuos.
* Indique métricas de sucesso claras para cada fase.

### **2. Estratégias Detalhadas para Superar Desafios C++ -> Rust:**

* Analise os desafios listados no documento "Conversão de MMORPG para Rust".
* Com base no código C++ do "Canary" e nas considerações em "RuOt v2" sobre a mudança de paradigma para ECS, identifique desafios **específicos e concretos** que a equipe pode encontrar (e.g., gerenciamento de memória, concorrência, uso de ponteiros, manipulação de strings).
* Para cada desafio, forneça estratégias **passo a passo** e exemplos de como o código C++ correspondente pode ser traduzido para Rust de forma segura e idiomática, destacando os benefícios do Rust (conforme mencionado em "RuOt v2").

### **3. Mapeamento Funcional e Tecnológico Extendido:**

* Para cada área funcional principal do "Canary" (identificadas em "RuOt v2" e possivelmente no código-fonte, como sistema de combate, IA de NPCs, carregamento de mapas):
    * Descreva brevemente a funcionalidade no "Canary".
    * Mapeie essa funcionalidade para as tecnologias Rust/Bevy que serão utilizadas no "RuOt" (conforme detalhado em ambos os documentos anexados).
    * Sugira alternativas ou complementos tecnológicos, se aplicável, justificando suas escolhas com base nas informações dos anexos.
    * Forneça **exemplos de código Rust** que ilustrem como essa funcionalidade poderia ser implementada no "RuOt", possivelmente comparando com trechos do código C++.

### **4. Arquitetura Detalhada do RuOt com Bevy ECS:**

* Expanda a estrutura de plugins Bevy sugerida no documento "Conversão de MMORPG para Rust" e em "RuOt v2" (CorePlugin, NetworkPlugin, etc.).
* Para cada plugin:
    * Defina claramente suas responsabilidades e funcionalidades.
    * Liste os componentes, recursos e sistemas Bevy que ele conteria.
    * Explique como este plugin interagiria com outros plugins dentro da arquitetura ECS para implementar a lógica do jogo de forma modular e eficiente (considere as discussões sobre ECS e o exemplo do sistema de regeneração em "RuOt v2").
    * Considere e descreva a organização dos dados e o fluxo de mensagens entre os diferentes sistemas.

### **5. Melhores Práticas Essenciais em Rust para RuOt:**

* Dentre as melhores práticas listadas no documento "Conversão de MMORPG para Rust" e mencionadas em "RuOt v2" (segurança, performance, uso de `cargo clippy`, evitar `unsafe`), identifique as 5 mais cruciais para o sucesso do "RuOt".
* Para cada prática, explique detalhadamente por que ela é importante no contexto deste projeto e como a equipe de desenvolvimento pode garantir sua adoção consistente (e.g., através de linters, formatters, code reviews, workshops).
* Forneça exemplos específicos de como aplicar essas práticas ao converter a lógica do "Canary" para Rust.

### **6. Estrutura de Diretórios e Arquivos Otimizada:**

* Revise a estrutura de diretórios sugerida no documento "Conversão de MMORPG para Rust" e possivelmente detalhada em "RuOt v2" para os crates (`ruot_server`, `ruot_chat`, `ruot_shared`, `wasm_scripts`, `server_tui`, `ruot_login`, `ruot_proxy`).
* Sugira otimizações ou considerações adicionais para a organização dos arquivos dentro de cada crate, levando em conta a separação de responsabilidades e a manutenibilidade do código.
* Se possível, faça paralelos com a organização do código no projeto "Canary" e justifique as diferenças.

### **7. Exemplos Detalhados de `Cargo.toml`:**

* Forneça exemplos de arquivos `Cargo.toml` para os seguintes crates:
    * `ruot_server` (incluindo features relevantes para Bevy, WASM, rede, etc., conforme mencionado em "RuOt v2").
    * `ruot_login` (incluindo dependências para autenticação, banco de dados, 2FA, OAuth).
    * `ruot_proxy` (incluindo dependências para `pingora-proxy` ou alternativas, TLS).
* Para cada dependência, explique brevemente sua função e sugira versões específicas que sejam compatíveis com o ecossistema Bevy e Rust mais recente.

### **8. Documentação e Diagramação da Arquitetura Detalhada:**

* Sugira um conjunto específico de ferramentas (além de Rustdoc) para gerar e manter a documentação técnica do projeto "RuOt" (e.g., para diagramas UML, C4, ERD), considerando as necessidades de documentação identificadas em "RuOt v2".
* Proponha um fluxo de trabalho eficiente para a criação, revisão e atualização da documentação ao longo do ciclo de desenvolvimento.
* Indique como a documentação deve abordar as diferenças e similaridades entre a arquitetura do "Canary" e do "RuOt".

### **9. Implementação Prática do Versionamento de Banco de Dados:**

* Forneça um exemplo completo do código Rust para uma migration `diesel_migrations` que crie a tabela de contas de usuário para o servidor de login, incluindo os campos necessários (username, password hash, salt, timestamps, 2FA secret, etc.).
* Explique como essa migration seria aplicada e revertida usando o `diesel-cli`.

### **10. Estratégia Robusta para o Servidor de Chat:**

* Justifique a escolha de um protocolo específico (WebSockets, gRPC, etc.) para a comunicação entre `ruot_server`, `ruot_chat` e `ruot_login`, considerando performance, confiabilidade e facilidade de implementação (baseie-se nas discussões em "RuOt v2").
* Detalhe a arquitetura do servidor de chat, incluindo como ele receberia e distribuiria mensagens entre os jogadores.
* Considere aspectos como persistência de histórico de chat (se necessário) e moderação.

### **11. Automação Completa com GitHub Actions:**

* Forneça exemplos detalhados dos arquivos `.github/workflows/ci.yml` e `.github/workflows/cd.yml` adaptados para a estrutura de crates do "RuOt", considerando as etapas de build, teste e deploy mencionadas em "RuOt v2".
* O workflow de CI deve incluir etapas para build, testes unitários e de integração (para diferentes crates), linting (e.g., `clippy`), e formatação (e.g., `rustfmt`).
* O workflow de CD deve demonstrar como os diferentes componentes do "RuOt" (servidor de login, servidor de jogo, proxy) poderiam ser buildados e deployados (usando Docker como base).

### **12. Integração Detalhada e Exemplificada de WASM e WIT:**

* Defina uma interface WIT mais complexa para a lógica de um item (além do exemplo básico), incluindo funções para ativar o item, usar o item em um alvo, etc.
* Forneça exemplos completos de código Rust (host) e um script WASM (guest) que demonstrem a interação através dessa interface usando `wasmtime` e `wit-bindgen`, incluindo a troca de dados e a chamada de funções em ambas as direções.

### **13. Implementação Avançada de TUI e Localização:**

* Demonstre como carregar traduções para múltiplos idiomas usando `bevy_fluent`, incluindo a estrutura dos arquivos de tradução e como acessar as strings traduzidas no código Rust.
* Forneça um exemplo mais elaborado de um widget `ratatui` que exiba informações dinâmicas do servidor (e.g., número de jogadores online, uso de memória, TPS).
* Se aplicável, mostre como integrar a CLI (`bevy_cli`) para executar comandos administrativos no servidor.

### **14. Exemplos Abrangentes de Tradução de Código C++ para Rust/Bevy:**

* Escolha duas áreas de lógica complexa do "Canary", sugeridas em "RuOt v2" ou identificadas no código-fonte (e.g., o sistema de combate completo, incluindo cálculo de dano, habilidades, estados; ou a IA de NPCs com pathfinding, comportamento, interações).
* Para cada área, apresente o código C++ correspondente e forneça uma tradução completa e idiomática para Rust/Bevy, utilizando componentes e sistemas ECS. Explique as decisões de design e as vantagens da abordagem Rust (conforme discutido em "RuOt v2").

### **15. Implementação Completa do Servidor de Login:**

* Detalhe a arquitetura do servidor de login, incluindo os módulos principais (autenticação, gerenciamento de sessões, registro de contas, recuperação de senha, etc.).
* Especifique as bibliotecas Rust recomendadas para hashing seguro de senhas (e.g., `bcrypt`, `argon2`), geração e verificação de tokens (e.g., `jsonwebtoken`), e gerenciamento de sessões (e.g., usando um banco de dados em memória como Redis ou diretamente no PostgreSQL).
* Forneça um fluxo de autenticação completo, incluindo o processo de login com nome de usuário e senha, a implementação do **Two-Factor Authentication (2FA)** (detalhando o fluxo de registro e login com TOTP ou e-mail, e sugerindo bibliotecas como `oath2` ou `otpauth`), e a geração de tokens JWT para validação via OAuth. Inclua exemplos de código para as principais etapas.

### **16. Implementação Detalhada do Reverse Proxy com Pingora:**

* Forneça um exemplo de configuração completo do `pingora-proxy` para o projeto "RuOt".
* Inclua a configuração do mTLS com os forward proxies (definição de certificados, chaves).
* Demonstre como utilizar o `pingora-proxy` para receber conexões de clientes e encaminhá-las para o servidor de jogo, adicionando o cabeçalho do PROXY protocol v2 com as informações do IP do cliente e do forward proxy.

### **17. Arquitetura de Rede End-to-End:**

* Descreva detalhadamente o fluxo completo de conexão de um cliente:
    * Conexão inicial ao reverse proxy.
    * Encaminhamento para o servidor de login.
    * Processo de autenticação (incluindo 2FA).
    * Obtenção do token OAuth.
    * Seleção de personagem (se aplicável).
    * Conexão ao servidor de jogo através do reverse proxy, incluindo a apresentação do token OAuth para validação.
    * Interação com o servidor de chat (como as mensagens são roteadas).
* Inclua detalhes sobre os protocolos e formatos de dados utilizados em cada etapa (WebSockets, MessagePack, etc.).

### **18. Sistemas Detalhados do Servidor de Jogo (`ruot_server`):**

* Liste e descreva detalhadamente os principais sistemas que seriam implementados dentro do crate `ruot_server` (utilizando Bevy ECS), considerando os exemplos fornecidos em "RuOt v2" (e.g., sistema de combate).
* Para cada sistema, explique sua responsabilidade específica na lógica do jogo e como ele interage com outros sistemas através de eventos, componentes e recursos Bevy.
* Inclua exemplos de sistemas para:
    * Movimentação de entidades (jogadores, NPCs).
    * Combate (detecção de alvos, cálculo de dano, aplicação de efeitos).
    * Gerenciamento de itens (inventário, uso, drop).
    * Inteligência Artificial de NPCs (comportamento, pathfinding).
    * Interação com o mapa do mundo.
    * Gerenciamento de estados de jogo.
    * Qualquer outro sistema crucial para a funcionalidade do MMORPG.

### **19. Validação Segura de Login com OAuth no Servidor de Jogo:**

* Detalhe o processo pelo qual o servidor de jogo (`ruot_server`) validaria os tokens JWT emitidos pelo servidor de login usando OAuth.
* Sugira bibliotecas Rust adequadas para verificar a assinatura e a validade dos tokens (e.g., `jsonwebtoken`).
* Explique como garantir a segurança e a integridade dessa validação, incluindo o tratamento de erros e a prevenção de ataques como replay.
* Considere a necessidade de rotação de chaves e como isso seria implementado.

### **20. Estratégias para o Uso Eficaz de Stored Procedures:**

* Discuta cenários específicos dentro da lógica do jogo do "RuOt" onde o uso de Stored Procedures no PostgreSQL poderia ser vantajoso em termos de performance, segurança ou complexidade de lógica (e.g., operações atômicas complexas, lógica de negócios específica do banco de dados).
* Explique como o Diesel ORM pode ser utilizado para interagir com Stored Procedures (raw SQL queries ou outros mecanismos).
* Apresente exemplos de como Stored Procedures poderiam ser implementadas para funcionalidades específicas do jogo.

**Além disso, com base em sua análise dos documentos anexados, poderia identificar os potenciais gargalos ou caminhos críticos que a equipe pode encontrar durante a conversão?**

Agradeço imensamente sua ajuda na elaboração deste plano de conversão!