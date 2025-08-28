use std::{env::var, path::PathBuf};

fn main(){
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    
    let file = PathBuf::from(&manifest_dir)
        .join("src")
        .join("heap.c");

    cc::Build::new()
        .file(file)
        .compile("heap");
}