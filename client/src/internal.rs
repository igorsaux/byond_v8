use std::{cell::RefCell, process::Command};

use shared::ipc::{
    IpcMessage::*, IpcNotification, IpcRequest, IpcServer, IpcServerNaked, RuntimeType,
};

thread_local! {
  static SERVER: RefCell<Option<IpcServer>> = RefCell::new(None);
}

pub fn spawn_server(server_path: &str) -> IpcServer {
    let server_path = server_path.to_string();
    let ipc = IpcServerNaked::new();

    let _ = Command::new(server_path).arg(ipc.name()).spawn().unwrap();

    ipc.wait_connection()
}

pub fn start_v8(path: &str) {
    SERVER.with(|server| {
        let mut worker = server.borrow_mut();

        if worker.is_some() {
            panic!("V8 is running.");
        }

        let server = spawn_server(path);
        *worker = Some(server);
    });
}

pub fn stop_v8() {
    SERVER.with(|server| {
        let mut worker = server.borrow_mut();

        if worker.is_none() {
            panic!("V8 is not running.");
        }

        let server = worker.take().unwrap();
        server.send_notification(IpcNotification::Exit).unwrap();
    });
}

pub fn execute_js(code: &str, isolate: &str) -> String {
    SERVER.with(|server| {
        let worker = server.borrow();

        if worker.is_none() {
            panic!("Run V8 first.")
        }

        let server = worker.as_ref().unwrap();
        server
            .send_reqeust(IpcRequest::ExecuteCode {
                code: code.to_string(),
                isolate: isolate.to_string(),
            })
            .unwrap();

        let response = server.recv().unwrap();

        if let Notification(IpcNotification::CodeExecutionResult(result)) = response {
            return result;
        }

        String::new()
    })
}

pub fn create_isolate() -> String {
    SERVER.with(|server| {
        let worker = server.borrow();

        if worker.is_none() {
            panic!("Run V8 first.")
        }

        let server = worker.as_ref().unwrap();

        server
            .send_reqeust(IpcRequest::CreateIsolate(RuntimeType::Byond))
            .unwrap();

        let response = server.recv().unwrap();

        if let Notification(IpcNotification::IsolateCreated(uuid)) = response {
            return uuid;
        }

        panic!("Can't create isolate.");
    })
}

pub fn delete_isolate(isolate: &str) {
    SERVER.with(|server| {
        let server = server.borrow_mut();

        if server.is_none() {
            panic!("Run V8 first.");
        }

        let server = server.as_ref().unwrap();

        server
            .send_reqeust(IpcRequest::DeleteIsolate {
                isolate: isolate.to_string(),
            })
            .unwrap();
    })
}

pub fn get_isolates() -> Vec<String> {
    SERVER.with(|server| {
        let server = server.borrow_mut();

        if server.is_none() {
            panic!("Run V8 first.");
        }

        let server = server.as_ref().unwrap();

        server.send_reqeust(IpcRequest::GetIsolates).unwrap();

        let response = server.recv().unwrap();

        if let Notification(IpcNotification::ListOfIsolates(isolates)) = response {
            return isolates;
        }

        Vec::new()
    })
}
