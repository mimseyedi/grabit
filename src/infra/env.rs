use std::fs;
use std::env;
use std::path::{
    Path,
    PathBuf,
};
use std::sync::{
    LazyLock,
};
use directories::{
    ProjectDirs,
};
use crate::error::handler::{
    GbResult,
    IOResultExt,
};
use crate::interface::output::{
    GbOutput,
    GbOutputKind,
    GbSingleOutput,
};

static APP_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Ok(custom_path) = env::var("GRABIT_HOME") {
        return PathBuf::from(custom_path);
    }
    if let Some(proj_dirs) = ProjectDirs::from(
        "com", "Grabit", "grabit"
    ) {
        return proj_dirs.data_dir().to_path_buf();
    }
    panic!(
        "Can't access to root dir. \
        set 'GRABIT_HOME' environment variable"
    );
});

pub fn app_root() -> &'static Path {
    APP_ROOT.as_path()
}

pub fn hands_dir() -> PathBuf {
    app_root().join("hands")
}

pub fn ledger_path() -> PathBuf {
    app_root().join("ledger.json")
}

#[derive(
    Debug,
    Clone,
)] pub struct HandEnv {
    name: String,
} impl HandEnv {
    pub fn new(name: &str) -> HandEnv {
        Self { name: name.to_string() }
    }

    pub fn init(&self) -> GbResult<GbOutput> {
        for d in &[
            hands_dir(),
            self.root_dir(),
            self.pocket_dir(),
        ] {
            fs::create_dir(d)
            .with_path(d)?;
        }
        Ok(GbOutput::Single(
            GbSingleOutput::new(
                GbOutputKind::Success,
                "The hand space was successfully initialized"
            )
        ))
    }

    pub fn root_dir(&self) -> PathBuf {
        hands_dir().join(&self.name)
    }

    pub fn pocket_dir(&self) -> PathBuf {
        self.root_dir().join("pocket")
    }

    pub fn config_file(&self) -> PathBuf {
        self.root_dir().join("config.json")
    }

    pub fn db_file(&self) -> PathBuf {
        self.root_dir().join("index.db")
    }

    pub fn touched_file(&self) -> PathBuf {
        self.root_dir().join(".touched.json")
    }
}
