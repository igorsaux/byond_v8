#[macro_use]
extern crate auxtools;

use auxtools::Value as ByondValue;

pub mod internal;
pub mod message;
pub mod worker;

#[hook("/proc/_start_v8")]
fn start_v8() {
  internal::start_v8("server.exe");

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

  use crate::internal;

  const fn get_server_path() -> &'static str {
    // Lol linux users be like:
    #[cfg(unix)]
    return "../target/debug/server";

    #[cfg(windows)]
    return "server.exe";

    #[cfg(not(windows))]
    #[cfg(not(unix))]
    compile_error!("Only unix or windows supported.");
  }

  #[test]
  fn execute_js() {
    let path = get_server_path();
    internal::start_v8(path);

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
    internal::start_v8(get_server_path());

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
