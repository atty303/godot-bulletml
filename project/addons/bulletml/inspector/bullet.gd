@tool
extends Node2D

@onready var line = $Line2D

var rect = Rect2(-10, -10, 20, 20)
var on_screen = false
var prev_position: Vector2

func _enter_tree():
	on_screen = false
	RenderingServer.canvas_item_set_visibility_notifier(get_canvas_item(), true, rect, _on_visibility_enter, _on_visibility_exit)


func _ready():
	prev_position = position


func _on_visibility_enter():
	pass


func _on_visibility_exit():
	queue_free()


func _physics_process(delta):
	#var pos = get_parent().position
	var pos = position
	line.clear_points()
	line.add_point(prev_position - pos)
	line.add_point(Vector2(0, 0))
	prev_position = pos
