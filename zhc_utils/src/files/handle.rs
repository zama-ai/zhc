use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::files::{Extension, random_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileHandle(PathBuf);

impl FileHandle {
    pub fn from(path: impl AsRef<Path>) -> Self {
        FileHandle(path.as_ref().to_path_buf())
    }

    pub fn random(ext: Extension) -> Self {
        FileHandle(random_path(ext))
    }

    pub fn move_to(&mut self, path: impl AsRef<Path>){
        let path = path.as_ref();
        fs::rename(&self.0, path).unwrap();
        self.0 = path.to_path_buf();
    }

    pub fn open(&self) {
        let path = self.0.canonicalize().unwrap();

        let launcher = if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        };

        let status = Command::new(launcher)
            .arg(&path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();

        if !status.success() {
            panic!(
                "`{launcher}` failed to open {}: {status}",
                path.display()
            );
        }
    }
}

impl AsRef<Path> for FileHandle {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
