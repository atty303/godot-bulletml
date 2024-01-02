@tool
extends EditorInspectorPlugin


func _can_handle(object):
	return object.is_class("BulletML")


func _parse_begin(object):
	var bulletml = object as BulletML
	if not bulletml:
		return
	
	var preview = preload("res://addons/bulletml/inspector/preview.tscn").instantiate()
	add_custom_control(preview)
	
	preview.bulletml = bulletml
