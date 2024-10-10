
use std::str::FromStr;

pub fn to_int(s: &str) -> isize {
    let numero: isize = isize::from_str(s.trim()).expect("Errore nella conversione");
    numero
}

pub fn to_uint(s: &str) -> usize {
    let numero: usize = usize::from_str(s.trim()).expect("Errore nella conversione");
    numero
}