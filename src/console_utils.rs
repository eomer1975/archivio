use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{self};
use std::process::Command;

pub fn clear_console() {
    execute!(io::stdout(), Clear(ClearType::All)).expect("Errore nella pulizia della console");
}



pub fn clear_console2() {
    if cfg!(target_os = "windows") {
        // Per Windows
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        // Per Unix-like (Linux, macOS)
        let _ = Command::new("clear").status();
    }
}