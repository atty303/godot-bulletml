use std::cell::RefCell;

use godot::engine::ResourceLoader;
use godot::init::EditorRunBehavior;
use godot::prelude::*;

use crate::resource::BulletMLResourceFormatLoader;

mod canvas;
mod pool;
mod bullet;
mod player;
mod resource;

thread_local! {
    static BULLETML_RESOURCE_FORMAT_LOADER: RefCell<Option<Gd<BulletMLResourceFormatLoader>>> = RefCell::new(None);
}

struct BulletMLExtension {
}

#[gdextension]
unsafe impl ExtensionLibrary for BulletMLExtension {
    fn editor_run_behavior() -> EditorRunBehavior {
        EditorRunBehavior::AllClasses
    }

    fn min_level() -> InitLevel {
        InitLevel::Scene
    }

    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                godot_print!("init");
                // auto_register_classes();
                let loader = Gd::<BulletMLResourceFormatLoader>::from_init_fn(|base| BulletMLResourceFormatLoader::new(base, 16, 1024));

                BULLETML_RESOURCE_FORMAT_LOADER.with(|l| {
                    l.replace(Some(loader.clone()));
                });

                ResourceLoader::singleton().add_resource_format_loader(loader.upcast());
                godot_print!("init done");
            }
            _ => {}
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                godot_print!("deinit");
                BULLETML_RESOURCE_FORMAT_LOADER.with(|l| {
                    if let Some(loader) = l.borrow_mut().take() {
                        ResourceLoader::singleton().remove_resource_format_loader(loader.clone().upcast());
                    }
                    l.replace(None);
                });
                godot_print!("deinit done");
            }
            _ => {}
        }
    }
}
