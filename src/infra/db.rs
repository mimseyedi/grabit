use std::{path::{
    Path,
    PathBuf,
}, str::FromStr};
use sqlx::{
    sqlite::{
        SqlitePoolOptions,
        SqliteConnectOptions,
        SqliteJournalMode,
        SqliteSynchronous,
    },
    Executor,
    SqlitePool,
    QueryBuilder,
};
use chrono::{
    Utc,
    DateTime,
};
use crate::core::reference::{
    Ref,
    RefT,
    NewRef,
};
use crate::infra::logs::{
    NewLog,
    LogLevel,
    LogEntry,
};
use crate::infra::env::{
    HandEnv,
};
use crate::error::handler::{
    GbError,
    GbResult,
};

#[derive(
    Debug,
    Clone,
)] pub struct HandDB {
    hand: String,
    pool: SqlitePool,
} impl HandDB {
    pub fn hand(&self) -> &str {
        &self.hand
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn new(hand: &str) -> GbResult<Self> {
        let db_path = HandEnv::new(hand).db_file();
        let pool = Self::create_pool(&db_path).await?;
        Self::run_migrations(&pool).await?;
        Ok( Self {
            hand: hand.to_string(),
            pool,
        } )
    }

    async fn create_pool(db_path: &Path) -> GbResult<SqlitePool> {
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());
        let options = SqliteConnectOptions::from_str(&db_url)
            .map_err(GbError::from)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok( pool )
    }

    async fn run_migrations(pool: &SqlitePool) -> GbResult<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS __migration_versions (
                version INTEGER PRIMARY KEY,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#
        ).execute(pool)
        .await?;

        let last_version: Option<i64> = sqlx::query_scalar(
            "SELECT MAX(version) FROM __migration_versions"
        ).fetch_optional(pool)
        .await?;

        let last_version = last_version.unwrap_or(0);

        for (i, migration_sql) in
        MIGRATIONS.iter().enumerate() {
            let version = (i + 1) as i64;

            if version > last_version {
                let mut tx = pool.begin().await?;

                tx.execute(*migration_sql).await?;

                sqlx::query(
                    "INSERT INTO __migration_versions (version) VALUES (?)"
                ).bind(version)
                .execute(&mut *tx)
                .await?;

                tx.commit().await?;

                tracing::debug!(
                    "Applied migration version {}",
                    version,
                );
            }
        }

        Ok( () )
    }

    pub async fn acquire(&self) -> GbResult<sqlx::pool::PoolConnection<sqlx::Sqlite>> {
        Ok( self.pool.acquire().await? )
    }

    pub async fn begin(&self) -> GbResult<sqlx::Transaction<'_, sqlx::Sqlite>> {
        Ok( self.pool.begin().await? )
    }
}

