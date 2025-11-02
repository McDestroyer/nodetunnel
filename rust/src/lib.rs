mod node_tunnel_peer;
pub mod renet_handler;

use godot::prelude::*;

struct NodeTunnel;

#[gdextension]
unsafe impl ExtensionLibrary for NodeTunnel {}
