use std::collections::{hash_map::Entry, HashMap};

use ipc_channel::ipc::TryRecvError;
use shared::ipc::{IpcClient, IpcMessage::*, IpcNotification, IpcRequest, RuntimeType};
use uuid::Uuid;

use crate::runtime::{ByondRuntime, Runtime};

type RequestResult = Result<(), ipc_channel::Error>;

pub struct Server {
    ipc: IpcClient,
    isolates: HashMap<String, Box<dyn Runtime>>,
}

impl Server {
    pub fn new(server_name: &str) -> Self {
        let ipc = IpcClient::new(server_name);
        let isolates = HashMap::new();

        Self { ipc, isolates }
    }

    fn send_notification(&self, notification: IpcNotification) -> RequestResult {
        self.ipc.sender().send(Notification(notification))
    }

    async fn handle_request(&mut self, request: IpcRequest) -> RequestResult {
        match request {
            IpcRequest::ExecuteCode { code, isolate } => {
                self.on_execute_code(&code, &isolate).await
            }
            IpcRequest::CreateIsolate(ty) => self.on_create_isolate(ty),
            IpcRequest::DeleteIsolate { isolate } => self.on_delete_isolate(&isolate),
            IpcRequest::GetIsolates => self.on_get_isolates(),
        }
    }

    // Requests
    async fn on_execute_code(&mut self, code: &str, isolate: &str) -> RequestResult {
        let isolate = self.isolates.get_mut(isolate).unwrap().as_mut();

        let result = crate::execute_code(isolate, code).await;

        self.send_notification(IpcNotification::CodeExecutionResult(result))
    }

    // TODO: Match RuntimeType.
    fn on_create_isolate(&mut self, _ty: RuntimeType) -> RequestResult {
        let isolate_uuid = Uuid::new_v4().to_string();
        self.isolates
            .insert(isolate_uuid.clone(), Box::new(ByondRuntime::new()));

        self.send_notification(IpcNotification::IsolateCreated(isolate_uuid))
    }

    fn on_delete_isolate(&mut self, isolate: &str) -> RequestResult {
        if let Entry::Occupied(entry) = self.isolates.entry(isolate.to_string()) {
            entry.remove();
        }

        Ok(())
    }

    fn on_get_isolates(&self) -> RequestResult {
        let isolates: Vec<String> = self.isolates.keys().cloned().collect();

        self.send_notification(IpcNotification::ListOfIsolates(isolates))
    }

    async fn handle_notification(&mut self, notification: IpcNotification) {
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
                Request(request) => self.handle_request(request).await.unwrap(),
                Notification(notification) => self.handle_notification(notification).await,
            }
        }
    }
}
