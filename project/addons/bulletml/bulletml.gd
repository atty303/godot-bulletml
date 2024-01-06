@tool
extends EditorPlugin

const EditorPanel = preload("res://addons/bulletml/Editor.tscn")
const CONFIG_SECTION: String = "BulletMLPlugin"
const CONFIG_LAYOUT: String = "layout"
var preview_inspector_plugin: EditorInspectorPlugin
var editor_panel
var code_edit: CodeEdit
var preview_height: int = 0
var config = ConfigFile.new()


func _enter_tree():
	_load_config()

	preview_height = config.get_value(CONFIG_LAYOUT, "preview_height", 128)

	preview_inspector_plugin = preload("res://addons/bulletml/inspector/main.gd").new()
	add_inspector_plugin(preview_inspector_plugin)
	preview_inspector_plugin.connect("preview_height_changed", _on_preview_height_changed)
	preview_inspector_plugin.set_preview_height(preview_height)

	editor_panel = EditorPanel.instantiate()
	get_editor_interface().get_editor_main_screen().add_child(editor_panel)
	_make_visible(false)

	code_edit = editor_panel.find_child("CodeEdit") # get_node

	# var c = editor_panel.get_node("HBoxContainer")
	# c.add_child(get_editor_interface().get_script_editor())


func _exit_tree():
	if preview_inspector_plugin:
		remove_inspector_plugin(preview_inspector_plugin)
		preview_inspector_plugin = null

	if editor_panel:
		editor_panel.queue_free()


func _has_main_screen() -> bool:
	return true


func _make_visible(visible):
	if editor_panel:
		editor_panel.visible = visible


func _get_plugin_name():
	return "BulletML"


func _get_plugin_icon():
	return preload("res://addons/bulletml/bulletml-logo-16.png")


#func _edit(object):
#	if object is BulletML and code_edit:
#		print("_edit")
#		var file = FileAccess.open(object.resource_path, FileAccess.READ)
#		code_edit.text = file.get_as_text(true)


#func _handles(object):
#	return object is BulletML


func _on_preview_height_changed(height: int):
	preview_height = height
	config.set_value(CONFIG_LAYOUT, "preview_height", preview_height)
	_save_config()


func _get_config_path() -> String:
	var dir: String = EditorInterface.get_editor_paths().get_project_settings_dir()
	return dir.path_join("plugins/bulletml/settings.cfg")


func _load_config():
	var path = _get_config_path()
	if FileAccess.file_exists(path):
		config.load(path)
	else:
		DirAccess.make_dir_recursive_absolute(path.get_base_dir())
		config.save(path)


func _save_config():
	var path = _get_config_path()
	config.save(path)
