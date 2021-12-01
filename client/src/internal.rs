use std::{cell::RefCell, process::Command};

use shared::ipc::{
  IpcMessage::*, IpcNotification, IpcRequest, IpcServer,
  IpcServerNaked,
};

thread_local! {
  static SERVER: RefCell<Option<IpcServer>> = RefCell::new(None);
}

pub fn spawn_server(server_path: &str) -> IpcServer {
  let server_path = server_path.to_string();
  let ipc = IpcServerNaked::new();

  let _ = Command::new(server_path)
    .arg(ipc.name())
    .spawn()
    .unwrap();

  ipc.wait_connection()
}

pub fn start_v8(path: &str) {
  SERVER.with(|worker| {
    let mut worker = worker.borrow_mut();

    if worker.is_some() {
      panic!("V8 is running.");
    }

    let server = spawn_server(path);
    *worker = Some(server);
  });
}

pub fn stop_v8() {
  SERVER.with(|worker| {
    let mut worker = worker.borrow_mut();

    if worker.is_none() {
      panic!("V8 is not running.");
    }

    let server = worker.take().unwrap();
    let tx = server.sender();
    tx.send(Notification(IpcNotification::Exit))
      .unwrap();
  });
}

pub fn execute_js(code: &str) -> String {
  SERVER.with(|worker| {
    let worker = worker.borrow();

    if worker.is_none() {
      panic!("Run V8 first.")
    }

    let message =
      Request(IpcRequest::ExecuteCode(code.to_string()));
    let channel = worker.as_ref().unwrap();
    channel
      .sender()
      .send(message)
      .unwrap();

    let response = channel
      .receiver()
      .recv()
      .unwrap();

    if let Notification(
      IpcNotification::CodeExecutionResult(result),
    ) = response
    {
      return result;
    }

    String::new()
  })
}
