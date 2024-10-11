mod data;
mod files_io;
mod numbers;
mod console_utils;
use std::fs::File;
use std::io;

use data::available_functions;
use files_io::get_file_list2;
use files_io::read_file_content;
use numbers::to_uint;
use console_utils::clear_console;

struct State {
    file: Option<File>,
}

fn main() {
    let mut state = State { file: None };
    loop {
        clear_console();
        println!("Scegli una funzione");

        let functions: Vec<String> = available_functions();
        let mut selected_fn = String::new();

        for (index, function) in functions.iter().enumerate() {
            println!("{}: {}", index + 1, function);
        }
        println!("Per uscire: {:?}", functions.len() + 1);

        io::stdin()
            .read_line(&mut selected_fn)
            .expect("Failed to read line");

        let selected_fn = selected_fn.trim();

        let numero = to_uint(selected_fn);

        println!("Numero: {}", numero);

        if numero == 1 {
            state.file = file_list();
            println!("File selezionato: {:?}", state.file);
            get_file_content(&mut state);

            let mut selected_fn = String::new();
            io::stdin()
            .read_line(&mut selected_fn)
            .expect("Failed to read line");

        }

        if numero == functions.len() + 1 {
            println!("Uscita dal programma...");
            break;
        }
    }
}

fn file_list() -> Option<File> {
    let current_dir =
        std::env::current_dir().expect("Errore nel recupero della directory corrente");

    let mut files_dir = current_dir; // "files";
    files_dir.push("src");
    files_dir.push("files");
    println!("Il percorso è: {:?}", files_dir);
    println!("Scegli il file");
    let mut file_list = get_file_list2(&files_dir);

    for (index, file) in file_list.iter().enumerate() {
        println!("{}: {:?}", index, file);
    }

    println!("Per tornare indietro: {:?}", file_list.len() + 1);
    let mut selected_file = String::new();

    io::stdin()
        .read_line(&mut selected_file)
        .expect("Failed to read line");

    let numero = to_uint(&selected_file);

    if numero == file_list.len() + 1 {
        return None;
    }

    let file = file_list.get(numero);

    match file {
        Some(f) => Some(f.try_clone().expect("Errore nella clonazione del file")), // Restituisce una copia del file
        None => None,
    }
}


pub fn get_file_content(state: &mut State) -> String { 
    if let Some(file) =  &state.file { 
        match read_file_content(file) {
            Ok(c) => {
                println!("{}", c);
                c
            },
            Err(e) => {
                eprintln!("Errore durante la lettura del file: {}", e);
                String::new()
            }
        }
    } else {
        eprintln!("Nessun file disponibile.");
        String::new() // Restituisci una stringa vuota se non c'è un file
    }
}
