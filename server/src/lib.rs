pub mod runtime;

use deno_core::v8;
use tokio::time::{Duration, Instant};

use runtime::Runtime;

use deno_core::{
  anyhow::Error,
  v8::{Global, Value},
  JsRuntime,
};

async fn execute_with_timeout(
  rt: &mut JsRuntime,
  code: &str,
  timeout: Duration,
) -> Result<Global<Value>, Error> {
  let isolate = rt
    .v8_isolate()
    .thread_safe_handle();

  let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

  std::thread::spawn(move || {
    let start_time = Instant::now();

    loop {
      if rx.try_recv().is_ok() {
        return;
      }

      if start_time.elapsed() > timeout {
        isolate.terminate_execution();
        return;
      }
    }
  });

  let result = rt.execute_script("<anon>", code);
  tx.send(()).ok();

  rt.v8_isolate()
    .cancel_terminate_execution();

  result
}

pub async fn execute_code(
  rt: &mut impl Runtime,
  code: &str,
) -> String {
  let rt = rt.runtime_mut();
  let value =
    execute_with_timeout(rt, code, Duration::from_secs(2))
      .await;

  match value {
    Ok(global) => {
      let scope = &mut rt.handle_scope();
      let local = v8::Local::new(scope, global);

      serde_v8::from_v8::<serde_json::Value>(scope, local)
        .unwrap()
        .to_string()
    }
    Err(error) => error.to_string(),
  }
}

#[cfg(test)]
mod tests {
  use crate::{execute_code, runtime::ByondRuntime};
  use tokio::time::Duration;

  #[tokio::test]
  async fn execute_js() {
    let mut rt = ByondRuntime::new();

    let result = execute_code(&mut rt, "2 + 2").await;
    assert_eq!(result, "4");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let result = execute_code(
      &mut rt,
      "let a = [1, 2, 3]; a.map(i => i * 2)",
    )
    .await;
    assert_eq!(result, "[2,4,6]");
  }

  #[tokio::test]
  async fn execute_infinite_loop() {
    let mut rt = ByondRuntime::new();

    let result =
      execute_code(&mut rt, "while (true) {}; 1").await;
    assert_eq!(
      result,
      "Uncaught Error: execution terminated"
    );

    tokio::time::sleep(Duration::from_secs(5)).await;

    let result = execute_code(&mut rt, "2 + 2").await;
    assert_eq!(result, "4");
  }
}
