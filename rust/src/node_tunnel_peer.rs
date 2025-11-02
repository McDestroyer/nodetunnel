use std::net::SocketAddr;
use godot::builtin::PackedByteArray;
use godot::prelude::{godot_api, GodotClass, ToGodot};
use godot::classes::{IMultiplayerPeerExtension, MultiplayerPeerExtension};
use godot::classes::multiplayer_peer::{ConnectionStatus, TransferMode};
use godot::global::{godot_print, Error};
use godot::obj::{Base, WithBaseField, WithUserSignals};
use crate::renet_handler::RenetHandler;

struct Packet {}

#[derive(GodotClass)]
#[class(tool, base=MultiplayerPeerExtension)]
struct NodeTunnelPeer {
    unique_id: i32,
    connection_status: ConnectionStatus,

    target_peer: i32,

    incoming_packets: Vec<Packet>,
    current_packet: Option<Packet>,

    renet_handler: Option<RenetHandler>,

    base: Base<MultiplayerPeerExtension>
}

#[godot_api]
impl IMultiplayerPeerExtension for NodeTunnelPeer {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            unique_id: 0,
            connection_status: ConnectionStatus::DISCONNECTED,
            target_peer: 0,
            incoming_packets: vec![],
            current_packet: None,
            renet_handler: None,
            base,
        }
    }

    fn get_available_packet_count(&self) -> i32 {
        self.incoming_packets.len() as i32
    }

    fn get_max_packet_size(&self) -> i32 {
        1 << 24
    }

    fn put_packet_script(&mut self, p_buffer: PackedByteArray) -> Error {
        godot_print!("PACKET");
        godot_print!("{}", p_buffer.to_string());
        Error::OK
    }

    fn get_packet_channel(&self) -> i32 {
        0
    }

    fn get_packet_mode(&self) -> TransferMode {
        TransferMode::RELIABLE
    }

    fn set_transfer_channel(&mut self, p_channel: i32) {

    }

    fn get_transfer_channel(&self) -> i32 {
        0
    }

    fn set_transfer_mode(&mut self, p_mode: TransferMode) {

    }

    fn get_transfer_mode(&self) -> TransferMode {
        TransferMode::RELIABLE
    }

    fn set_target_peer(&mut self, p_peer: i32) {
        self.target_peer = p_peer;
    }

    fn get_packet_peer(&self) -> i32 {
        1
    }

    fn is_server(&self) -> bool {
        self.unique_id == 1
    }

    fn poll(&mut self) {
        if let Some(handler) = &mut self.renet_handler {
            if let Err(e) = handler.update() {
                godot_print!("Renet error: {}", e);
            }
        }
    }

    fn close(&mut self) {

    }

    fn disconnect_peer(&mut self, p_peer: i32, p_force: bool) {

    }

    fn get_unique_id(&self) -> i32 {
        self.unique_id
    }

    fn get_connection_status(&self) -> ConnectionStatus {
        self.connection_status
    }
}

#[godot_api]
impl NodeTunnelPeer {
    #[func]
    fn host_room(&mut self, relay_addr: String) {
        if let Ok(addr) = relay_addr.parse::<SocketAddr>() {
            let mut handler = RenetHandler::new();
            if handler.create_client(addr).is_ok() {
                godot_print!("created renet client");
                self.renet_handler = Some(handler);
                self.unique_id = 1;
                self.connection_status = ConnectionStatus::CONNECTING;
            }
        }
    }

    #[func]
    fn add_dummy(&mut self, id: i64) {
        self.signals().peer_connected().emit(id);
    }

    #[func]
    fn join_room(&mut self, server_addr: String) {

    }
}
