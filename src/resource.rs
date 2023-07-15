use std::ops::Deref;
use std::rc::Rc;

use bulletml::parse::BulletMLParser;
use godot::engine::{FileAccess, ResourceFormatLoader, ResourceFormatLoaderVirtual};
use godot::engine::global::Error;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct BulletMLFile {
    #[base]
    base: Base<Resource>,

    pub bml: Rc<bulletml::BulletML>,
}

#[godot_api]
impl BulletMLFile {
}

#[godot_api]
impl ResourceVirtual for BulletMLFile {
}


#[derive(GodotClass)]
#[class(base=ResourceFormatLoader, init)]
pub struct BulletMLResourceFormatLoader {
    #[base]
    base: Base<ResourceFormatLoader>,

    #[init(default = 1024)]
    refs_capacity: usize,

    #[init(default = 1024)]
    expr_capacity: usize,
}

#[godot_api]
impl BulletMLResourceFormatLoader {
    pub fn new(base: Base<ResourceFormatLoader>) -> Self {
        Self {
            base,
            refs_capacity: 1024,
            expr_capacity: 1024,
        }
    }
}

#[godot_api]
impl ResourceFormatLoaderVirtual for BulletMLResourceFormatLoader {
    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[GodotString::from("xml")])
    }

    // fn recognize_path(&self, path: GodotString, type_: StringName) -> bool {
    //     type_ == StringName::from("BulletMLFile")
    // }

    fn handles_type(&self, type_: StringName) -> bool {
        type_ == StringName::from("BulletMLFile")
    }

    fn get_resource_type(&self, _path: GodotString) -> GodotString {
        GodotString::from("BulletMLFile")
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

    // fn get_classes_used(&self, path: GodotString) -> PackedStringArray {
    //     PackedStringArray::from(&[GodotString::from("BulletMLFile")])
    // }

    fn load(&self, path: GodotString, original_path: GodotString, use_sub_threads: bool, cache_mode: i32) -> Variant {
        let body = FileAccess::get_file_as_string(path.clone());
        let parser = BulletMLParser::with_capacities(self.refs_capacity, self.expr_capacity);
        match parser.parse(body.to_string().as_str()) {
            Ok(bml) => Variant::from(Gd::<BulletMLFile>::with_base(|base| BulletMLFile { base, bml: Rc::new(bml) })),
            Err(err) => {
                godot_error!("Failed to parse BulletML file at {}: {:?}", path, err);
                Variant::from(Error::ERR_INVALID_DATA)
            }
        }
    }
}
