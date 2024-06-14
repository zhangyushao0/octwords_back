fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/login.proto")?;
    tonic_build::compile_protos("proto/word.proto")?;
    Ok(())
}
