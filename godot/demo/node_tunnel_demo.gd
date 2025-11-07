extends Node2D

@onready var host_id = $UI/HostID

var peer: NodeTunnelPeer

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("ui_accept"):
		network_print.rpc("Hello world")

@rpc("any_peer", "reliable")
func network_print(msg: String):
	print("Message For ", multiplayer.get_unique_id(), ": ", msg)

func _on_host_pressed() -> void:
	push_warning("HOST")
	peer = NodeTunnelPeer.new()
	peer.host_room("127.0.0.1:8080")
	multiplayer.multiplayer_peer = peer
	
	peer.room_connected.connect(
		func(room_id: String):
			DisplayServer.clipboard_set(room_id)
	)
	
	$UI.hide()

func _on_join_pressed() -> void:
	push_warning("JOIN")
	peer = NodeTunnelPeer.new()
	peer.join_room("127.0.0.1:8080", host_id.text)
	multiplayer.multiplayer_peer = peer
	
	$UI.hide()
