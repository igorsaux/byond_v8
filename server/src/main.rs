use deno_core::{JsRuntime, RuntimeOptions};
use shared::{ipc::IpcChild, Message};

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
  let mut ipc =
    IpcChild::new(std::io::stdin(), std::io::stderr());

  let raw_message = ipc.read().unwrap();
  let message =
    serde_json::from_str(raw_message.as_str()).unwrap();

  match message {
    Message::ExecuteCode { code } => {
      eprintln!("{}", execute_code(&code));
    }
  }
}
