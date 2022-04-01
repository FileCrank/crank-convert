use std::collections::HashMap;
use std::env;
use std::path::{Component, Components, Path, PathBuf};
use std::fs;
use proc_macro2::TokenStream;
use walkdir::{WalkDir};

type ModuleComponent = (Vec<String>, TokenStream);

fn resolve_module_component(p: &Path,
                            dir_component_len: usize) -> Option<ModuleComponent> {
    if p.is_file() {
        if let Some(ext) = p.extension() {
            if ext == "rs" {
                println!("Got p {}", p.display());
                let x: TokenStream = fs::read_to_string(p)
                    .unwrap()
                    .parse()
                    .unwrap();
                let mut components: Vec<Component> = p
                    .components()
                    .collect();
                components = components[dir_component_len..]
                    .into();
                let component_strs: Vec<String> = components.
                    iter()
                    .map(|i| i
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .to_string())
                    .collect();
                println!("{:?}", component_strs);
                return Some((component_strs, x))
            }
        }
    }
    None
}

fn parse_module(dir: &Path) -> Vec<ModuleComponent> {
    let dir_components: Vec<Component> = dir
        .components()
        .collect();
    let dir_component_len = dir_components.len();
    println!("Dir is {} ", dir.display());
    let mut res: Vec<ModuleComponent> = Vec::new();
    for entry in WalkDir::new(dir) {
        let ent = entry.unwrap();
        let p = ent.path();
        if let Some(module_component) = resolve_module_component(p, dir_component_len) {
            res.push(module_component);
        }
    }
    res
}

/**
Walk the directory, looking for anything instantiating a file type,
and pull
**/
fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("Runnig!");
    let out_dir_raw = env::var("CARGO_MANIFEST_DIR")
        .unwrap();
    let out_dir = Path::new(out_dir_raw.as_str());
    let src_dir_buf: PathBuf = out_dir
        .join("src");
    let src_dir: &Path = src_dir_buf.as_path();
    assert!(src_dir.exists());
    // Find all rust files in the src directory and parse them into a token stream
    let module_components = parse_module(src_dir);

}