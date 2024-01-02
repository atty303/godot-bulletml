@tool
extends EditorPlugin

const EditorPanel = preload("res://addons/bulletml/Editor.tscn")

var inspector_plugin: EditorInspectorPlugin
var preview_inspector_plugin: EditorInspectorPlugin
var editor_panel
var code_edit: CodeEdit

func _enter_tree():
	preview_inspector_plugin = preload("res://addons/bulletml/inspector/main.gd").new()
	add_inspector_plugin(preview_inspector_plugin)
	
	# Initialization of the plugin goes here.
	inspector_plugin = preload("res://addons/bulletml/inspector.gd").new()
	add_inspector_plugin(inspector_plugin)
	inspector_plugin.inspector = get_editor_interface().get_inspector()
	
	editor_panel = EditorPanel.instantiate()
	get_editor_interface().get_editor_main_screen().add_child(editor_panel)
	_make_visible(false)
	
	code_edit = editor_panel.find_child("CodeEdit") # get_node
	
	connect("resource_saved", resource_saved)
	
	var s = get_editor_interface().get_editor_settings()
	print(s.get("shortcuts"))
	# var c = editor_panel.get_node("HBoxContainer")
	# c.add_child(get_editor_interface().get_script_editor())
	

func _exit_tree():
	remove_inspector_plugin(preview_inspector_plugin)
	remove_inspector_plugin(inspector_plugin)
	
	if editor_panel:
		editor_panel.queue_free()


func _has_main_screen():
	return true


func _make_visible(visible):
	if editor_panel:
		editor_panel.visible = visible


func _get_plugin_name():
	return "BulletML"


func _get_plugin_icon():
	return get_editor_interface().get_base_control().get_theme_icon("Node", "EditorIcons")


func _edit(object):
	if object is BulletML and code_edit:
		print("_edit")
		var file = FileAccess.open(object.resource_path, FileAccess.READ)
		code_edit.text = file.get_as_text(true)


func _handles(object):
	return object is BulletML


func resource_saved(r):
	print("resource_saved")
