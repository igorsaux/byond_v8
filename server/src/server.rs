use std::{cell::RefCell, collections::HashMap};

use deno_core::v8::IsolateHandle;
use ipc_channel::ipc::TryRecvError;
use shared::ipc::{
  IpcClient, IpcMessage::*, IpcNotification, IpcRequest,
  RuntimeType,
};
use uuid::Uuid;

use crate::runtime::{ByondRuntime, Runtime};

thread_local! {
  static ISOLATES: RefCell<HashMap<Uuid, IsolateHandle>> = RefCell::new(HashMap::new());
}

type ReqeustResult = Result<(), ipc_channel::Error>;

pub struct Server {
  ipc: IpcClient,
  isolates: HashMap<Uuid, Box<dyn Runtime>>,
}

impl Server {
  pub fn new(server_name: &str) -> Self {
    let ipc = IpcClient::new(server_name);
    let isolates = HashMap::new();

    Self { ipc, isolates }
  }

  async fn handle_request(
    &mut self,
    request: IpcRequest,
  ) -> ReqeustResult {
    match request {
      IpcRequest::ExecuteCode { code, isolate } => {
        self
          .on_execute_code(&code, &isolate)
          .await
      }
      IpcRequest::CreateIsolate(ty) => {
        self
          .on_create_isolate(ty)
          .await
      }
    }
  }

  // Requests
  async fn on_execute_code(
    &mut self,
    code: &str,
    isolate: &str,
  ) -> ReqeustResult {
    let Self { ipc, isolates } = self;
    let isolate_uuid = Uuid::parse_str(isolate).unwrap();
    let isolate = isolates
      .get_mut(&isolate_uuid)
      .unwrap()
      .as_mut();

    let result = crate::execute_code(isolate, code).await;

    ipc
      .sender()
      .send(Notification(
        IpcNotification::CodeExecutionResult(result),
      ))
  }

  // TODO: Match RuntimeType.
  async fn on_create_isolate(
    &mut self,
    _ty: RuntimeType,
  ) -> ReqeustResult {
    let Self { ipc, isolates } = self;
    let isolate_uuid = Uuid::new_v4();
    isolates
      .insert(isolate_uuid, Box::new(ByondRuntime::new()));

    ipc
      .sender()
      .send(Notification(IpcNotification::IsolateCreated(
        isolate_uuid.to_string(),
      )))
  }

  async fn handle_notification(
    &mut self,
    notification: IpcNotification,
  ) {
    if let IpcNotification::Exit = notification {
      self.on_exit().await
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
