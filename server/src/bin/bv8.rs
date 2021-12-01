use server::{execute_code, runtime::ByondRuntime};
use shared::ipc::{
  IpcClient, IpcMessage, IpcNotification, IpcRequest,
};

#[tokio::main]
async fn main() {
  use IpcMessage::*;

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
      Request(r) => match r {
        IpcRequest::ExecuteCode(code) => ipc
          .sender()
          .send(Notification(
            IpcNotification::CodeExecutionResult(
              execute_code(&mut rt, &code).await,
            ),
          ))
          .unwrap(),
        IpcRequest::Exit => return,
      },
      _ => {}
    };
  }
}
