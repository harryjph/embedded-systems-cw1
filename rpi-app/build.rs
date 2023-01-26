use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .compile(&["nodeapi.proto"], &["../server/proto"])
        .unwrap();
    Ok(())
}
