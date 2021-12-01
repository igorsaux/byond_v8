use auxtools::{hook, Value as ByondValue};
use shared::SERVER_NAME;

pub mod internal;

#[hook("/proc/_start_v8")]
fn start_v8() {
  internal::start_v8(SERVER_NAME);

  Ok(ByondValue::null())
}

#[hook("/proc/_stop_v8")]
fn stop_v8() {
  internal::stop_v8();

  Ok(ByondValue::null())
}

#[hook("/proc/_execute_js")]
fn execute_js(code: Value) {
  let code = code.to_string().unwrap();
  // 🤔
  let code = code.trim_matches('"');

  let result = internal::execute_js(code);

  Ok(ByondValue::from_string(result).unwrap())
}

#[cfg(test)]
mod tests {
  use std::{thread, time::Duration};

  use shared::SERVER_PATH;

  use crate::internal;

  #[test]
  fn execute_js() {
    internal::start_v8(SERVER_PATH);

    let result = internal::execute_js("2 + 2");
    assert_eq!(result, "4");
    thread::sleep(Duration::from_secs(4));
    let result = internal::execute_js(
      r#"let a = [1, 2, 3]; a.map(i => i * 2)"#,
    );
    assert_eq!(result, "[2,4,6]");

    internal::stop_v8();
  }

  #[test]
  fn execute_infinite_loop() {
    internal::start_v8(SERVER_PATH);

    let result =
      internal::execute_js(r#"while (true) {}; 1"#);
    assert_eq!(
      result,
      "Uncaught Error: execution terminated"
    );
    let result = internal::execute_js("2 + 2");
    assert_eq!(result, "4");

    internal::stop_v8();
  }
}
