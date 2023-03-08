use std::{env, fs, path::Path};

use bibi_types::Channel;

fn main() {
    let channels: Vec<Channel> = serde_json::from_str(include_str!("channels.json")).unwrap();
    // println!("cargo:warning={:?}", channels);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("channels.rs");

    fs::write(dest_path, format!("{:?}", channels)).unwrap();
}
