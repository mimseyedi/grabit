use std::fs;
use std::path::{
    Path,
};
use sqlx;
use chrono::{
    Utc,
    DateTime,
};
use crate::error::handler::{
    GbResult,
    IOResultExt,
};
use crate::shared::utils::{
    systemtime_to_datetime,
};

#[derive(
    Eq,
    Copy,
    Debug,
    Clone,
    PartialEq,
    sqlx::Type,
)] #[sqlx(
    rename_all="lowercase",
)] pub enum RefT {
    System,
    Pocket,
}

#[derive(
    Debug,
    Clone,
)] pub struct NewRef {
    pub rt: RefT,
    pub path: String,
} impl NewRef {
    pub fn new(rt: RefT, path: String) -> NewRef {
        Self { rt, path, }
    }
}

#[derive(
    Debug,
    Clone,
)] pub struct FileMetadata {
    pub size: u64,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
}

#[derive(
    Debug,
    Clone,
    sqlx::FromRow,
)] pub struct Ref {
    id: i64,
    rt: RefT,
    path: String,
    created: DateTime<Utc>,
} impl Ref {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn rt(&self) -> RefT {
        self.rt
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.path)
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }

    pub fn is_valid(&self) -> bool {
        self.path().exists()
    }

    pub fn name(&self) -> Option<&str> {
        self.path().file_name().and_then(|n| n.to_str())
    }

    pub fn stem(&self) -> Option<&str> {
        self.path().file_stem().and_then(|s| s.to_str())
    }

    pub fn extn(&self) -> Option<&str> {
        self.path().extension().and_then(|e| e.to_str())
    }

    pub fn metadata(&self) -> GbResult<FileMetadata> {
        let md = fs::metadata(&self.path)
        .with_path(self.path())?;
        Ok( FileMetadata {
            size: md.len(),
            created: md.created().ok()
                .and_then(systemtime_to_datetime),
            modified: md.modified().ok()
                .and_then(systemtime_to_datetime),
        } )
    }
}


