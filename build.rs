use anyhow::*;
// use fs_extra::copy_items;
// use fs_extra::dir::CopyOptions;
// use std::env;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=res/*");

    // let out_dir = env::var("OUT_DIR")?;
    Ok(())
}