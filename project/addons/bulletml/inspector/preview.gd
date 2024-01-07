@tool
extends VBoxContainer

signal viewport_height_changed(height: int)

@export var bulletml: BulletML:
    set(value):
        bulletml = value
        if player:
            player.bulletml = value
            player.play()

@onready var viewport = $SubViewportContainer
@onready var player = $SubViewportContainer/SubViewport/BulletMLPlayer
@onready var turn_label: Label = %TurnLabel


func _ready():
    if bulletml:
        player.bulletml = bulletml
        player.play()


func _physics_process(delta: float) -> void:
    if player.is_playing():
        turn_label.text = str(player.get_turn())
    else:
        turn_label.text = "-"


func _on_grabber_dragged(offset: int):
    var h = viewport.custom_minimum_size.y + offset
    viewport.custom_minimum_size = Vector2i(64, h)
    viewport_height_changed.emit(h)


func set_viewport_height(height: int):
    await ready
    viewport.custom_minimum_size = Vector2i(64, height)


func _on_play_button_pressed():
    player.stop()
    player.clear()
    player.play()
