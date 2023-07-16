@tool
extends AspectRatioContainer

var editing_player: BulletMLPlayer
var preview_player: BulletMLPlayer
var inspector: EditorInspector

func _ready():
	print("ready")
	preview_player = get_node("SubViewportContainer/SubViewport/BulletMLPlayer")

func _enter_tree():
	print("enter_tree")
	inspector.connect("property_edited", property_edited)
	inspector.connect("restart_requested", restart_requested)
	inspector.connect("resource_selected", resource_selected)
	inspector.connect("edited_object_changed", edited_object_changed)

func _exit_tree():
	# inspector.disconnect("property_edited", property_edited)
	pass

func property_edited(path: String):
	print("property_edited: ", path)
	if path == "bulletml":
		preview_player.stop()
		preview_player.clear()
		preview_player.bulletml = editing_player.bulletml
		preview_player.play()

func restart_requested():
	print("restart_requested")

func resource_selected(r, p):
	print("resource_selected")

func edited_object_changed():
	print("edited_object_changed")
