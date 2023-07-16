@tool
extends EditorInspectorPlugin

var inspector: EditorInspector


func _can_handle(object):
	return object.is_class("BulletMLPlayer")


func _parse_category(object, category):
	if category != "BulletMLPlayer":
		return

	var preview = preload("res://addons/bulletml/InspectorBulletMLPreview.tscn").instantiate()
	print(preview)
	# preview.the_inspector = inspector
	
	var w := ProjectSettings.get_setting("display/window/size/viewport_width", 1152)
	var h := ProjectSettings.get_setting("display/window/size/viewport_height", 648)
	
	var c := preview.get_node(".") as AspectRatioContainer
	c.inspector = inspector
	c.editing_player = object
	c.set_ratio(w as float / h as float)
	
	var viewport := preview.get_node("SubViewportContainer/SubViewport") as SubViewport
	viewport.set_size(Vector2i(w, h))
	
	var bullet_root := preview.get_node("SubViewportContainer/SubViewport/BulletRoot") as Node2D
	bullet_root.position = Vector2i(w / 2, h / 2)
	
	var player := preview.get_node("SubViewportContainer/SubViewport/BulletMLPlayer") as BulletMLPlayer
	player.set_bulletml(object.bulletml)
	player.play()
	
#	inspector.connect("resource_selected", resource_selected)
#	inspector.connect("edited_object_changed", edited_object_changed)
#	inspector.connect("property_edited", property_edited)
	
	add_custom_control(preview)

func resource_selected(resource: Resource, path: String):
	print("hoge")
	print(path)

func edited_object_changed():
	print("edited_object_changed")

func property_edited(path: String):
	if path == "bulletml":
		pass
	
