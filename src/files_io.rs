use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

pub fn get_file_list(dir: &PathBuf) -> Vec<String> {
    let mut file_list = Vec::new(); // Crea un vettore vuoto per memorizzare i nomi dei file

    // Leggi il contenuto della directory specificata
    let entries = fs::read_dir(dir);
    
    match entries {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        let file_name = e.file_name(); // Ottieni il nome del file
                        file_list.push(file_name.to_string_lossy().into_owned()); // Aggiungi il nome al vettore
                    }
                    Err(e) => eprintln!("Errore nell'accesso all'entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Errore nella lettura della directory: {}", e),
    }

    file_list // Restituisci il vettore con i nomi dei file
}


pub fn get_file_list2(dir: &PathBuf) -> Vec<File> {
    let mut file_list = Vec::new(); // Crea un vettore vuoto per memorizzare i file

    // Leggi il contenuto della directory specificata
    let entries = fs::read_dir(dir);
    
    match entries {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        let path = e.path(); // Ottieni il percorso completo dell'entry
                        match File::open(&path) { // Tenta di aprire il file
                            Ok(file) => {
                                file_list.push(file); // Aggiungi il file al vettore
                            }
                            Err(e) => eprintln!("Errore nell'apertura del file '{}': {}", path.display(), e),
                        }
                    }
                    Err(e) => eprintln!("Errore nell'accesso all'entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Errore nella lettura della directory: {}", e),
    }

    file_list // Restituisci il vettore con i file
}

pub fn read_file_content_no_error(file: &mut File) -> io::Result<String> {
    let mut contenuto = String::new();
    file.read_to_string(&mut contenuto)?;
    Ok(contenuto)
}

pub fn read_file_content(file: &File) -> io::Result<String> {
    let mut contenuto = String::new();
    
    match file.try_clone().unwrap().read_to_string(&mut contenuto) {
        Ok(_) => Ok(contenuto),
        Err(e) => {
            eprintln!("Errore durante la lettura del file: {}", e);
            Err(e)
        }
    }
}