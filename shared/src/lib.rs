pub mod ipc;

#[cfg(windows)]
pub const SERVER_NAME: &str = "bv8.exe";
#[cfg(unix)]
pub const SERVER_NAME: &str = "bv8";

#[cfg(windows)]
pub const SERVER_PATH: &str = "bv8.exe";
#[cfg(unix)]
pub const SERVER_PATH: &str = "./target/debug/bv8";

#[cfg(not(windows))]
#[cfg(not(unix))]
compile_error!("Only unix or windows supported.");
