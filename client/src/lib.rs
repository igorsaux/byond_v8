use byond::byond;
use std::io::Write;

byond!(execute_js: code; {
    let mut server = std::process::Command::new("server.exe")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = server.stdin.as_mut().unwrap();

    stdin
        .write(
            &serde_json::to_vec(&shared::Message::ExecuteCode {
                code: String::from(code),
            })
            .unwrap(),
        )
        .unwrap();

    drop(stdin);

    let output = String::from_utf8(server.wait_with_output().unwrap().stderr).unwrap();

    format!("{}", output)
});
