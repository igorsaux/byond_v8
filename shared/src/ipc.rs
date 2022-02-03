//! 🤯

use ipc_channel::{
    ipc::{self, IpcError, IpcOneShotServer, IpcReceiver, IpcSender},
    Error,
};
use serde::{Deserialize, Serialize};

pub struct IpcClient {
    rx: IpcReceiver<IpcMessage>,
    tx: IpcSender<IpcMessage>,
}

impl IpcClient {
    pub fn new(server_name: &str) -> Self {
        use crate::ipc::IpcMessage::*;

        let tx = IpcSender::connect(server_name.to_string()).unwrap();

        let (server_tx, rx) = ipc::channel().unwrap();

        tx.send(Notification(IpcNotification::IpcSender(server_tx)))
            .unwrap();

        let (new_tx, server_rx) = ipc::channel().unwrap();

        tx.send(Notification(IpcNotification::IpcReceiver(server_rx)))
            .unwrap();

        Self { rx, tx: new_tx }
    }

    pub fn receiver(&self) -> &IpcReceiver<IpcMessage> {
        &self.rx
    }

    pub fn sender(&self) -> &IpcSender<IpcMessage> {
        &self.tx
    }
}

pub struct IpcServer {
    rx: IpcReceiver<IpcMessage>,
    tx: IpcSender<IpcMessage>,
}

impl IpcServer {
    pub fn receiver(&self) -> &IpcReceiver<IpcMessage> {
        &self.rx
    }

    pub fn sender(&self) -> &IpcSender<IpcMessage> {
        &self.tx
    }

    pub fn send_reqeust(&self, request: IpcRequest) -> Result<(), Error> {
        self.tx.send(IpcMessage::Request(request))
    }

    pub fn send_notification(&self, notification: IpcNotification) -> Result<(), Error> {
        self.tx.send(IpcMessage::Notification(notification))
    }

    pub fn recv(&self) -> Result<IpcMessage, IpcError> {
        self.rx.recv()
    }
}

pub struct IpcServerNaked {
    name: String,
    server: IpcOneShotServer<IpcMessage>,
}

impl IpcServerNaked {
    pub fn new() -> Self {
        let (server, name) = IpcOneShotServer::new().unwrap();

        Self { name, server }
    }

    pub fn wait_connection(self) -> IpcServer {
        use crate::ipc::IpcMessage::*;

        let (rx, tx_message) = self.server.accept().unwrap();

        let tx = match tx_message {
            Notification(n) => match n {
                IpcNotification::IpcSender(tx) => tx,
                _ => panic!("Expected message with IpcSender"),
            },
            _ => panic!("Expected notification."),
        };

        let rx = match rx.recv().unwrap() {
            Notification(n) => match n {
                IpcNotification::IpcReceiver(rx) => rx,
                _ => panic!("Expected message with IpcReceiver"),
            },
            _ => panic!("Expected notification"),
        };

        IpcServer { rx, tx }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for IpcServerNaked {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RuntimeType {
    Byond,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcRequest {
    ExecuteCode { code: String, isolate: String },
    CreateIsolate(RuntimeType),
    DeleteIsolate { isolate: String },
    GetIsolates,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcNotification {
    CodeExecutionResult(String),
    IpcSender(IpcSender<IpcMessage>),
    IpcReceiver(IpcReceiver<IpcMessage>),
    IsolateCreated(String),
    ListOfIsolates(Vec<String>),
    Exit,
    Error { request: String, message: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcMessage {
    Request(IpcRequest),
    Notification(IpcNotification),
}
