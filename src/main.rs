use std::io;
use std::fs;
use serde::{Serialize, Deserialize};
use std::process::Command;


#[derive(Debug,Serialize, Deserialize)]
struct Todo {
    text: String,
    done: bool
}

const FILE_NAME: &str = "todos.json";

fn main(){
    let mut todos: Vec<Todo> = load_from_file();

    loop {
        clear_screen();
        list_todos(&mut todos);

        println!("\n\n1) Adicionar");
        // println!("2) Listar");
        println!("2) Marcar como concluída");
        println!("3) Remover");
        println!("4) Sair");
        println!("Escolha: ");

        let mut input = String::new();

        io::stdin().read_line(& mut input).expect("Erro de leitura");

        let choice = input.trim();

        match choice {
            "1" => add_todo(&mut todos),
            // "2" => list_todos(&mut todos),
            "2" => mark_done(&mut todos),
            "3" => remove_todo(&mut todos),
            "4" => {
                break;
            }
            _ => println!("Opção inválida."),
        }

        save_to_file(&todos);

    }
}

fn add_todo(todos: &mut Vec<Todo>){
    println!("Digite o texto da tarefa:");
    let mut text = String::new();

    io::stdin().read_line(&mut text).expect("Erro de leitura");

    todos.push(Todo{
        text: text.trim().to_string(),
        done: false
    });
    println!("Tarefa adicionada com sucesso");
}


fn list_todos(todos: &Vec<Todo>){
    println!("\n=== Tarefas ===");
    if todos.is_empty(){
        println!("Lista de tarefas vazia");
        return;
    }

    for(i, todo) in todos.iter().enumerate(){
        let status = if todo.done { "[x]"} else {"[ ]"};

        println!("{}: {} {}", i , status, todo.text);
    }
}

fn mark_done(todos: &mut Vec<Todo>){
    println!("Digite o índice da tarefa que deseja marcar como concluída:");
    
    let mut idx = String::new();
    io::stdin().read_line(&mut idx).expect("Erro de leitura");

    if let Ok(i) = idx.trim().parse::<usize>(){
        if let Some(todo) = todos.get_mut(i){
            todo.done = true;
            println!("Tarefa {}: concluida", todo.text);

        }
    }else{
        println!("Entrada inválida.");
    }

}

fn remove_todo(todos: &mut Vec<Todo>){
    println!("Digite o índice da tarefa que deseja remover:");

    let mut idx = String::new();
    io::stdin().read_line(&mut idx).expect("Erro ao ler");

    if let Ok(i) = idx.trim().parse::<usize>() {
        if i < todos.len() {
            todos.remove(i);
            println!("Tarefa removida!");
        } else {
            println!("Índice inválido.");
        }
    } else {
        println!("Entrada inválida.");
    }
}


fn save_to_file(todos: &Vec<Todo>){
    let json = serde_json::to_string_pretty(todos).expect("Erro ao converter para JSON");

    fs::write(FILE_NAME, json).expect("Erro ao salvar o arquivo");
}

fn load_from_file() -> Vec<Todo>{
    let file_content = fs::read_to_string(FILE_NAME);
    match file_content{
        Ok(data)=> {
            serde_json::from_str(&data).unwrap_or_else(|_|{
            println!("Arquivo corrompido, criando novo...");
            Vec::new()
            })
        }

        Err(_) =>{
            Vec::new()
        }
    }

}

fn clear_screen(){
    Command::new("clear").status().unwrap();
}