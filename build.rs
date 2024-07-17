fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compile protocol buffer using protoc
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/action.proto"], &["proto"])
        .expect("Should build dependencies");
    Ok(())
}