impl HandDB {
    pub async fn add_ref(
        &self,
        new_ref: &crate::core::reference::NewRef,
    ) -> GbResult<i64> {
        let mut tx = self.begin().await?;

        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO references (rt, path, created)
            VALUES (?, ?, CURRENT_TIMESTAMP)
            RETURNING id
            "#
        ).bind(new_ref.rt)
        .bind(&new_ref.path)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok( id )
    }

    pub async fn get_refs(&self) -> GbResult<Vec<Ref>> {
        Ok(
            sqlx::query_as::<_, Ref>(
                r#"
                SELECT id, rt, path, created
                FROM references
                ORDER BY created DESC
                "#
            ).fetch_all(&self.pool)
            .await?
        )
    }

    pub async fn get_refs_by_type(
        &self,
        rt: RefT,
        limit: Option<u32>,
    ) -> GbResult<Vec<Ref>> {
        let mut builder = QueryBuilder::new(
            "SELECT id, rt, path, created FROM references WHERE rt = "
        );
        builder.push_bind(rt);
        builder.push(" ORDER BY created DESC");

        if let Some(l) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(l as i64);
        }

        Ok(
            builder.build_query_as::<Ref>()
            .fetch_all(&self.pool)
            .await?
        )
    }

    pub async fn get_ref_by_id(
        &self,
        id: i64,
    ) -> GbResult<Option<Ref>> {
        Ok(
            sqlx::query_as::<_, Ref>(
                r#"
                SELECT id, rt, path, created
                FROM references
                WHERE id = ?
                "#
            ).bind(id)
            .fetch_optional(&self.pool)
            .await?
        )
    }

    pub async fn get_ref_by_path(
        &self,
        path: &str,
    ) -> GbResult<Option<Ref>> {
        Ok(
            sqlx::query_as::<_, Ref>(
                r#"
                SELECT id, rt, path, created
                FROM references
                WHERE path = ?
                "#
            ).bind(path)
            .fetch_optional(&self.pool)
            .await?
        )
    }

    pub async fn remove_ref(&self, id: i64) -> GbResult<bool> {
        let mut tx = self.begin().await?;

        let rows_affected = sqlx::query(
            "DELETE FROM references WHERE id = ?"
        ).bind(id)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        tx.commit().await?;
        Ok( rows_affected > 0 )
    }

    pub async fn remove_ref_by_path(&self, path: &str) -> GbResult<bool> {
        let mut tx = self.begin().await?;

        let rows_affected = sqlx::query(
            "DELETE FROM references WHERE path = ?"
        ).bind(path)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        tx.commit().await?;
        Ok( rows_affected > 0 )
    }

    pub async fn count_refs(&self) -> GbResult<i64> {
        Ok(
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM references"
            ).fetch_one(&self.pool)
            .await?
        )
    }

    pub async fn ref_exists(&self, path: &str) -> GbResult<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM references WHERE path = ?"
        ).bind(path)
        .fetch_one(&self.pool)
        .await?;

        Ok( count > 0 )
    }

    pub async fn update_ref_path(
        &self,
        id: i64,
        new_path: &str,
    ) -> GbResult<bool> {
        let mut tx = self.begin().await?;

        let rows_affected = sqlx::query(
            "UPDATE references SET path = ? WHERE id = ?"
        )
        .bind(new_path)
        .bind(id)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        tx.commit().await?;
        Ok( rows_affected > 0 )
    }
}

