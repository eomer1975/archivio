use std::borrow::Borrow;

pub fn available_functions() -> [&'static str; 4] {
    [
        "Lista archivi",
        "Apri archivio",
        "Elimina archivio",
        "Invia Archivio",
    ]
}

pub fn test_data(test: &str) -> Result<String, &str> {
    // Controlla se il test corrisponde a una delle funzioni disponibili
    for &function in available_functions().iter() {
        if function == test {
            return Ok(String::from(test)); // Restituisci la stringa se presente
        }
    }
    Err("valore non presente") // Restituisci un errore se non trovato
}
