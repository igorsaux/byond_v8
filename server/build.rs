use deno_core::{
  include_js_files, Extension, JsRuntime, RuntimeOptions,
};
use std::path::PathBuf;

fn main() {
  let out =
    PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
  let snapshot_path = out.join("BYOND_RT_SNAPSHOT.bin");

  let std = Extension::builder()
    .js(include_js_files!(
      prefix "byond:std",
      "js\\byond\\01_std.js",
    ))
    .build();

  let options = RuntimeOptions {
    will_snapshot: true,
    extensions: vec![std],
    ..Default::default()
  };

  let mut rt = JsRuntime::new(options);
  let snapshot = rt.snapshot();
  std::fs::write(&snapshot_path, &*snapshot).unwrap();
}
