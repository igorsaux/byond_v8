#[macro_use]
extern crate lazy_static;

mod ffi;
pub mod internal;
pub mod message;
pub mod worker;

#[cfg(test)]
mod tests {
  use crate::internal;

  #[test]
  fn execute_js() {
    internal::start_v8("server.exe");

    let result = internal::execute_js("2 + 2");
    assert_eq!(result, "4");
    let result = internal::execute_js(
      r#"let a = [1, 2, 3]; a.map(i => i * 2)"#,
    );
    assert_eq!(result, "[2,4,6]");

    internal::stop_v8();
  }
}
