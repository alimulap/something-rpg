extends Sprite2D

@export var tilemap: TileMapLayer
@export var speed = 75
@export var snap_threshold = 2.1

var state = State.Idle
var target_pos: Vector2
var counter = 0


func _ready() -> void:
	self.position = get_snapped_pos()
	pass


func _process(_delta: float) -> void:
	# counter += 1
	# if counter % 30 == 0:
	# 	var cells = self.tilemap.get_used_cells()
	# 	var tile = cells[0]
	# 	var atlas_coord = self.tilemap.get_cell_atlas_coords(tile)
	# 	print(tile, " ", cells.size(), " ", atlas_coord)
	pass


func _physics_process(delta: float) -> void:
	handle_movement(delta)


func handle_movement(delta: float):
	match state:
		State.Idle:
			var right_pressed = Input.is_action_pressed("move_right")
			var left_pressed = Input.is_action_pressed("move_left")
			var up_pressed = Input.is_action_pressed("move_up")
			var down_pressed = Input.is_action_pressed("move_down")

			var directions = []

			if right_pressed:
				directions.append(MoveDirection.Right)
			if left_pressed:
				directions.append(MoveDirection.Left)
			if up_pressed:
				directions.append(MoveDirection.Up)
			if down_pressed:
				directions.append(MoveDirection.Down)

			if directions.size() > 0:
				var random_direction = directions[randi_range(0, directions.size() - 1)]
				move(random_direction)
				self.state = State.Walk
		State.Walk:
			self.position = self.position.move_toward(self.target_pos, self.speed * delta)
			if self.position.distance_to(self.target_pos) < self.snap_threshold:
				self.position = self.target_pos
				self.state = State.Idle


func move(direction: MoveDirection):
	var velocity: Vector2
	match direction:
		MoveDirection.Right:
			velocity = Vector2(1, 0)
		MoveDirection.Left:
			velocity = Vector2(-1, 0)
		MoveDirection.Up:
			velocity = Vector2(0, -1)
		MoveDirection.Down:
			velocity = Vector2(0, 1)
	self.target_pos = get_snapped_pos() + velocity * Global.tile_size


func get_snapped_pos() -> Vector2:
	var coord = self.tilemap.local_to_map(self.position)
	return self.tilemap.map_to_local(coord)


enum State {
	Idle,
	Walk,
}

enum MoveDirection { Right, Up, Left, Down }
