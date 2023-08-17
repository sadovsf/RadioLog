#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Failed to layout UI element")]
    FileSystemError(#[from] std::io::Error),

    #[error("Failed to initialize database")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Invalid QTH locator provided")]
    InvalidQTHLocator
}