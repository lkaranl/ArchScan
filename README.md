# ArchScan 🛡️

O **ArchScan** é um scanner de portas TCP/UDP de alta performance, originalmente concebido em Python e reescrito em **Rust** utilizando concorrência assíncrona com `tokio` e interface gráfica nativa com `egui` (`eframe`).

Port Scans são programas cujo objetivo é identificar quais computadores/serviços estão ativos e, dessa forma, avaliar suas potenciais vulnerabilidades. Embora possam ser usados de forma maliciosa, este tipo de aplicação é essencial para realizar testes de intrusão (pentests) e auxiliar administradores de redes a reforçarem a segurança de seus sistemas e serviços.

A avaliação do ArchScan levou em consideração o estado das portas (abertas, fechadas, filtradas e abertas/filtradas) usando o Nmap como parâmetro de referência. Os resultados mostraram que o ArchScan detecta portas TCP/UDP de forma precisa e, em vários casos, com varreduras mais rápidas do que o Nmap sob as mesmas condições de teste.

---

## Recursos e Diferenciais da Versão Rust 🚀

1. **Concorrência Assíncrona Real:** Implementado com o runtime assíncrono [`tokio`](https://tokio.rs/), permitindo que o scanner lide com centenas de conexões de portas simultâneas de forma levíssima e sem bloqueios de thread de interface.
2. **Sem I/O Desnecessário:** A interface gráfica e a engine de varredura comunicam-se de forma assíncrona por meio de memória compartilhada thread-safe (`Arc<Mutex<>>`), sem precisar ler ou escrever em arquivos físicos temporários.
3. **Interface Gráfica Premium:** Construída em `egui` com tema escuro nativo, visualizações de colunas com scroll individuais para cada estado de porta (`Abertas`, `Fechadas`, `Filtradas`, `Aberta/Filtrada`), estatísticas em tempo real e barra de progresso animada.
4. **Detecção de Protocolo:**
   - **TCP:** Utiliza o estabelecimento de conexão TCP convencional.
   - **UDP:** Envia payloads específicos de serviços comuns (DNS, NTP, SNMP) e payloads genéricos para portas comuns.

---

## Estrutura do Projeto Rust 📂

Os principais arquivos do projeto estão em uma pasta dedicada [`rust/`](file:///Users/karan/Github/ArchScan/rust):

- [`rust/Cargo.toml`](file:///Users/karan/Github/ArchScan/rust/Cargo.toml): Configuração do Cargo e dependências do projeto.
- [`rust/src/main.rs`](file:///Users/karan/Github/ArchScan/rust/src/main.rs): Ponto de entrada do aplicativo que define a janela nativa e roda o loop de eventos.
- [`rust/src/app.rs`](file:///Users/karan/Github/ArchScan/rust/src/app.rs): Implementação da interface gráfica e controle dos estados visuais.
- [`rust/src/scanner/mod.rs`](file:///Users/karan/Github/ArchScan/rust/src/scanner/mod.rs): Orquestrador que gerencia as tarefas assíncronas concorrentes de varredura.
- [`rust/src/scanner/tcp.rs`](file:///Users/karan/Github/ArchScan/rust/src/scanner/tcp.rs): Motor de conexão TCP.
- [`rust/src/scanner/udp.rs`](file:///Users/karan/Github/ArchScan/rust/src/scanner/udp.rs): Motor de requisições UDP e payloads.
- [`rust/src/scanner/ports.rs`](file:///Users/karan/Github/ArchScan/rust/src/scanner/ports.rs): Parser inteligente de portas (aceita faixas ex: `0-1024`, lista ex: `80,443` ou portas avulsas) e armazena a lista com as Top 1000 portas mais comuns.
- [`rust/src/models.rs`](file:///Users/karan/Github/ArchScan/rust/src/models.rs): Definição de structs de configuração e resultados.

---

## Compilação e Testes 🛠️

Navegue até a pasta do projeto Rust:
```bash
cd rust
```

Para validar o código e verificar erros de sintaxe ou compilação:
```bash
cargo check
```

Para rodar os testes unitários da lógica de parsing e formatação:
```bash
cargo test
```
