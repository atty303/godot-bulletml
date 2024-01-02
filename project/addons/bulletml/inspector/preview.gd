@tool
extends VBoxContainer

@export var bulletml: BulletML:
	set(value):
		bulletml = value
		if player:
			player.bulletml = value
			player.play()

#@onready var grabber: TextureRect = $Grabber
@onready var viewport = $SubViewportContainer
@onready var player = $SubViewportContainer/SubViewport/BulletMLPlayer


# Called when the node enters the scene tree for the first time.
func _ready():
	if bulletml:
		player.bulletml = bulletml
		player.play()


func _on_grabber_dragged(offset: int):
	var h = viewport.custom_minimum_size.y
	viewport.custom_minimum_size = Vector2i(64, h + offset)
