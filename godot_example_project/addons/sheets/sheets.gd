@tool
extends EditorPlugin

const SHEET_MANAGER_NAME: StringName = "SheetManager"

func _enable_plugin() -> void:
	# Add autoloads here.
	add_autoload_singleton(SHEET_MANAGER_NAME, "res://addons/sheets/script/sheet_manager.gd")
	


func _disable_plugin() -> void:
	# Remove autoloads here.
	remove_autoload_singleton(SHEET_MANAGER_NAME)


func _enter_tree() -> void:
	pass


func _exit_tree() -> void:
	# Clean-up of the plugin goes here.
	pass
