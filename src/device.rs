use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc},
};

use thiserror::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::Mutex,
};

use crate::{
    command::{Command, CommandResult},
    method::Method,
};

pub const DEFAULT_PORT: u16 = 55443;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Timeout(#[from] tokio::time::error::Elapsed),
}

struct UniqueCommandId {
    id: AtomicUsize,
}

impl UniqueCommandId {
    fn new() -> Self {
        Self {
            id: AtomicUsize::new(0),
        }
    }

    fn next(&self) -> usize {
        self.id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

struct Responses {
    responses: HashMap<usize, CommandResult>,
}

impl Responses {
    fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }

    fn add(&mut self, id: usize, response: CommandResult) {
        self.responses.insert(id, response);
    }

    fn consume(&mut self, id: usize) -> Option<CommandResult> {
        self.responses.remove(&id)
    }
}

pub struct Device {
    pub ip: String,
    pub port: u16,
    responses: Arc<Mutex<Responses>>,
    tcp_writer: WriteHalf<TcpStream>,
    command_id: UniqueCommandId,
}

impl Device {
    pub async fn new_with_port(ip: String, port: u16) -> Result<Device, DeviceError> {
        let tcp_stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;
        let (read, write) = tokio::io::split(tcp_stream);
        let responses = Arc::new(Mutex::new(Responses::new()));
        let responses_clone = Arc::clone(&responses);
        let device = Device {
            tcp_writer: write,
            ip,
            port,
            responses,
            command_id: UniqueCommandId::new(),
        };

        tokio::spawn(Device::listen_responses(read, responses_clone));

        Ok(device)
    }

    pub const fn get_rgb_color(&self, r: u8, g: u8, b: u8) -> i32 {
        (r as i32) << 16 | (g as i32) << 8 | (b as i32)
    }

    pub async fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<CommandResult, DeviceError> {
        let command = Command::new(
            self.command_id.next(),
            Method::SetRgb(self.get_rgb_color(r, g, b)),
        );

        self.execute_command(command).await
    }

    pub async fn execute_method(&mut self, method: Method) -> Result<CommandResult, DeviceError> {
        let command = Command::new(self.command_id.next(), method);

        self.execute_command(command).await
    }

    pub async fn execute_command(
        &mut self,
        command: Command,
    ) -> Result<CommandResult, DeviceError> {
        // terminate every message with \r\n"
        let json = format!("{}\r\n", serde_json::to_string(&command)?);
        self.tcp_writer.write_all(json.as_bytes()).await?;

        let result = tokio::time::timeout(std::time::Duration::from_secs(2), async move {
            // wait for response listener with new response of our id
            self.responses.lock().await.consume(command.id).unwrap()
        })
        .await?;

        Ok(result)
    }

    async fn listen_responses(
        mut reader: ReadHalf<TcpStream>,
        responses: Arc<Mutex<Responses>>,
    ) -> Result<(), DeviceError> {
        let mut buffer = [0u8; 1024];
        let mut response = String::new();
        loop {
            let n = reader.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            response.push_str(&String::from_utf8_lossy(&buffer[..n]));
        }

        Ok(())
    }
}
