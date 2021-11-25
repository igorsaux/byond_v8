use std::time::{Duration, Instant};

use deno_core::{
  anyhow::Error,
  v8::{Global, Value},
  JsRuntime,
};

pub async fn execute_with_timeout(
  rt: &mut JsRuntime,
  code: &str,
  timeout: Duration,
) -> Result<Global<Value>, Error> {
  let isolate = rt
    .v8_isolate()
    .thread_safe_handle();

  let guard = tokio::spawn(async move {
    let start_time = Instant::now();

    loop {
      if start_time.elapsed() > timeout {
        isolate.terminate_execution();
        drop(isolate);
        return;
      }
    }
  });

  let result = rt.execute_script("<anon>", code);
  guard.abort();

  rt.v8_isolate()
    .cancel_terminate_execution();

  result
}
