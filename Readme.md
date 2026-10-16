📋 Todo List CLI

Uma ferramenta de linha de comando (CLI) leve, rápida e moderna para gerenciamento de tarefas pessoais, desenvolvida inteiramente em **Rust** utilizando o framework `clap` e persistência automática em formato estruturado.

---

## ✨ Funcionalidades (CRUD Completado)

*   **Create:** Adicione novas tarefas com facilidade.
*   **Read:** Liste suas tarefas pendentes ou concluídas de forma organizada.
*   **Update:** Marque tarefas como concluídas utilizando apenas o ID numérico.
*   **Delete:** Remova tarefas permanentemente da sua lista.
*   **Auto-reorganização:** Os IDs das tarefas remanescentes são reajustados automaticamente após qualquer exclusão.
*   **Persistência Global:** Seus dados são salvos de forma centralizada no sistema, funcionando em qualquer diretório do seu terminal.

---

## 🚀 Como Instalar

Como o projeto possui uma esteira automatizada no GitHub, você não precisa compilar o código na sua máquina caso não queira.

### Opção 1: Download Manual (Recomendado)
1. Vá até a aba [Releases](https://github.com) do repositório.
2. Baixe o binário correspondente ao seu sistema operacional (`todo-list-cli-windows.exe` ou `todo-list-cli-linux`).
3. (Opcional) Adicione o executável às variáveis de ambiente do seu sistema para utilizá-lo globalmente apenas digitando `todo`.

### Opção 2: Compilando a partir do Código Fonte (Requer Rust instalado)
Se você tiver o ambiente Rust configurado, pode clonar e instalar localmente via Cargo:

```bash
# Clonar o repositório
git clone https://github.com
cd SEU_REPOSITORIO

# Instalar globalmente via Cargo
cargo install --path .
```

---

## 🛠️ Como Usar (Instruções de Uso)

Após a instalação, você pode usar os seguintes subcomandos diretamente no seu terminal:

### 1. Adicionar uma Tarefa (`Add`)
Insira uma nova tarefa na sua lista passando a descrição entre aspas.
```bash
todo add "Estudar conceitos de propriedade (ownership) em Rust"
todo add "Limpar a mesa de trabalho"
```

### 2. Listar Tarefas (`List`)
Exibe todas as tarefas salvas com seus respectivos IDs e status de conclusão.
```bash
todo list
```
*Saída esperada:*
```text
📋 SUA LISTA DE TAREFAS:
  [ ] 1: Estudar conceitos de propriedade (ownership) em Rust
  [ ] 2: Limpar a mesa de trabalho
```

### 3. Concluir uma Tarefa (`Done`)
Modifica o status de uma tarefa específica para concluído utilizando o seu número identificador.
```bash
todo done 1
```

### 4. Remover uma Tarefa (`Delete`)
Exclui permanentemente uma tarefa do arquivo e atualiza em cascata os IDs das tarefas seguintes.
```bash
todo delete 1
```

### 5. Limpa toda a lista de tarefas (DeleteAll)
limpa permanentemente todas as tarefas 

```bash
    todo delete-all
```

---

## 📦 Onde os dados ficam salvos?

Para garantir estabilidade, as tarefas não se perdem quando você muda de diretório no terminal. O arquivo `todos.json` é armazenado nos caminhos padrões de dados de cada sistema:
*   **No Windows:** `C:\Users\SEU_USUARIO\AppData\Roaming\todo-list-cli\todos.json`
*   **No Linux:** `~/.local/share/todo-list-cli/todos.json`

---

## 🛠️ Tecnologias Utilizadas

*   [Rust](https://rust-lang.org) — Linguagem base focada em segurança de memória e performance.
*   [Clap (v4)](https://crates.io) — Gerenciador e parseador robusto de argumentos de linha de comando.
*   [Serde / Serde_JSON](https://serde.rs) — Serialização ultra veloz de estruturas em texto estruturado.
*   [Dirs](https://crates.io) — Descoberta multiplataforma de diretórios locais do sistema operacional.

---

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.