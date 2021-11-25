use deno_core::{JsRuntime, RuntimeOptions};
use std::path::PathBuf;

fn main() {
  let out =
    PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
  let snapshot_path = out.join("BYOND_RT_SNAPSHOT.bin");

  let options = RuntimeOptions {
    will_snapshot: true,
    ..Default::default()
  };

  let mut rt = JsRuntime::new(options);
  let snapshot = rt.snapshot();
  std::fs::write(&snapshot_path, &*snapshot).unwrap();
}
