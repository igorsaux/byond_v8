use byond::byond;
use shared::{ipc::IpcParent, Message};

byond!(execute_js: code; {
    let server = std::process::Command::new("server.exe")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut ipc = IpcParent::new(server);
    let message = Message::ExecuteCode {
        code: String::from(code)
    };

    ipc.write(serde_json::to_string(&message).unwrap().as_str()).unwrap();
    ipc.read().unwrap()
});
