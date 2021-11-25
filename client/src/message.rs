#[derive(Debug)]
pub(crate) enum WorkerMessage {
  Exit,
  ExecuteCode(String),
  CodeExecutionResult(String),
}
