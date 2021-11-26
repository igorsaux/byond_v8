use std::time::Duration;

use code_executor::execute_with_timeout;
use deno_core::v8;
use runtime::{ByondRuntime, Runtime};
use shared::ipc::{IpcClient, IpcMessage};

mod code_executor;
mod runtime;

async fn execute_code(
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

#[tokio::main]
async fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  let server_name = args
    .get(1)
    .expect("Pass the server name in args.");

  let ipc = IpcClient::new(server_name);
  let mut rt = ByondRuntime::new();

  loop {
    let message = ipc.receiver().try_recv().ok();

    if message.is_none() {
      continue;
    }

    let message = message.unwrap();

    match message {
      IpcMessage::ExecuteCode(code) => ipc
        .sender()
        .send(IpcMessage::CodeExecutionResult(
          execute_code(&mut rt, &code).await,
        ))
        .unwrap(),
      IpcMessage::Exit => return,
      _ => continue,
    }
  }
}
