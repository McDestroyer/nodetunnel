use godot::global::godot_print;
use renet::{ConnectionConfig, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};
use std::error::Error;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Instant, SystemTime};

pub struct RenetHandler {
    client: Option<RenetClient>,
    transport: Option<NetcodeClientTransport>,
    last_updated: Instant,
}

impl RenetHandler {
    pub fn new() -> Self {
        Self {
            client: None,
            transport: None,
            last_updated: Instant::now(),
        }
    }

    pub fn create_client(&mut self, server_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        let socket = UdpSocket::bind("127.0.0.1:0")?;

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let client_id = current_time.as_millis() as u64;

        let authentication = ClientAuthentication::Unsecure {
            server_addr,
            client_id,
            user_data: None,
            protocol_id: 0,
        };

        let client = RenetClient::new(ConnectionConfig::default());
        let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

        self.client = Some(client);
        self.transport = Some(transport);
        self.last_updated = Instant::now();

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        if let (Some(client), Some(transport)) = (self.client.as_mut(), self.transport.as_mut()) {
            client.update(duration);
            transport.update(duration, client)?;

            if client.is_connected() {
                godot_print!("Connected!");
            }
        }

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.client.as_ref().map_or(false, |c| c.is_connected())
    }
}