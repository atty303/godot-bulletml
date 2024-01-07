@tool
extends EditorInspectorPlugin

signal config_changed

var config: ConfigFile = null


func _can_handle(object):
    return object.is_class("BulletML")


func _parse_begin(object) -> void:
    assert(config, "config is not set")

    var bulletml = object as BulletML
    if not bulletml:
        return

    var preview = preload("res://addons/bulletml/inspector/preview.tscn").instantiate()
    add_custom_control(preview)

    preview.config = config
    preview.config_changed.connect(func(): config_changed.emit())
    preview.bulletml = bulletml
