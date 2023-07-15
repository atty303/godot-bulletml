@tool
extends EditorPlugin

const EditorPanel = preload("res://addons/bulletml/Editor.tscn")

var dock
var editor_panel


func _enter_tree():
	# Initialization of the plugin goes here.
	editor_panel = EditorPanel.instantiate()
	get_editor_interface().get_editor_main_screen().add_child(editor_panel)
	_make_visible(false)
	
	var c = editor_panel.get_node("HBoxContainer")
	c.add_child(get_editor_interface().get_script_editor())
	
	dock = preload("res://addons/bulletml/Preview.tscn").instantiate()
	add_control_to_dock(DOCK_SLOT_RIGHT_BL, dock)


func _exit_tree():
	# Clean-up of the plugin goes here.
	if editor_panel:
		editor_panel.queue_free()

	remove_control_from_docks(dock)
	dock.free()


func _has_main_screen():
	return true


func _make_visible(visible):
	if editor_panel:
		editor_panel.visible = visible


func _get_plugin_name():
	return "BulletML Plugin"


func _get_plugin_icon():
	return get_editor_interface().get_base_control().get_theme_icon("Node", "EditorIcons")
