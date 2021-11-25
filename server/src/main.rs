use deno_core::{JsRuntime, RuntimeOptions};
use shared::ipc::{IpcClient, IpcMessage};

fn execute_code(code: &str) -> String {
  let mut rt = JsRuntime::new(RuntimeOptions::default());
  let value = rt
    .execute_script("vm", code)
    .unwrap();
  let value = value.open(rt.v8_isolate());

  let result =
    value.to_rust_string_lossy(&mut rt.handle_scope());

  result
}

fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  let server_name = args
    .get(1)
    .expect("Pass the server name in args.");

  let ipc = IpcClient::new(server_name);

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
          execute_code(&code),
        ))
        .unwrap(),
      IpcMessage::Exit => return,
      _ => continue,
    }
  }
}
