use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir("./").unwrap();
    let path = env::var("PROTO").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    tonic_build::compile_protos(path).unwrap();
    Ok(())
}