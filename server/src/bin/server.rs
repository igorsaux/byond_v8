use shared::ipc::{IpcClient, IpcMessage};

extern crate server;

use server::{execute_code, runtime::ByondRuntime};

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
