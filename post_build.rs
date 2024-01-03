use std::{env, fs};
use std::path::{Path, PathBuf};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CRATE_MANIFEST_DIR").unwrap();
    let build_type = env::var("CRATE_PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    PathBuf::from(path)
}

fn main() {
    println!("Running post build script");
    // println!("{:?}", env::vars().collect::<Vec<_>>());
    let profile = env::var("CRATE_PROFILE").unwrap();

    let target_dir = get_output_path();
    let src_path = target_dir.join("godot_bulletml.dll");

    // let target_triple = env::var("CRATE_TARGET_TRIPLE").unwrap();
    let toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    let target_os = toolchain.split('-').nth(3).unwrap();
    let target_arch = toolchain.split('-').nth(1).unwrap();

    let dest_path = Path::new("project").join("addons").join("bulletml").join("bin").join(format!("libbulletml.{target_os}.{profile}.{target_arch}.dll"));
    if dest_path.exists() {
        fs::remove_file(&dest_path).unwrap();
    }
    if src_path.exists() {
        fs::copy(&src_path, &dest_path).unwrap();
        println!("Copied {} to {}", src_path.display(), dest_path.display());
    }
}
