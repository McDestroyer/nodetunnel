extends Node2D

@export var nodetunnel_config: NodeTunnelConfig

@onready var host_id = $UI/HostID
@onready var peer := NodeTunnelPeer.new()

func _ready() -> void:
	peer.config = nodetunnel_config
	peer.connect_to_relay()

func _on_host_pressed() -> void:
	peer.host_room()
	multiplayer.multiplayer_peer = peer
	
	peer.room_connected.connect(
		func(room_id: String):
			DisplayServer.clipboard_set(room_id)
	)
	
	$UI.hide()

func _on_join_pressed() -> void:
	peer.join_room(host_id.text)
	multiplayer.multiplayer_peer = peer
	
	$UI.hide()

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("ui_accept"):
		network_print.rpc("Hello world")

@rpc("any_peer", "reliable")
func network_print(msg: String):
	print("Message For ", multiplayer.get_unique_id(), ": ", msg)
