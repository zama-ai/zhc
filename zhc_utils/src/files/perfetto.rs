use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::files::{FileHandle, random_path};

const UI_ORIGIN: &str = "https://ui.perfetto.dev";

const SHIM: &str = r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>__TITLE__</title>
    <style>
      html, body { margin: 0; height: 100%; background: #1c1e21; }
      iframe { display: block; border: 0; width: 100%; height: 100%; }
    </style>
  </head>
  <body>
    <iframe id="ui" src="__ORIGIN__/#!/"></iframe>
    <script>
      const ORIGIN = "__ORIGIN__";
      const TITLE = __TITLE_JS__;
      const FILE_NAME = __FILE_NAME_JS__;

      // The trace is inlined rather than fetched: a `file://` page cannot XHR
      // its siblings, and we do not want to run a local server.
      const bin = atob("__TRACE_B64__");
      const bytes = new Uint8Array(bin.length);
      for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);

      // The UI only accepts traces from its opener or its embedder; framing it
      // keeps us in the latter case, and avoids tripping popup blockers.
      const ui = document.getElementById("ui").contentWindow;

      // Messages are silently dropped until the UI document is complete, and
      // being cross-origin we cannot observe that. Ping until it answers.
      const ping = setInterval(() => ui.postMessage("PING", ORIGIN), 50);

      const onMessage = (event) => {
        if (event.data !== "PONG") return;
        // Several pings may be in flight; only hand over the trace once.
        clearInterval(ping);
        window.removeEventListener("message", onMessage);
        ui.postMessage(
          { perfetto: { buffer: bytes.buffer, title: TITLE, fileName: FILE_NAME } },
          ORIGIN,
        );
      };
      window.addEventListener("message", onMessage);
    </script>
  </body>
</html>
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerfettoTrace(PathBuf);

impl PerfettoTrace {
    pub fn from(path: impl AsRef<Path>) -> Self {
        PerfettoTrace(path.as_ref().to_path_buf())
    }

    pub fn random() -> Self {
        PerfettoTrace(random_path(super::Extension::Json))
    }

    pub fn open(&self) {
        let shim_file = FileHandle::random(super::Extension::Html);
        fs::write(&shim_file, self.shim().unwrap()).unwrap();
        shim_file.open()
    }

    fn shim(&self) -> io::Result<String> {
        fn lossy(part: Option<&OsStr>) -> String {
            part.unwrap_or_default().to_string_lossy().into_owned()
        }
        let title = lossy(self.0.file_stem());
        let file_name = lossy(self.0.file_name());

        let quote = |s: &str| serde_json::to_string(s).expect("strings always serialize");

        Ok(SHIM
            .replace("__ORIGIN__", UI_ORIGIN)
            .replace("__TITLE_JS__", &quote(&title))
            .replace("__FILE_NAME_JS__", &quote(&file_name))
            .replace("__TITLE__", &title)
            .replace("__TRACE_B64__", &base64(&fs::read(&self.0)?)))
    }
}

impl AsRef<Path> for PerfettoTrace {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut encoded = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let group = u32::from(chunk[0]) << 16
            | u32::from(chunk.get(1).copied().unwrap_or(0)) << 8
            | u32::from(chunk.get(2).copied().unwrap_or(0));
        for (sextet, shift) in [18, 12, 6, 0].into_iter().enumerate() {
            if sextet <= chunk.len() {
                encoded.push(ALPHABET[(group >> shift & 0b11_1111) as usize] as char);
            } else {
                encoded.push('=');
            }
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::base64;

    #[test]
    fn base64_matches_reference_vectors() {
        // RFC 4648 §10, covering the three padding cases.
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foob"), "Zm9vYg==");
        assert_eq!(base64(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
        // Exercises both ends of the alphabet.
        assert_eq!(base64(&[0x00, 0x10, 0x83]), "ABCD");
        assert_eq!(base64(&[0xff, 0xef, 0xbe]), "/+++");
    }
}
