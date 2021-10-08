fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/nova.ratelimit.v1alpha.proto").unwrap();
    Ok(())
}
