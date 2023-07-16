use godot::engine::class_macros::auto_register_classes;
use godot::engine::ResourceLoader;
use godot::prelude::*;

use crate::resource::BulletMLResourceFormatLoader;

mod bulletml;
mod resource;

struct BulletMLExtension {
    loader: Option<Gd<BulletMLResourceFormatLoader>>,
}

#[gdextension]
unsafe impl ExtensionLibrary for BulletMLExtension {
    fn load_library(handle: &mut InitHandle) -> bool {
        handle.register_layer(InitLevel::Scene, BulletMLExtension { loader: None });
        true
    }
}

impl ExtensionLayer for BulletMLExtension {
    fn initialize(&mut self) {
        godot_print!("init");
        auto_register_classes();
        let loader = Gd::<BulletMLResourceFormatLoader>::with_base(|base| BulletMLResourceFormatLoader::new(base, 16, 1024));
        self.loader = Some(loader.share());
        ResourceLoader::singleton().add_resource_format_loader(loader.upcast());
        godot_print!("init done");
    }

    fn deinitialize(&mut self) {
        godot_print!("deinit");
        if let Some(loader) = &self.loader {
            ResourceLoader::singleton().remove_resource_format_loader(loader.share().upcast());
            self.loader = None;
        }
        godot_print!("deinit done");
    }
}
