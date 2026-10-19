# 📋 Todo List CLI (Secure Version)

Uma ferramenta de linha de comando (CLI) ultra veloz e moderna para gerenciamento de tarefas pessoais, desenvolvida em **Rust** utilizando o framework `clap` e persistência binária protegida contra alterações externas.

---

## ✨ Funcionalidades (CRUD + Segurança)

*   **Create:** Adicione novas tarefas com facilidade (`add`).
*   **Read:** Liste suas tarefas de forma organizada (`list`).
*   **Update:** Marque tarefas como concluídas utilizando apenas o ID numérico (`done`).
*   **Delete:** Remova tarefas individualmente (`delete`) ou limpe toda a lista (`delete-all`).
*   **Auto-reorganização:** Os IDs das tarefas remanescentes são reajustados em cascata após qualquer exclusão.
*   **Persistência Binária Avançada (`postcard`):** Os dados não são salvos em texto aberto. O aplicativo utiliza codificação de bytes compacta e segura.
*   **Detecção de Adulteração:** Se o arquivo de dados for aberto e modificado manualmente por fora da CLI (ex: via Bloco de Notas), o aplicativo detectará a violação de integridade por segurança.

---

## 🚀 Como Instalar

Como o projeto possui uma esteira automatizada no GitHub Actions, você pode baixar as versões prontas diretamente.

### Opção 1: Download Manual
1. Vá até a aba [Releases](https://github.com) do repositório.
2. Baixe o binário correspondente ao seu sistema operacional (`todo-list-cli-windows.exe` ou `todo-list-cli-linux`).
3. Adicione o executável às variáveis de ambiente do seu sistema para utilizá-lo globalmente digitando apenas `todo`.

### Opção 2: Compilando do Código Fonte (Requer Rust)
```bash
git clone https://github.com/crl1092/todo_cli.git
cd todo_cli
cargo install --path .
```

---

## 🛠️ Demonstração Visual e Comandos

Após a instalação, gerencie seu fluxo de trabalho no terminal:

```bash
# 1. Adicionar tarefas
todo add "Estudar conceitos de propriedade (ownership) em Rust"
todo add "Limpar a mesa de trabalho"

# 2. Listar tarefas (Exibe status colorido: [ ] Pendente / [X] Concluído)
todo list

# 3. Concluir tarefa (Gera um efeito fosco/dimmed no texto)
todo done 1

# 4. Remover uma tarefa específica (Reorganiza os IDs automaticamente)
todo delete 1

# 5. Limpar todo o histórico
todo delete-all
```

---

## 🔒 Segurança e Local dos Dados

Para garantir que suas tarefas funcionem em qualquer lugar do terminal, o arquivo `todos.dat` é armazenado centralizadamente:
*   **No Windows:** `C:\Users\SEU_USUARIO\AppData\Roaming\todo-list-cli\todos.dat`
*   **No Linux:** `~/.local/share/todo-list-cli/todos.dat`

> ⚠️ **Nota de Integridade:** O arquivo utiliza serialização binária através da biblioteca **`postcard`**. Modificações manuais de caracteres que corrompam o formato binário farão com que a CLI acione o alerta de segurança e resete a estrutura para preservar a estabilidade da aplicação.

---

## 🛠️ Dependências Utilizadas

*   [Rust](https://rust-lang.org) — Performance e segurança de memória.
*   [Clap (v4)](https://crates.io/crates/clap) — Argument Parser robusto para CLI.
*   [Serde](https://serde.rs) — Framework de serialização/deserialização.
*   [Postcard](https://crates.io/crates/postcard) — Serializador binário seguro focado em portabilidade.
*   [Dirs](https://crates.io/crates/dirs) — Descoberta multiplataforma de diretórios do sistema.
*   [Colored](https://crates.io/crates/colored) — Estilização ansi de cores no terminal.
