@tool
extends VBoxContainer

signal config_changed

const CONFIG_SECTION = "preview"

@export var bulletml: BulletMLResource:
	set(value):
		bulletml = value
		if player:
			player.bulletml = value
			player.play()

@onready var viewport = $SubViewportContainer
@onready var player = %BulletMLPlayer
@onready var bullet_counter_label: Label = %BulletCounterLabel
@onready var bullet_canvas: BulletMLCanvas = %BulletMLCanvas
@onready var turn_label: Label = %TurnLabel

var config: ConfigFile = null


func _ready():
	assert(config, "config is not set")

	viewport.custom_minimum_size = config.get_value(CONFIG_SECTION, "viewport_size", Vector2i(64, 64))
	viewport.gui_input.connect(_on_sub_viewport_container_gui_input)

	bullet_canvas.create(6000)
	# bullet_root.position = config.get_value(CONFIG_SECTION, "bullet_root_position", Vector2(0, 0))

	if bulletml:
		player.bulletml = bulletml
		player.play()


func _process(delta: float) -> void:
	bullet_counter_label.text = str(bullet_canvas.get_count())
	pass

func _physics_process(delta: float) -> void:
	if bullet_canvas:
		turn_label.text = str(bullet_canvas.get_turn())
	else:
		turn_label.text = "-"


func _on_grabber_dragged(offset: int):
	var size = Vector2i(viewport.custom_minimum_size.x, viewport.custom_minimum_size.y + offset)
	viewport.custom_minimum_size = size
	config.set_value(CONFIG_SECTION, "viewport_size", size)
	config_changed.emit()


func _on_play_button_pressed():
	player.stop()
	player.reset()
	player.play()


func _on_sub_viewport_container_gui_input(event: InputEvent):
	if event is InputEventMouseMotion or event is InputEventMouseButton:
		if event.button_mask & MOUSE_BUTTON_MASK_RIGHT:
			# bullet_root.position = event.position
			config.set_value(CONFIG_SECTION, "bullet_root_position", event.position)
			config_changed.emit()