impl HandDB {
    pub async fn add_log(
        &self,
        new_log: &NewLog,
    ) -> GbResult<i64> {
        let mut tx = self.begin().await?;

        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO logs (level, message, created)
            VALUES (?, ?, CURRENT_TIMESTAMP)
            RETURNING id
            "#
        ).bind(new_log.level)
        .bind(&new_log.message)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok( id )
    }

    pub async fn log_message(
        &self,
        level: LogLevel,
        message: impl Into<String>,
    ) -> GbResult<i64> {
        let new_log = NewLog {
            level,
            message: message.into(),
        };
        self.add_log(&new_log).await
    }

    pub async fn get_log_by_id(
        &self,
        id: i64,
    ) -> GbResult<Option<LogEntry>> {
        Ok(
            sqlx::query_as::<_, LogEntry>(
                r#"
                SELECT id, level, message, created
                FROM logs
                WHERE id = ?
                "#
            ).bind(id)
            .fetch_optional(&self.pool)
            .await?
        )
    }

    pub async fn get_all_logs(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> GbResult<Vec<LogEntry>> {
        let mut builder = QueryBuilder::new(
            "SELECT id, level, message, created FROM logs ORDER BY created DESC"
        );

        if let Some(l) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(l as i64);  // SQLite LIMIT به‌صورت i64 bind میشه
        }
        if let Some(o) = offset {
            builder.push(" OFFSET ");
            builder.push_bind(o as i64);
        }

        Ok(
            builder
            .build_query_as::<LogEntry>()
            .fetch_all(&self.pool)
            .await?
        )
    }


    pub async fn get_logs_by_level(
        &self,
        level: LogLevel,
        limit: Option<u32>,
    ) -> GbResult<Vec<LogEntry>> {
        let mut builder = QueryBuilder::new(
            "SELECT id, level, message, created FROM logs WHERE level = "
        );
        builder.push_bind(level);
        builder.push(" ORDER BY created DESC");

        if let Some(l) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(l as i64);
        }

        Ok(
            builder
            .build_query_as::<LogEntry>()
            .fetch_all(&self.pool)
            .await?
        )
    }

    pub async fn search_logs(
        &self,
        query_text: &str,
        limit: Option<u32>,
    ) -> GbResult<Vec<LogEntry>> {
        let search_pattern = format!("%{}%", query_text);

        let mut builder = QueryBuilder::new(
            "SELECT id, level, message, created FROM logs WHERE message LIKE "
        );
        builder.push_bind(search_pattern);
        builder.push(" ORDER BY created DESC");

        if let Some(l) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(l as i64);
        }

        Ok(
            builder
            .build_query_as::<LogEntry>()
            .fetch_all(&self.pool)
            .await?
        )
    }

    pub async fn get_logs_in_range(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        limit: Option<u32>,
    ) -> GbResult<Vec<LogEntry>> {
        let mut builder = QueryBuilder::new(
            "SELECT id, level, message, created FROM logs WHERE created BETWEEN "
        );
        builder.push_bind(start_date);
        builder.push(" AND ");
        builder.push_bind(end_date);
        builder.push(" ORDER BY created DESC");

        if let Some(l) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(l as i64);
        }

        Ok(
            builder
            .build_query_as::<LogEntry>()
            .fetch_all(&self.pool)
            .await?
        )
    }

    pub async fn delete_logs_older_than(
        &self,
        cutoff_date: DateTime<Utc>,
    ) -> GbResult<u64> {
        let rows_affected = sqlx::query(
            "DELETE FROM logs WHERE created < ?"
        ).bind(cutoff_date)
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok( rows_affected )
    }

    pub async fn delete_log_by_id(&self, id: i64) -> GbResult<bool> {
        let rows_affected = sqlx::query(
            "DELETE FROM logs WHERE id = ?"
        ).bind(id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok( rows_affected > 0 )
    }

    pub async fn clear_all_logs(&self) -> GbResult<u64> {
        let rows_affected = sqlx::query("DELETE FROM logs")
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok( rows_affected )
    }

    pub async fn count_logs(&self) -> GbResult<i64> {
        Ok(
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM logs"
            ).fetch_one(&self.pool)
            .await?
        )
    }

    pub async fn count_logs_by_level(
        &self,
        level: LogLevel,
    ) -> GbResult<i64> {
        Ok(
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM logs WHERE level = ?"
            ).bind(level)
            .fetch_one(&self.pool)
            .await?
        )
    }

    pub async fn get_recent_logs(
        &self,
        count: u32,
    ) -> GbResult<Vec<LogEntry>> {
        self.get_all_logs(Some(count), None).await
    }

    pub async fn get_today_logs(&self) -> GbResult<Vec<LogEntry>> {
        Ok(
            sqlx::query_as::<_, LogEntry>(
                r#"
                SELECT id, level, message, created
                FROM logs
                WHERE date(created) = date('now')
                ORDER BY created DESC
                "#
            ).fetch_all(&self.pool)
            .await?
        )
    }

    pub async fn get_this_week_logs(&self) -> GbResult<Vec<LogEntry>> {
        Ok(
            sqlx::query_as::<_, LogEntry>(
                r#"
                SELECT id, level, message, created
                FROM logs
                WHERE created >= date('now', 'weekday 6', '-7 days')
                ORDER BY created DESC
                "#
            ).fetch_all(&self.pool)
            .await?
        )
    }
}

impl HandDB {
    pub async fn add_ref_with_log(
        &self,
        new_ref: NewRef,
        log_message: &str,
    ) -> GbResult<(i64, i64)> {
        let mut tx = self.begin().await?;

        let ref_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO references (rt, path, created)
            VALUES (?, ?, CURRENT_TIMESTAMP)
            RETURNING id
            "#
        ).bind(new_ref.rt)
        .bind(&new_ref.path)
        .fetch_one(&mut *tx)
        .await?;

