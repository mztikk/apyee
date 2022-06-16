use crate::{
    command::{Command, CommandResponse},
    method::Method,
};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc},
};
use thiserror::Error;
use tokio::io;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{Mutex, Notify},
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
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
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
    responses: HashMap<usize, CommandResponse>,
    notify: Notify,
}

impl Responses {
    fn new() -> Self {
        Self {
            responses: HashMap::new(),
            notify: Notify::new(),
        }
    }

    fn add(&mut self, id: usize, response: CommandResponse) {
        self.notify.notify_one();
        self.responses.insert(id, response);
    }

    fn consume(&mut self, id: usize) -> Option<CommandResponse> {
        self.responses.remove(&id)
    }

    async fn wait(&self) {
        self.notify.notified().await;
    }
}

pub struct Device {
    pub ip: String,
    pub port: u16,
    responses: Arc<Mutex<Responses>>,
    tcp_writer: OwnedWriteHalf,
    command_id: UniqueCommandId,
}

impl Device {
    pub async fn new_with_port(ip: String, port: u16) -> Result<Device, DeviceError> {
        let (read, write) = TcpStream::connect(format!("{}:{}", ip, port))
            .await?
            .into_split();

        let responses = Arc::new(Mutex::new(Responses::new()));
        let responses_clone = Arc::clone(&responses);

        let device = Self {
            tcp_writer: write,
            ip,
            port,
            responses,
            command_id: UniqueCommandId::new(),
        };

        tokio::spawn(Self::listen_responses_console_error(read, responses_clone));

        Ok(device)
    }

    pub async fn new(ip: String) -> Result<Device, DeviceError> {
        Self::new_with_port(ip, DEFAULT_PORT).await
    }

    pub const fn get_rgb_color(r: u8, g: u8, b: u8) -> i32 {
        (r as i32) << 16 | (g as i32) << 8 | (b as i32)
    }

    pub async fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<CommandResponse, DeviceError> {
        let command = Command::new(
            self.command_id.next(),
            Method::SetRgb(Self::get_rgb_color(r, g, b)),
        );

        self.execute_command(command).await
    }

    pub async fn set_bg_rgb(
        &mut self,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<CommandResponse, DeviceError> {
        let command = Command::new(
            self.command_id.next(),
            Method::BgSetRgb(Self::get_rgb_color(r, g, b)),
        );

        self.execute_command(command).await
    }

    pub async fn toggle(&mut self) -> Result<CommandResponse, DeviceError> {
        let command = Command::new(self.command_id.next(), Method::Toggle);

        self.execute_command(command).await
    }

    pub async fn power_on(&mut self) -> Result<CommandResponse, DeviceError> {
        let command = Command::new(self.command_id.next(), Method::SetPower(true));

        self.execute_command(command).await
    }

    pub async fn power_off(&mut self) -> Result<CommandResponse, DeviceError> {
        let command = Command::new(self.command_id.next(), Method::SetPower(false));

        self.execute_command(command).await
    }

    pub async fn execute_method(&mut self, method: Method) -> Result<CommandResponse, DeviceError> {
        let command = Command::new(self.command_id.next(), method);

        self.execute_command(command).await
    }

    pub async fn execute_command(
        &mut self,
        command: Command,
    ) -> Result<CommandResponse, DeviceError> {
        // terminate every message with \r\n"
        let json = format!("{}\r\n", serde_json::to_string(&command)?);
        self.tcp_writer.write_all(json.as_bytes()).await?;

        let result = tokio::time::timeout(std::time::Duration::from_secs(2), async move {
            let mut responses = self.responses.lock().await;

            // check if we already have a response for this command
            if let Some(response) = responses.consume(command.id) {
                return Some(response);
            }

            // otherwise wait for a new response
            loop {
                responses.wait().await;

                if let Some(response) = responses.consume(command.id) {
                    return Some(response);
                }
            }
        })
        .await?;

        Ok(unsafe { result.unwrap_unchecked() })
    }

    async fn listen_responses(
        reader: OwnedReadHalf,
        responses: Arc<Mutex<Responses>>,
    ) -> Result<(), DeviceError> {
        let mut buffer = [0u8; 1024];
        loop {
            // wait for data to become available and readable
            if (reader.readable().await).is_err() {
                continue;
            }

            // read the data
            match reader.try_read(&mut buffer) {
                Ok(n) => {
                    // parse the json
                    let data = std::str::from_utf8(&buffer[..n])?;
                    let response: CommandResponse = serde_json::from_str(data)?;
                    responses.lock().await.add(response.id, response);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    async fn listen_responses_console_error(
        reader: OwnedReadHalf,
        responses: Arc<Mutex<Responses>>,
    ) {
        match Self::listen_responses(reader, responses).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
