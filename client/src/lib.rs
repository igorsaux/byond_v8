use auxtools::{hook, runtime, Value as ByondValue};
use byond_message::ByondMessage;
use shared::SERVER_NAME;

mod byond_message;
pub mod internal;

#[hook("/proc/_start_v8")]
fn start_v8() {
    internal::start_v8(SERVER_NAME);

    Ok(ByondValue::null())
}

#[hook("/proc/_stop_v8")]
fn stop_v8() {
    internal::stop_v8();

    Ok(ByondValue::null())
}

#[hook("/proc/_execute_code")]
fn execute_js(code: ByondValue, isolate: ByondValue) {
    let code = match code.to_string() {
        Ok(code) => code,
        Err(_) => return Err(runtime!("`code` must be a string.")),
    };

    let isolate = match isolate.to_string() {
        Ok(isolate) => isolate,
        Err(_) => return Err(runtime!("`isolate` must be a string.")),
    };

    // 🤔
    let code = code.trim_matches('"');
    let isolate = isolate.trim_matches('"');

    let result = internal::execute_js(code, isolate);

    Ok(ByondValue::from_string(result).unwrap())
}

#[hook("/proc/_create_isolate")]
fn create_isolate() {
    let uuid = internal::create_isolate();
    let result = serde_json::to_string(&ByondMessage::NewIsolate { uuid }).unwrap();

    Ok(ByondValue::from_string(result).unwrap())
}

#[hook("/proc/_delete_isolate")]
fn delete_isolate(isolate: ByondValue) {
    let isolate = match isolate.to_string() {
        Ok(isolate) => isolate,
        Err(_) => return Err(runtime!("`isolate` must be a string.")),
    };

    let isolate = isolate.trim_matches('"');

    internal::delete_isolate(isolate);

    Ok(ByondValue::null())
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use shared::SERVER_PATH;

    use crate::internal;

    #[test]
    fn execute_js() {
        internal::start_v8(SERVER_PATH);
        let isolate = internal::create_isolate();

        let result = internal::execute_js("2 + 2", &isolate);
        assert_eq!(result, "4");
        thread::sleep(Duration::from_secs(4));
        let result = internal::execute_js(r#"let a = [1, 2, 3]; a.map(i => i * 2)"#, &isolate);
        assert_eq!(result, "[2,4,6]");

        internal::stop_v8();
    }

    #[test]
    fn execute_infinite_loop() {
        internal::start_v8(SERVER_PATH);
        let isolate = internal::create_isolate();

        let result = internal::execute_js(r#"while (true) {}; 1"#, &isolate);
        assert_eq!(result, "Uncaught Error: execution terminated");
        let result = internal::execute_js("2 + 2", &isolate);
        assert_eq!(result, "4");

        internal::stop_v8();
    }

    #[test]
    fn get_isolates() {
        internal::start_v8(SERVER_PATH);

        assert_eq!(internal::get_isolates().len(), 0);

        let isolate = internal::create_isolate();
        let total_isolates = internal::get_isolates();

        assert_eq!(total_isolates.len(), 1);
        assert_eq!(total_isolates[0], isolate);

        internal::stop_v8();
    }

    #[test]
    fn delete_isolate() {
        internal::start_v8(SERVER_PATH);

        let isolate = internal::create_isolate();
        assert_eq!(internal::get_isolates().len(), 1);

        internal::delete_isolate(&isolate);
        assert_eq!(internal::get_isolates().len(), 0);

        internal::stop_v8();
    }
}
