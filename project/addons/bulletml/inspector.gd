@tool
extends EditorInspectorPlugin


func _can_handle(object):
	return object.is_class("BulletML")


func _parse_begin(object):
	var preview := preload("res://addons/bulletml/InspectorBulletMLPreview.tscn").instantiate()
	var c := preview.get_node(".") as AspectRatioContainer
	var w := ProjectSettings.get_setting("display/window/size/viewport_width", 1152)
	var h := ProjectSettings.get_setting("display/window/size/viewport_height", 648)
	c.set_ratio(w as float / h as float)
	
	var viewport := preview.get_node("SubViewportContainer/SubViewport") as SubViewport
	viewport.set_size(Vector2i(w, h))
	
	var bulletml := preview.get_node("SubViewportContainer/SubViewport/BulletMLPlayer") as BulletMLPlayer
	bulletml.set_file(object)
	
	add_custom_control(preview)
