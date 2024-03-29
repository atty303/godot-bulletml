use std::sync::{Arc, OnceLock};
use godot::engine::{FileAccess, IResourceFormatLoader, ResourceFormatLoader};
use godot::engine::global::Error;
use godot::prelude::*;

pub(crate) fn get_empty_bulletml() -> &'static Arc<bulletml::BulletML> {
    static CELL: OnceLock<Arc<bulletml::BulletML>> = OnceLock::new();
    CELL.get_or_init(|| Arc::new(bulletml::parse::BulletMLParser::new().parse("<bulletml/>").unwrap()))
}

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct BulletMLResource {
    #[base]
    base: Base<Resource>,

    pub bml: Arc<bulletml::BulletML>,
}

#[godot_api]
impl BulletMLResource {
}

#[godot_api]
impl IResource for BulletMLResource {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            bml: get_empty_bulletml().clone(),
        }
    }
}

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader)]
pub struct BulletMLResourceFormatLoader {
    #[base]
    base: Base<ResourceFormatLoader>,

    refs_capacity: usize,
    expr_capacity: usize,
}

#[godot_api]
impl BulletMLResourceFormatLoader {
    pub fn new(base: Base<ResourceFormatLoader>, refs_capacity: usize, expr_capacity: usize) -> Self {
        Self {
            base,
            refs_capacity,
            expr_capacity,
        }
    }
}

#[godot_api]
impl IResourceFormatLoader for BulletMLResourceFormatLoader {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            refs_capacity: 1024,
            expr_capacity: 1024,
        }
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("xml"), GString::from("bulletml")])
    }

    // fn recognize_path(&self, path: GodotString, type_: StringName) -> bool {
    //     type_ == StringName::from("BulletMLFile")
    // }

    fn handles_type(&self, type_: StringName) -> bool {
        type_ == StringName::from("BulletMLResource")
    }

    fn get_resource_type(&self, _path: GString) -> GString {
        GString::from("BulletMLResource")
    }

    // fn get_resource_script_class(&self, path: GodotString) -> GodotString {
    //     GodotString::from("")
    // }

    // fn get_resource_uid(&self, path: GodotString) -> i64 {
    //     0
    // }

    // fn get_dependencies(&self, path: GodotString, add_types: bool) -> PackedStringArray {
    //     PackedStringArray::new()
    // }
    //
    // fn rename_dependencies(&self, path: GodotString, renames: Dictionary) -> Error {
    //     Error::OK
    // }

    // fn exists(&self, path: GodotString) -> bool {
    //     true
    // }

    // fn get_classes_used(&self, path: GString) -> PackedStringArray {
    //     PackedStringArray::from(&[GString::from("BulletML")])
    // }

    fn load(&self, path: GString, _original_path: GString, _use_sub_threads: bool, _cache_mode: i32) -> Variant {
        godot_print!("Loading BulletML file at {}", path);
        let body = FileAccess::get_file_as_string(path.clone());
        let parser = bulletml::parse::BulletMLParser::with_capacities(self.refs_capacity, self.expr_capacity);
        match parser.parse(body.to_string().as_str()) {
            Ok(bml) => {
                let bulletml = Gd::<BulletMLResource>::from_init_fn(|base| BulletMLResource { base, bml: Arc::new(bml) });
                godot_print!("Loaded BulletML file: {:?}", bulletml);
                Variant::from(bulletml)
            }
            Err(err) => {
                godot_error!("Failed to parse BulletML file at {}: {:?}", path, err);
                Variant::from(Error::ERR_INVALID_DATA)
            }
        }
    }
}

