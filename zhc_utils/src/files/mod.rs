mod handle;
mod perfetto;

use std::path::PathBuf;

pub use handle::*;
pub use perfetto::*;

pub enum Extension {
    Json,
    Html,
    Asm,
    Svg,
    Txt,
}

pub fn random_path(ext: Extension) -> PathBuf {
    use std::env::temp_dir;
    let extension = match ext {
        Extension::Json => ".json",
        Extension::Html => ".html",
        Extension::Asm => ".asm",
        Extension::Svg => ".svg",
        Extension::Txt => ".txt",
    };
    temp_dir().join(format!(
        "zhc-{}-{}{}",
        std::process::id(),
        rand::random::<u64>(),
        extension
    ))
}
