extends Node2D

var peer: NodeTunnelPeer
@onready var enet_peer = ENetMultiplayerPeer.new()

func _ready() -> void:
	peer = NodeTunnelPeer.new()
	peer.host_room("127.0.0.1:8080")
	multiplayer.multiplayer_peer = peer
	
	multiplayer.peer_connected.connect(
		func(pid: int):
			print(pid, " has joined.")
	)

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("ui_cancel"):
		peer.add_dummy(2)
	
	if Input.is_action_just_pressed("ui_accept"):
		print("Send RPC ", multiplayer.get_peers().size())
		network_print.rpc_id(2, "Hello world")

@rpc("authority", "reliable")
func network_print(msg: String):
	print(msg)
