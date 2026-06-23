use thiserror::{
    Error as ThisE
};

#[derive(
    Debug,
    ThisE,
)] pub enum IOError {
    #[error("The path '{0}' does not point to a file")]
    IsNotFile(String),

    #[error("File '{0}' already exists")]
    FileAlreadyExists(String),

    #[error("No file found in '{0}'")]
    FileNotFound(String),

    #[error("Invalid input: '{0}'")]
    InvalidInput(String),

    #[error("Permission denied for path: '{0}'")]
    PermissionDenied(String),

    #[error("IO Error at '{path}': {source}")]
    RawIOError {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

#[derive(
    Debug,
    ThisE,
)] pub enum LedgerError {
    #[error("Ledger file is corrupted and could not be parsed. Details: {0}")]
    LedgerCorrupted(String),

    #[error("Permission denied for ledger file at '{0}'")]
    LedgerPermissionDenied(String),

    #[error("Failed to read ledger file at '{path}': {source}")]
    LedgerReadError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write ledger file at '{path}': {source}")]
    LedgerWriteError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Ledger state is inconsistent: {0}")]
    LedgerInvalidState(String),

    #[error("Ledger is locked!")]
    LedgerLockedError,
}

#[derive(
    Debug,
    ThisE,
)] pub enum HandError {
    #[error("Hand '{0}' not found.")]
    HandNotFound(String),

    #[error("Hand '{0}' already exists.")]
    HandAlreadyExists(String),

    #[error("Pocket dir not found in hand '{0}'.")]
    PocketNotFound(String),

    #[error("File could not be copied to pocket vault: '{0}'")]
    PocketCopyFailed(String),
}

#[derive(
    Debug,
    ThisE,
)] pub enum DBError {
    #[error("Database connection failed: '{0}'")]
    DBConnectionFailed(String),

    #[error("Database query failed: '{0}'")]
    DBQueryFailed(String),

    #[error("Database corruption detected! Risk of data loss. Details: '{0}'")]
    DBCorrupted(String),

    #[error("Record not found: {0}")]
    RecordNotFound(String),

    #[error("General database error: {0}")]
    GeneralDatabaseError(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

#[derive(
    Debug,
    ThisE,
)] pub enum GbErrorKind {
    #[error(transparent)]
    IOError(#[from] IOError),

    #[error(transparent)]
    LedgerError(#[from] LedgerError),

    #[error(transparent)]
    HandError(#[from] HandError),

    #[error(transparent)]
    DBError(#[from] DBError),

    #[error("`grabit` General error: {0}")]
    General(String),
}
