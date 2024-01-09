extends Node2D

@onready var player = $BulletMLPlayer
@onready var canvas = $BulletMLCanvas
@onready var count = $BulletCountLabel

# Called when the node enters the scene tree for the first time.
func _ready():
	canvas.create(10000)
	player.play()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	count.text = "count: {0} FPS: {1}".format([canvas.get_count(), str(Engine.get_frames_per_second())])

