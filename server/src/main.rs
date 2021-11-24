use deno_core::JsRuntime;
use shared::Message;
use std::io::Read;

fn execute_code(code: &str) -> String {
    let mut rt = JsRuntime::new(Default::default());
    let value = rt.execute_script("vm", code).unwrap();
    let value = value.open(rt.v8_isolate());

    let result = value.to_rust_string_lossy(&mut rt.handle_scope()).clone();
    result
}

fn main() {
    let mut buffer = Vec::new();
    std::io::stdin().read_to_end(&mut buffer).ok().unwrap();

    let message = serde_json::from_slice::<Message>(&buffer).unwrap();

    match message {
        Message::ExecuteCode { code } => {
            eprintln!("{}", execute_code(&code));
        }
    }
}
