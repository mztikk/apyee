use crate::{
    command::{Command, CommandResponse, NotificationResult},
    method::Method,
};
use rand::Rng;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::{atomic::AtomicI32, Arc},
};
use thiserror::Error;
use tokio::{io, sync::Mutex};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Notify};

/// Default Port of Yeelight Bulbs
pub const DEFAULT_PORT: u16 = 55443;

/// Errors that can occur when interacting with a Yeelight Bulb
#[derive(Error, Debug)]
pub enum DeviceError {
    /// Error when connecting or sending packets to the Yeelight Bulb
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Error when parsing a packet from the Yeelight Bulb
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Error when a response times out
    #[error(transparent)]
    Timeout(#[from] tokio::time::error::Elapsed),
    #[error(transparent)]
    /// Error when a response contains invalid utf8
    Utf8(#[from] std::str::Utf8Error),
}

struct UniqueCommandId {
    id: AtomicI32,
}

impl UniqueCommandId {
    fn new() -> Self {
        let rand = rand::thread_rng().gen_range(15..1500);
        Self {
            id: AtomicI32::new(rand),
        }
    }

    fn next(&self) -> i32 {
        self.id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

struct Responses {
    responses: HashMap<i32, CommandResponse>,
}

impl Responses {
    fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }

    fn add(&mut self, response: CommandResponse) {
        self.responses.insert(response.id, response);
    }

    fn consume(&mut self, id: i32) -> Option<CommandResponse> {
        self.responses.remove(&id)
    }
}

/// A Yeelight device.
pub struct Device {
    /// The IP address of the device.
    pub ip: IpAddr,
    /// The port of the device.
    pub port: u16,
    responses: Arc<Mutex<Responses>>,
    tcp_stream: Arc<Mutex<TcpStream>>,
    command_id: UniqueCommandId,
    notify: Arc<Notify>,
}

type ExecutionResult = Result<CommandResponse, DeviceError>;
type DeviceResult = Result<Device, DeviceError>;

impl Device {
    /// Creates a new device with ip and port.
    /// The device will connect to the device at the given IP address and port.
    /// If the connection fails, the function will return an error.
    /// The device will also start listening for responses from the device.
    ///
    /// # Arguments
    /// * `ip` - The IP address of the device.
    /// * `port` - The port of the device.
    ///
    /// # Errors
    /// * `DeviceError::Io` - If the connection fails.
    ///
    /// # Examples
    /// ```no_run
    /// use apyee::device::Device;
    /// use std::net::IpAddr;
    /// async {
    ///     let device = Device::new_with_port("127.0.0.1".parse().unwrap(), 55443).await.unwrap();
    /// };
    /// ```
    pub async fn new_with_port(ip: IpAddr, port: u16) -> DeviceResult {
        let stream = TcpStream::connect(SocketAddr::new(ip, port)).await?;
        let stream = Arc::new(Mutex::new(stream));
        let stream_clone = Arc::clone(&stream);

        let responses = Arc::new(Mutex::new(Responses::new()));
        let responses_clone = Arc::clone(&responses);

        let notify = Arc::new(Notify::new());
        let notify_clone = Arc::clone(&notify);

        tokio::spawn(Self::listen_responses_console_error(
            stream_clone,
            responses_clone,
            notify_clone,
        ));

        let device = Self {
            ip,
            port,
            tcp_stream: stream,
            responses,
            command_id: UniqueCommandId::new(),
            notify,
        };

        Ok(device)
    }

    /// Creates a new device with ip and default port.
    /// The device will connect to the device at the given IP address and default port.
    /// If the connection fails, the function will return an error.
    /// The device will also start listening for responses from the device.
    ///
    /// # Arguments
    /// * `ip` - The IP address of the device.
    ///
    /// # Errors
    /// * `DeviceError::Io` - If the connection fails.
    ///
    /// # Examples
    /// ```no_run
    /// use apyee::device::Device;
    /// use std::net::IpAddr;
    /// async {
    ///     let device = Device::new("127.0.0.1".parse().unwrap()).await.unwrap();
    /// };
    /// ```
    pub async fn new(ip: IpAddr) -> DeviceResult {
        Self::new_with_port(ip, DEFAULT_PORT).await
    }

    /// Converts u8 RGB values into the i32 RGB format used by the Yeelight device.\
    /// The i32 RGB format is a 24-bit integer with the red, green, and blue values packed into a single integer.
    ///
    /// # Arguments
    /// * `r` - The red value.
    /// * `g` - The green value.
    /// * `b` - The blue value.
    pub const fn get_rgb_color(r: u8, g: u8, b: u8) -> i32 {
        (r as i32) << 16 | (g as i32) << 8 | (b as i32)
    }

    /// Sets the color of the device, given as separate u8 RGB values.
    ///
    /// # Arguments
    /// * `r` - The red value.
    /// * `g` - The green value.
    /// * `b` - The blue value.
    pub async fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> ExecutionResult {
        self.execute_method(Method::SetRgb(Self::get_rgb_color(r, g, b), None, None))
            .await
    }

    /// Sets the background color of the device, given as separate u8 RGB values.
    ///
    /// # Arguments
    /// * `r` - The red value.
    /// * `g` - The green value.
    /// * `b` - The blue value.
    pub async fn set_bg_rgb(&mut self, r: u8, g: u8, b: u8) -> ExecutionResult {
        self.execute_method(Method::BgSetRgb(Self::get_rgb_color(r, g, b), None, None))
            .await
    }

    /// Toggles the devices power state.
    /// If the device is on, it will be turned off.
    /// If the device is off, it will be turned on.
    pub async fn toggle(&mut self) -> ExecutionResult {
        self.execute_method(Method::Toggle).await
    }

    /// Sets the power state of the device to on.
    pub async fn power_on(&mut self) -> ExecutionResult {
        self.execute_method(Method::SetPower(true, None, None))
            .await
    }

    /// Sets the power state of the device to off.
    pub async fn power_off(&mut self) -> ExecutionResult {
        self.execute_method(Method::SetPower(false, None, None))
            .await
    }

    /// Executes a given [`Method`] on the device by creating a new command with a unique id.
    pub async fn execute_method(&mut self, method: Method) -> ExecutionResult {
        let command = Command::new(self.command_id.next(), method);

        self.execute_command(command).await
    }

    /// Executes a given [`Command`] on the device.
    pub async fn execute_command(&mut self, command: Command) -> ExecutionResult {
        // terminate every message with \r\n"
        let json = serde_json::to_string(&command)?;
        let json_command = format!("{}\r\n", json);

        self.tcp_stream
            .lock()
            .await
            .write_all(json_command.as_bytes())
            .await?;

        let result = tokio::time::timeout(std::time::Duration::from_secs(10), async {
            loop {
                tokio::time::timeout(std::time::Duration::from_secs(3), self.notify.notified())
                    .await?;

                if let Some(response) = self.responses.lock().await.consume(command.id) {
                    return Ok(response);
                }
            }
        })
        .await?;

        result
    }

    async fn listen_responses(
        tcp_stream: Arc<Mutex<TcpStream>>,
        responses: Arc<Mutex<Responses>>,
        notify: Arc<Notify>,
    ) -> Result<(), DeviceError> {
        loop {
            let mut buffer = [0u8; 8192];
            match tcp_stream.lock().await.try_read(&mut buffer) {
                Ok(0) => {
                    // if the connection is closed, return
                    return Ok(());
                }
                Ok(n) => {
                    // parse the json
                    let data = std::str::from_utf8(&buffer[..n])?;
                    let entries = data.split_terminator("\r\n");
                    for entry in entries {
                        // let response: CommandResponse = serde_json::from_str(entry)?;
                        // responses.lock().await.add(response);
                        // notify.notify_one();
                        if let Ok(response) = serde_json::from_str::<CommandResponse>(entry) {
                            responses.lock().await.add(response);
                            notify.notify_one();
                        };

                        if let Ok(response) = serde_json::from_str::<NotificationResult>(entry) {
                            // TODO: Save properies somewhere
                        }
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    async fn listen_responses_console_error(
        tcp_stream: Arc<Mutex<TcpStream>>,
        responses: Arc<Mutex<Responses>>,
        notify: Arc<Notify>,
    ) {
        match Self::listen_responses(tcp_stream, responses, notify).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
