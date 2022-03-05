use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use unidirs::{Directories, UnifiedDirs, Utf8Path, Utf8PathBuf};

pub static DIRS: Lazy<Dirs> = Lazy::new(|| Dirs::new().unwrap());

pub struct Dirs {
    config_file: Utf8PathBuf,
    db_file: Utf8PathBuf,
    dirs: UnifiedDirs,
}

impl Dirs {
    fn new() -> Result<Self> {
        let dirs = UnifiedDirs::simple("rocks", "dnaka91", "amelio")
            .build()
            .context("failed finding project dirs")?;

        Ok(Self {
            config_file: dirs.config_dir().join("config.toml"),
            db_file: dirs.data_dir().join("data.db"),
            dirs,
        })
    }

    pub fn config_file(&self) -> &Utf8Path {
        &self.config_file
    }

    pub fn db_file(&self) -> &Utf8Path {
        &self.db_file
    }

    pub fn db_dir(&self) -> &Utf8Path {
        self.dirs.data_dir()
    }
}
