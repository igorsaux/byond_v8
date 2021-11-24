use std::{
    io::{Read, Result, Stderr, Stdin, Stdout, Write},
    process::Child,
};

pub struct IpcParent {
    child: Child,
}

impl IpcParent {
    pub fn new(child: Child) -> Self {
        Self { child }
    }

    pub fn read(&mut self) -> Result<String> {
        let mut stderr = self.child.stderr.take().unwrap();
        let mut buffer = Vec::new();

        stderr.read_to_end(&mut buffer)?;
        let buffer = String::from_utf8(buffer).unwrap();

        Ok(buffer)
    }

    pub fn write(&mut self, message: &str) -> Result<()> {
        let mut stdin = self.child.stdin.take().unwrap();

        stdin.write_all(message.as_bytes())?;
        stdin.flush()
    }
}

impl Drop for IpcParent {
    fn drop(&mut self) {
        self.child.wait().unwrap();
    }
}

pub struct IpcChild {
    stdin: Stdin,
    stderr: Stderr,
}

impl IpcChild {
    pub fn new(stdin: Stdin, stderr: Stderr) -> Self {
        Self { stdin, stderr }
    }

    pub fn read(&mut self) -> Result<String> {
        let mut buffer = String::new();
        self.stdin.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    pub fn write(&mut self, message: &str) -> Result<()> {
        self.stderr.write_all(message.as_bytes())
    }
}
