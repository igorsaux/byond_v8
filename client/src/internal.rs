use crate::message::WorkerMessage;
use crate::worker::{ClientWorker, DuplexChannel};
use std::sync::{Arc, Mutex};

lazy_static! {
  static ref WORKER: Arc<Mutex<Option<DuplexChannel>>> =
    Arc::new(Mutex::new(None));
}

pub fn start_v8(path: &str) {
  let mut worker = WORKER.lock().unwrap();

  if worker.is_some() {
    panic!("V8 is running.");
  }

  let channel = ClientWorker::spawn(path);
  *worker = Some(channel);
}

pub fn stop_v8() {
  let mut worker = WORKER.lock().unwrap();

  if worker.is_none() {
    panic!("V8 is not running.");
  }

  let channel = worker.take().unwrap();
  let tx = channel.sender();
  tx.send(WorkerMessage::Exit)
    .unwrap();
}

pub fn execute_js(code: &str) -> String {
  let worker = WORKER.lock().unwrap();

  if worker.is_none() {
    panic!("Run V8 first.")
  }

  let message =
    WorkerMessage::ExecuteCode(code.to_string());
  let channel = worker.as_ref().unwrap();
  channel
    .sender()
    .send(message)
    .unwrap();

  let result = channel
    .receiver()
    .recv()
    .unwrap();

  if let WorkerMessage::CodeExecutionResult(result) = result
  {
    return result;
  }

  String::new()
}
