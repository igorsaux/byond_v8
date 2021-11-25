use deno_core::{JsRuntime, RuntimeOptions, Snapshot};

use super::Runtime;

static BYOND_RT_SNAPSHOT: &[u8] = include_bytes!(concat!(
  env!("OUT_DIR"),
  "/BYOND_RT_SNAPSHOT.bin"
));

pub struct ByondRuntime {
  rt: JsRuntime,
}

impl ByondRuntime {
  pub fn new() -> Self {
    let options = RuntimeOptions {
      startup_snapshot: Some(Snapshot::Static(
        BYOND_RT_SNAPSHOT,
      )),
      ..Default::default()
    };

    let rt = JsRuntime::new(options);

    Self { rt }
  }
}

impl Runtime for ByondRuntime {
  fn runtime(&self) -> &JsRuntime {
    &self.rt
  }

  fn runtime_mut(&mut self) -> &mut JsRuntime {
    &mut self.rt
  }
}
