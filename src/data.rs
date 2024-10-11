pub trait FunctionLabel {
    fn label(&self) -> &'static str;
}

impl FunctionLabel for AvailableFunctions {
    fn label(&self) -> &'static str {
        match self {
            AvailableFunctions::ListaArchivi => "Lista archivi",
            AvailableFunctions::AariArchivio => "Apri archivio",
            AvailableFunctions::EliminaArchivio => "Elimina archivio",
            AvailableFunctions::InviaArchivio => "Invia Archivio",
        }
    }
}

pub enum AvailableFunctions {

    ListaArchivi,
    AariArchivio,
    EliminaArchivio,
    InviaArchivio,
}

