extends EditorImportPlugin

var bulletml_plugin = BulletMLPlugin.new()


func _get_importer_name():
	return "io.github.atty303.godot-bulletml"

func _get_visible_name():
	return "BulletML"

func _get_recognized_extensions():
	return ["xml"]

func _get_save_extension():
	return "bml"

func _get_resource_type():
	return "BulletML"

func _get_priority():
	return 1.0

func _get_import_order():
	return 0

func _get_import_options(path, preset_index):
	return []

func _import(source_file, save_path, options, platform_variants, gen_files):
	print(source_file)
	print(save_path)
	
	var bml = bulletml_plugin.load_xml(source_file)
	print(bml)
	
	var filename = save_path + "." + _get_save_extension()
	#ResourceSaver.save()
