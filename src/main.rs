use clap::{Parser,Subcommand};
use std::fs::{self,File, OpenOptions};
use std::io::{Read, Write };
use std::path::PathBuf;
use colored::*;

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "Uma CLI simples de lista de tarefas escrita em Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command:Commands,
}

//comandos da CLI
#[derive(Subcommand)]
enum Commands {
    #[command(about = "Adiciona uma tarefa")]
    Add {
        task: String,
    },
    #[command(about = "Lista todas as tarefas")]
    List,
    #[command(about = "Marca uma tarefa como concluida")]
    Done {
        id: usize,
    },
    #[command(about = "Deleta uma tarefa")]
    Delete {
        id: usize
    },
    #[command(about = "limpa todas as tarefas")]
    DeleteAll
}

//representar uma tarefa
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct TodoItem {
    id: usize,
    content: String,
    completed: bool,
}


fn get_data_path() -> PathBuf {
    // Busca a pasta de dados do usuário (AppData no Win, ~/.local/share no Linux)
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));

    // Cria uma pasta própria para o seu app
    path.push("todo-list-cli");

    // Garante que a pasta exista no sistema
    if !path.exists() {
        fs::create_dir_all(&path).expect("Não foi possível criar a pasta do aplicativo");
    }

    // Define o nome do arquivo final
    path.push("todos.json");
    path
}

fn main() {
    let cli = Cli::parse();
    let mut  todos = load_todos();
    match &cli.command {
        Commands::Add { task } => {
            let id = todos.len() + 1;
            let new_todo = TodoItem {
                id,
                content: task.to_string(),
                completed: false,
            };
            todos.push(new_todo);
            save_todos(&todos);
            // Mensagem estilizada em verde e negrito
            println!("{}", "🚀 Tarefa adicionada com sucesso!".green().bold());
        }
        Commands::List => {
            if todos.is_empty() {
                // Mensagem em azul quando não há tarefas
                println!("{}", "🎉 Nenhuma tarefa encontrada! Sua lista está limpa.".cyan());
                return;
            }
            // Cabeçalho em negrito e sublinhado
            println!("\n{}", "📋 SUA LISTA DE TAREFAS:".bold().underline());
            for item in &todos {
                if item.completed {
                    // Tarefas concluídas: Status verde [X] e texto em cinza escuro (dimmed)
                    let status = "[X]".green().bold();
                    println!("  {} {}: {}", status, item.id.to_string().dimmed(), item.content.dimmed());
                } else {
                    // Tarefas pendentes: Status amarelo [ ] e ID/texto em branco destacado
                    let status = "[ ]".yellow().bold();
                    println!("  {} {}: {}", status, item.id, item.content.normal());
                }
            }
            println!();
        }
        Commands::Done { id } => {
            if let Some(item) = todos.iter_mut().find(|t| t.id == *id) {
                item.completed = true;
                save_todos(&todos);
                // Feedback de conclusão em verde-claro
                println!("{}", format!("✅ Tarefa {} marcada como concluída!", id).green());
            } else {
                // Erros destacados em vermelho
                println!("{}", format!("❌ Erro: Tarefa com o ID {} não foi encontrada.", id).red().bold());
            }
        }
        Commands::Delete { id } => {
            let tamanho_antes = todos.len();
            todos.retain(|item| item.id != *id);

            if todos.len() < tamanho_antes {
                for (index, item) in todos.iter_mut().enumerate() {
                    item.id = index + 1;
                }
                save_todos(&todos);
                // Feedback de deleção em magenta (roxo)
                println!("{}", format!("🗑️ Tarefa {} removida e IDs reorganizados!", id).magenta());
            } else {
                println!("{}", format!("❌ Erro: Tarefa com o ID {} não foi encontrada.", id).red().bold());
            }
        }
        Commands::DeleteAll => {
            if !todos.is_empty() {
                todos.clear();
                println!("{}","🗑️ Todas as Tarefas Expluidas com sucesso!".green().bold());
            }
        }
    }
}


//pegar tudo
fn load_todos() -> Vec<TodoItem> {
    let path = get_data_path();

    // Se o arquivo não existir, retorna uma lista vazia
    if !path.exists() {
        return Vec::new();
    }

    // Abre o arquivo para leitura
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return Vec::new();
    }

    // Converte o texto JSON de volta para o Vetor de tarefas em Rust
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_todos(todos: &[TodoItem]) {
    let path =get_data_path();

    // Abre o arquivo configurado para sobrescrever ou criar se não existir
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Não foi possível abrir o arquivo para salvar.");

    // Converte a estrutura de dados Rust para uma string formatada em JSON
    let json = serde_json::to_string_pretty(todos)
        .expect("Erro ao serializar os dados para JSON.");

    // Escreve os bytes no arquivo físico
    file.write_all(json.as_bytes())
        .expect("Erro ao gravar os dados no arquivo.");
}


