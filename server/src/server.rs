use ipc_channel::ipc::TryRecvError;
use shared::ipc::{
  IpcClient, IpcMessage::*, IpcNotification, IpcRequest,
};

use crate::runtime::ByondRuntime;

type ReqeustResult = Result<(), ipc_channel::Error>;

pub struct Server {
  ipc: IpcClient,
  rt: ByondRuntime,
}

impl Server {
  pub fn new(server_name: &str) -> Self {
    let ipc = IpcClient::new(server_name);
    let rt = ByondRuntime::new();

    Self { ipc, rt }
  }

  async fn handle_request(
    &mut self,
    request: IpcRequest,
  ) -> ReqeustResult {
    match request {
      IpcRequest::ExecuteCode(code) => {
        self
          .on_execute_code(&code)
          .await
      }
    }
  }

  // Requests
  async fn on_execute_code(
    &mut self,
    code: &str,
  ) -> ReqeustResult {
    let Self { ipc, rt } = self;

    let result = crate::execute_code(rt, code).await;

    ipc
      .sender()
      .send(Notification(
        IpcNotification::CodeExecutionResult(result),
      ))
  }

  async fn handle_notification(
    &mut self,
    notification: IpcNotification,
  ) {
    match notification {
      IpcNotification::Exit => self.on_exit().await,
      _ => (),
    }
  }

  // Notifications
  async fn on_exit(&mut self) -> ! {
    std::process::exit(0)
  }

  pub async fn run(&mut self) {
    loop {
      let message = self
        .ipc
        .receiver()
        .try_recv()
        .map_err(|e| match e {
          TryRecvError::IpcError(e) => panic!("{:?}", e),
          _ => e,
        })
        .ok();

      let message = match message {
        None => continue,
        Some(m) => m,
      };

      match message {
        Request(request) => self
          .handle_request(request)
          .await
          .unwrap(),
        Notification(notification) => {
          self
            .handle_notification(notification)
            .await
        }
      }
    }
  }
}
