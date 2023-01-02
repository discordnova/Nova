fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<String> = glob::glob("../../proto/nova/**/*.proto")?
        .map(|f| f.unwrap().to_str().unwrap().to_string())
        .collect();

    tonic_build::configure()
        .include_file("genproto.rs")
        .compile(&paths, &["../../proto"])?;

    Ok(())
}
