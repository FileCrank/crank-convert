

/**
Walk the directory, looking for anything instantiating a file type,
and pull
**/
fn main() {
    println!("cargo:rerun-if-changed=src");

}