use std::cell::RefCell;

use crate::message::WorkerMessage;
use crate::worker::{ClientWorker, DuplexChannel};

thread_local! {
  static WORKER: RefCell<Option<DuplexChannel>> =
   RefCell::new(None)
}

pub fn start_v8(path: &str) {
  WORKER.with(|worker| {
    let mut worker = worker.borrow_mut();

    if worker.is_some() {
      panic!("V8 is running.");
    }

    let channel = ClientWorker::spawn(path);
    *worker = Some(channel);
  });
}

pub fn stop_v8() {
  WORKER.with(|worker| {
    let mut worker = worker.borrow_mut();

    if worker.is_none() {
      panic!("V8 is not running.");
    }

    let channel = worker.take().unwrap();
    let tx = channel.sender();
    tx.send(WorkerMessage::Exit)
      .unwrap();
  });
}

pub fn execute_js(code: &str) -> String {
  WORKER.with(|worker| {
    let worker = worker.borrow();

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

    if let WorkerMessage::CodeExecutionResult(result) =
      result
    {
      return result;
    }

    String::new()
  })
}
