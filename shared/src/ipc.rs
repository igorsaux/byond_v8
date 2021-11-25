//! 🤯

use ipc_channel::ipc::{
  self, IpcOneShotServer, IpcReceiver, IpcSender,
};
use serde::{Deserialize, Serialize};

pub struct IpcClient {
  rx: IpcReceiver<IpcMessage>,
  tx: IpcSender<IpcMessage>,
}

impl IpcClient {
  pub fn new(server_name: &str) -> Self {
    let tx =
      IpcSender::connect(server_name.to_string()).unwrap();

    let (server_tx, rx) = ipc::channel().unwrap();

    tx.send(IpcMessage::Sender(server_tx))
      .unwrap();

    let (new_tx, server_rx) = ipc::channel().unwrap();

    tx.send(IpcMessage::Receiver(server_rx))
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
    let (rx, tx_message) = self.server.accept().unwrap();

    let tx = match tx_message {
      IpcMessage::Sender(tx) => tx,
      _ => panic!("Expected message with IpcSender"),
    };

    let rx = match rx.recv().unwrap() {
      IpcMessage::Receiver(rx) => rx,
      _ => panic!("Expected message with IpcReceiver"),
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
pub enum IpcMessage {
  Sender(IpcSender<IpcMessage>),
  Receiver(IpcReceiver<IpcMessage>),
  ExecuteCode(String),
  CodeExecutionResult(String),
  Exit,
}
