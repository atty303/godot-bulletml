use std::{env, fs};
use std::path::{Path, PathBuf};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    PathBuf::from(path)
}

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let target_dir = get_output_path();
    let src_path = target_dir.join("godot_bulletml.dll");
    let dest_path = Path::new("project").join("addons").join("bulletml").join("bin").join(format!("libbulletml.{target_os}.{profile}.{target_arch}.dll"));
    if dest_path.exists() {
        fs::remove_file(&dest_path).unwrap();
    }
    if src_path.exists() {
        fs::copy(&src_path, &dest_path).unwrap();
    }
}
