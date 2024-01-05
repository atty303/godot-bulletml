@tool
extends EditorInspectorPlugin

signal preview_height_changed(height: int)
var preview_height: int = 32


func _can_handle(object):
	return object.is_class("BulletML")


func _parse_begin(object) -> void:
	var bulletml = object as BulletML
	if not bulletml:
		return

	var preview = preload("res://addons/bulletml/inspector/preview.tscn").instantiate()
	add_custom_control(preview)

	preview.connect("viewport_height_changed", _on_viewport_height_changed)
	preview.bulletml = bulletml
	preview.set_viewport_height(preview_height)


func _on_viewport_height_changed(height):
	preview_height = height
	preview_height_changed.emit(height)


func set_preview_height(height: int):
	preview_height = height
