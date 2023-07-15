use std::rc::Rc;
use bulletml::parse::BulletMLParser;
use godot::engine::FileAccess;
use godot::engine::global::Error;
use godot::prelude::*;
use crate::resource::BulletML;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct BulletMLPlugin {
    #[base]
    base: Base<Object>,

    parser: BulletMLParser,
}

#[godot_api]
impl BulletMLPlugin {
    #[func]
    fn load_xml(&mut self, path: GodotString) -> Variant {
        let body = FileAccess::get_file_as_string(path.clone());
        let parser = BulletMLParser::with_capacities(1024, 1024);
        match parser.parse(body.to_string().as_str()) {
            Ok(bml) => Variant::from(Gd::<BulletML>::with_base(|base| BulletML::new(base, Rc::new(bml)))),
            Err(err) => {
                godot_error!("Failed to parse BulletML file at {}: {:?}", path, err);
                Variant::from(Error::ERR_INVALID_DATA)
            }
        }
    }
}

#[godot_api]
impl ObjectVirtual for BulletMLPlugin {
    fn init(base: Base<Object>) -> Self {
        Self { base, parser: BulletMLParser::with_capacities(1024, 1024) }
    }
}
