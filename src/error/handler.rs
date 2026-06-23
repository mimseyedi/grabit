use std::io::{
    ErrorKind as IOEK,
    Result as IOResult,
};
use std::path::{
    Path,
};
use sqlx;
use crate::interface::output::{
    GbOutputT,
    GbOutputKind,
    GbSingleOutput,
};
use crate::error::types::*;

pub type GbResult<T> = std::result::Result<T, GbError>;

#[derive(
    Debug,
)] pub struct GbError {
    kind: GbErrorKind,
} impl GbError {
    pub fn new(kind: GbErrorKind) -> GbError {
        Self { kind }
    }

    pub fn print(&self) {
        GbSingleOutput::new(
            GbOutputKind::Error,
            self.kind.to_string(),
        ).print();
    }
}

pub trait IOResultExt<T> {
    fn with_path<P: AsRef<Path>>(self, path: P) -> GbResult<T>;
} impl<T> IOResultExt<T> for IOResult<T> {
    fn with_path<P: AsRef<Path>>(self, path: P) -> GbResult<T> {
        self.map_err(|err| {
            let path_str = path.as_ref().display().to_string();
            let io_err = match err.kind() {
                IOEK::NotFound =>
                    IOError::FileNotFound(path_str),

                IOEK::AlreadyExists =>
                    IOError::FileAlreadyExists(path_str),

                IOEK::PermissionDenied =>
                    IOError::PermissionDenied(path_str),

                IOEK::IsADirectory =>
                    IOError::IsNotFile(path_str),

                IOEK::InvalidInput =>
                    IOError::InvalidInput(path_str),

                _ => IOError::RawIOError {
                        path: path_str,
                        source: err,
                    },
            };
            GbError::new(GbErrorKind::IOError(io_err))
        })
    }
}

macro_rules! impl_from_for_gb_error {
    ($err:ty, $variant:ident) => {
        impl From<$err> for GbError {
            fn from(e: $err) -> Self {
                GbError::new(
                    GbErrorKind::$variant(e)
                )
            }
        }
    };
}
impl_from_for_gb_error!(IOError, IOError);
impl_from_for_gb_error!(LedgerError, LedgerError);
impl_from_for_gb_error!(HandError, HandError);
impl_from_for_gb_error!(DBError, DBError);

impl From<sqlx::Error> for GbError {
    fn from(e: sqlx::Error) -> Self {
        GbError::new(
            GbErrorKind::DBError(
                DBError::SqlxError(e)
            )
        )
    }
}
