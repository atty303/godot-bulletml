use std::rc::Rc;

use bulletml::parse::BulletMLParser;
use godot::engine::{FileAccess, ResourceFormatLoader, ResourceFormatLoaderVirtual, ResourceFormatSaver, ResourceFormatSaverVirtual};
use godot::engine::global::Error;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct BulletML {
    #[base]
    base: Base<Resource>,

    pub bml: Rc<bulletml::BulletML>,

    #[export]
    pub bullet_scenes: Array<Gd<PackedScene>>,
}

#[godot_api]
impl BulletML {
    pub fn new(base: Base<Resource>, bml: Rc<bulletml::BulletML>) -> Self {
        Self {
            base,
            bml,
            bullet_scenes: array![],
        }
    }
}

#[godot_api]
impl ResourceVirtual for BulletML {
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
        PackedStringArray::from(&[GodotString::from("bml")])
    }

    fn handles_type(&self, type_: StringName) -> bool {
        type_ == StringName::from("BulletML")
    }

    fn get_resource_type(&self, _path: GodotString) -> GodotString {
        GodotString::from("BulletML")
    }

    fn load(&self, path: GodotString, _original_path: GodotString, _use_sub_threads: bool, _cache_mode: i32) -> Variant {
        let body = FileAccess::get_file_as_string(path.clone());
        let parser = BulletMLParser::with_capacities(self.refs_capacity, self.expr_capacity);
        match parser.parse(body.to_string().as_str()) {
            Ok(bml) => Variant::from(Gd::<BulletML>::with_base(|base| BulletML { base, bullet_scenes: array![], bml: Rc::new(bml) })),
            Err(err) => {
                godot_error!("Failed to parse BulletML file at {}: {:?}", path, err);
                Variant::from(Error::ERR_INVALID_DATA)
            }
        }
    }
}


#[derive(GodotClass)]
#[class(base=ResourceFormatSaver, init)]
pub struct BulletMLResourceFormatSaver {
    #[base]
    base: Base<ResourceFormatSaver>,
}

#[godot_api]
impl BulletMLResourceFormatSaver {
    pub fn new(base: Base<ResourceFormatSaver>) -> Self {
        Self {
            base,
        }
    }
}

#[godot_api]
impl ResourceFormatSaverVirtual for BulletMLResourceFormatSaver {
    fn get_recognized_extensions(&self, resource: Gd<Resource>) -> PackedStringArray {
        if self.recognize(resource) {
            PackedStringArray::from(&[GodotString::from("bml")])
        } else {
            PackedStringArray::new()
        }
    }

    fn recognize(&self, resource: Gd<Resource>) -> bool {
        resource.is_class(GodotString::from("BulletML"))
    }

    fn recognize_path(&self, resource: Gd<Resource>, _path: GodotString) -> bool {
        self.recognize(resource)
    }
}
