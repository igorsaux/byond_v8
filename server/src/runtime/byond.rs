use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use deno_core::{error::AnyError, Extension, JsRuntime, OpState, RuntimeOptions, Snapshot};

use super::Runtime;

static BYOND_RT_SNAPSHOT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/BYOND_RT_SNAPSHOT.bin"));

pub struct ByondRuntime {
    rt: JsRuntime,
}

impl ByondRuntime {
    pub fn new() -> Self {
        let options = RuntimeOptions {
            startup_snapshot: Some(Snapshot::Static(BYOND_RT_SNAPSHOT)),
            extensions: vec![extensions()],
            ..Default::default()
        };

        let rt = JsRuntime::new(options);

        Self { rt }
    }
}

impl Default for ByondRuntime {
    fn default() -> Self {
        Self::new()
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

impl Deref for ByondRuntime {
    type Target = JsRuntime;

    fn deref(&self) -> &Self::Target {
        &self.rt
    }
}

impl DerefMut for ByondRuntime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rt
    }
}

// TODO: Make it work.
async fn op_byond_href(
    _state: Rc<RefCell<OpState>>,
    _link: String,
    _: (),
) -> Result<i32, AnyError> {
    Ok(12)
}

fn extensions() -> deno_core::Extension {
    Extension::builder()
        .ops(vec![("op_byond_href", deno_core::op_async(op_byond_href))])
        .build()
}

#[cfg(test)]
mod tests {
    use deno_core::v8::Local;

    use super::ByondRuntime;

    #[test]
    fn test_op_byond_href() {
        let mut rt = ByondRuntime::new();

        let result = rt
            .execute_script("<anon>", "this.byond.href('?v8')")
            .unwrap();

        let mut scope = rt.handle_scope();
        let result = Local::new(&mut scope, result);

        let result = serde_v8::from_v8::<serde_json::Value>(&mut scope, result)
            .unwrap()
            .to_string();

        // Promise
        assert_eq!(result, r#"{}"#);
    }
}
