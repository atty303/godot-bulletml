mod bulletml;
mod resource;

use std::ptr::null;
use godot::engine::ResourceLoader;
use godot::prelude::*;
use godot::sys::{__GdextStringName, GDExtensionConstStringNamePtr, GDExtensionGodotVersion, GDExtensionInterfaceGetGodotVersion, get_interface};
use crate::resource::BulletMLResourceFormatLoader;

struct BulletMLExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BulletMLExtension {
    // fn load_library(handle: &mut InitHandle) -> bool {
    //     handle.register_layer(InitLevel::Editor, BulletMLExtension);
    //     true
    // }
}

// impl ExtensionLayer for BulletMLExtension {
//     fn initialize(&mut self) {
//         godot_print!("init");
//         // let loader = Gd::<BulletMLResourceFormatLoader>::with_base(|base| BulletMLResourceFormatLoader { base });
//         // ResourceLoader::singleton().add_resource_format_loader(loader.upcast());
//         godot_print!("init done");
//     }
//
//     fn deinitialize(&mut self) {
//         godot_print!("deinit");
//     }
// }