mod bulletml;

use godot::prelude::*;

struct BulletMLExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BulletMLExtension {}
