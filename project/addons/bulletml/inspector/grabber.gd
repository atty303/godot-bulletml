@tool
extends TextureRect

var dragging = false
var drag_from = 0

signal dragged(offset: int)


# Called when the node enters the scene tree for the first time.
func _ready():
	texture = get_theme_icon("v_grabber", "SplitContainer")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _gui_input(event: InputEvent):
	var mb = event as InputEventMouseButton
	if mb and mb.button_index == MOUSE_BUTTON_LEFT:
		if mb.is_pressed():
			dragging = true
			drag_from = get_transform().basis_xform(mb.position).y
		else:
			dragging = false
			queue_redraw()

	var mm = event as InputEventMouseMotion
	if mm and dragging:
		var in_parent_pos = get_transform().basis_xform(mm.position)
		var offset = in_parent_pos.y - drag_from
		emit_signal("dragged", offset)
