extends Node2D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func on_reset_button_pressed() -> void:
	var dirt = get_node("dirt")
	dirt.queue_free()
	await dirt.tree_exited
	var scene: PGTilemap = load("res://scenes/dirt_layer.tscn").instantiate()
	add_child(scene)
