use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    /// Erreur d'entrée/sortie.
    #[error("erreur d'E/S : {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
