use std::{env, path::Path, fs::File, io::Write};

#[path = "build/block.rs"] mod block;
#[path = "build/item.rs"] mod item;

fn main() {
    block::write_block_states().unwrap();
    item::write_items().unwrap();

    println!("cargo:rerun-if-changed=./minecraft-data");
    println!("cargo:rerun-if-changed=build.rs");
}

fn file_src(filename: &'static str) -> File {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("src/").join(filename);
    let mut f = File::create(&dest_path).unwrap();
    write_header(&mut f);
    f
}

fn file_out(filename: &'static str) -> File {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(filename);
    let mut f = File::create(&dest_path).unwrap();
    write_header(&mut f);
    f
}

fn write_header(f: &mut File) {
    f.write_all(b"// DO NOT MANUALLY EDIT THIS FILE\n").unwrap();
    f.write_all(b"// This file has been autogenerated by minecraft_constants/build.rs\n").unwrap();
    f.write_all(b"// Data is provided courtesy of `https://github.com/PrismarineJS/minecraft-data`\n\n").unwrap();
}