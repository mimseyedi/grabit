use sqlx;
use chrono::{
    Utc,
    DateTime,
};
use crate::interface::output::{
    GbOutputKind, GbOutputT, GbSingleOutput,
};

#[derive(
    Copy,
    Debug,
    Clone,
    PartialEq,
    sqlx::Type,
)] #[sqlx(
    type_name="TEXT",
    rename_all="UPPERCASE",
)] pub enum LogLevel {
    Info,
    Error,
    Warning,
    Command,
} impl LogLevel {
    fn to_output(self) -> GbOutputKind {
        match self {
            LogLevel::Info | LogLevel::Command
                => GbOutputKind::Info,
            LogLevel::Error
                => GbOutputKind::Error,
            LogLevel::Warning
                => GbOutputKind::Warning
        }
    }
}

#[derive(
    Debug,
    Clone,
)] pub struct NewLog {
    pub level: LogLevel,
    pub message: String,
} impl NewLog {
    pub fn new(
        level: LogLevel,
        message: impl Into<String>,
    ) -> NewLog {
        Self {
            level,
            message: message.into(),
        }
    }
}

#[derive(
    Debug,
    Clone,
    sqlx::FromRow,
)] pub struct LogEntry {
    id: i64,
    level: LogLevel,
    message: String,
    created: DateTime<Utc>,
} impl LogEntry {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn level(&self) -> LogLevel {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }

    pub fn print(&self) {
        GbSingleOutput::new(
            self.level().to_output(),
            self.message(),
        ).print();
    }
}
