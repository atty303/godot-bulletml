@tool
extends EditorPlugin

const EditorPanel = preload("res://addons/bulletml/Editor.tscn")

var inspector_plugin: EditorInspectorPlugin
var editor_panel

func _enter_tree():
	# Initialization of the plugin goes here.
	inspector_plugin = preload("res://addons/bulletml/inspector.gd").new()
	add_inspector_plugin(inspector_plugin)
	inspector_plugin.inspector = get_editor_interface().get_inspector()
	
	editor_panel = EditorPanel.instantiate()
	get_editor_interface().get_editor_main_screen().add_child(editor_panel)
	_make_visible(false)
	
	# var c = editor_panel.get_node("HBoxContainer")
	# c.add_child(get_editor_interface().get_script_editor())
	

func _exit_tree():
	# Clean-up of the plugin goes here.
	remove_inspector_plugin(inspector_plugin)
	
	if editor_panel:
		editor_panel.queue_free()


func _has_main_screen():
	return true


func _make_visible(visible):
	if editor_panel:
		editor_panel.visible = visible


func _get_plugin_name():
	return "BulletML Plugin"


func _get_plugin_icon():
	return get_editor_interface().get_base_control().get_theme_icon("Node", "EditorIcons")
