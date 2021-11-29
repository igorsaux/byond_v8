use std::{
  process::Command,
  sync::mpsc::{self, Receiver, Sender},
  thread,
};

use shared::ipc::{IpcMessage, IpcServerNaked};

use crate::message::WorkerMessage;

pub(crate) struct DuplexChannel {
  tx: Sender<WorkerMessage>,
  rx: Receiver<WorkerMessage>,
}

impl DuplexChannel {
  pub fn sender(&self) -> &Sender<WorkerMessage> {
    &self.tx
  }

  pub fn receiver(&self) -> &Receiver<WorkerMessage> {
    &self.rx
  }
}

pub(crate) struct ClientWorker {
  rx: Receiver<WorkerMessage>,
  tx: Sender<WorkerMessage>,
}

impl ClientWorker {
  pub fn spawn(server_path: &str) -> DuplexChannel {
    let (tx, worker_rx) = mpsc::channel();
    let (worker_tx, rx) = mpsc::channel();
    let worker = ClientWorker {
      rx: worker_rx,
      tx: worker_tx,
    };
    let server_path = server_path.to_string();

    thread::spawn(move || worker.main(server_path));

    DuplexChannel { rx, tx }
  }

  fn main(self, server_path: String) {
    let ipc = IpcServerNaked::new();

    let _ = Command::new(server_path)
      .arg(ipc.name())
      .spawn()
      .unwrap();

    let ipc = ipc.wait_connection();

    loop {
      let worker_message = self.rx.try_recv().ok();

      if worker_message.is_none() {
        continue;
      }

      let worker_message = worker_message.unwrap();

      match worker_message {
        WorkerMessage::Exit => {
          ipc
            .sender()
            .send(IpcMessage::Exit)
            .unwrap();

          return;
        }
        WorkerMessage::ExecuteCode(code) => {
          let message = IpcMessage::ExecuteCode(code);

          ipc
            .sender()
            .send(message)
            .unwrap();

          let result = ipc.receiver().recv().unwrap();

          if let IpcMessage::CodeExecutionResult(result) =
            result
          {
            self
              .tx
              .send(WorkerMessage::CodeExecutionResult(
                result,
              ))
              .unwrap();
          }
        }
        _ => {}
      }
    }
  }
}