        let log_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO logs (level, message, created)
            VALUES (?, ?, CURRENT_TIMESTAMP)
            RETURNING id
            "#
        ).bind(LogLevel::Info)
        .bind(format!("{} - Reference ID: {}", log_message, ref_id))
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok( (ref_id, log_id) )
    }

    pub async fn remove_ref_with_log(
        &self,
        ref_id: i64,
        log_message: &str,
    ) -> GbResult<(bool, Option<i64>)> {
        let mut tx = self.begin().await?;

        let rows_affected = sqlx::query(
            "DELETE FROM references WHERE id = ?"
        ).bind(ref_id)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        let deleted = rows_affected > 0;

        let log_id = if deleted {
            let log_id: i64 = sqlx::query_scalar(
                r#"
                INSERT INTO logs (level, message, created)
                VALUES (?, ?, CURRENT_TIMESTAMP)
                RETURNING id
                "#
            ).bind(LogLevel::Info)
            .bind(format!("{} - Reference ID: {}", log_message, ref_id))
            .fetch_one(&mut *tx)
            .await?;
            Some(log_id)
        } else {
            None
        };

        tx.commit().await?;

        Ok( (deleted, log_id) )
    }

    pub async fn get_logs_for_ref(
        &self,
        ref_id: i64,
    ) -> GbResult<Vec<LogEntry>> {
        Ok(
            sqlx::query_as::<_, LogEntry>(
                r#"
                SELECT l.id, l.level, l.message, l.created
                FROM logs l
                WHERE l.message LIKE ?
                ORDER BY l.created DESC
                "#
            ).bind(format!("%Reference ID: {}%", ref_id))
            .fetch_all(&self.pool)
            .await?
        )
    }
}

impl HandDB {
    pub async fn health_check(&self) -> GbResult<bool> {
        match sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&self.pool)
            .await
        {
            Ok(_)  => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub async fn get_db_info(&self) -> GbResult<DBInfo> {
        let ref_count: i64 = self.count_refs().await?;

        let log_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM logs"
            ).fetch_one(&self.pool)
            .await?;

        let db_size: i64 = sqlx::query_scalar(
            "SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()"
        ).fetch_one(&self.pool)
        .await?;

        Ok(
            DBInfo {
                ref_count,
                log_count,
                db_size_bytes: db_size,
                db_path: HandEnv::new(self.hand()).db_file().clone(),
            }
        )
    }

    pub async fn vacuum(&self) -> GbResult<()> {
        sqlx::query("VACUUM").execute(&self.pool).await?;
        Ok(())
    }
}

#[derive(
    Debug,
    Clone,
)] pub struct DBInfo {
    pub ref_count: i64,
    pub log_count: i64,
    pub db_size_bytes: i64,
    pub db_path: PathBuf,
}

const MIGRATIONS: &[&str] = &[
    // Migration 001: Initial
    r#"
    CREATE TABLE IF NOT EXISTS references (
        id        INTEGER    PRIMARY KEY AUTOINCREMENT,
        rt        TEXT       NOT NULL,
        path      TEXT       NOT NULL,
        created   DATETIME   DEFAULT CURRENT_TIMESTAMP,

        UNIQUE(path)
    );

    CREATE INDEX IF NOT EXISTS idx_references_path ON references (path);
    CREATE INDEX IF NOT EXISTS idx_references_created ON references (created);

    CREATE TABLE IF NOT EXISTS logs (
        id        INTEGER    PRIMARY KEY AUTOINCREMENT,
        level     TEXT       NOT NULL,
        message   TEXT       NOT NULL,
        created   DATETIME   DEFAULT CURRENT_TIMESTAMP
    );

    CREATE INDEX IF NOT EXISTS idx_logs_created ON logs (created);
    CREATE INDEX IF NOT EXISTS idx_logs_level ON logs (level);
    "#,
];

impl std::fmt::Display for DBInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Database Info:\n
            Path: {}\n
            References: {}\n
            Logs: {}\n
            Size: {} bytes",
            self.db_path.display(),
            self.ref_count,
            self.log_count,
            self.db_size_bytes
        )
    }
}
