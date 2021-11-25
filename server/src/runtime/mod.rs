use deno_core::JsRuntime;

pub mod byond;

pub use byond::ByondRuntime;

pub trait Runtime {
  fn runtime_mut(&mut self) -> &mut JsRuntime;
  fn runtime(&self) -> &JsRuntime;
}
