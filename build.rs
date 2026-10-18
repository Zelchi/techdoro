use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let icon = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("assets/icon.ico");
    let resource = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("techdoro.rc");
    let icon = icon
        .to_str()
        .expect("Icon path must be valid Unicode")
        .replace('\\', "\\\\");
    fs::write(&resource, format!("1 ICON \"{icon}\"\n"))
        .expect("Could not write Windows icon resource");
    embed_resource::compile_for(resource, ["techdoro"], embed_resource::NONE)
        .manifest_required()
        .expect("Could not embed the application icon in techdoro.exe");
}
