use std::env;
use std::error::Error;
use std::path::PathBuf;
use sea_orm_entity_generator::generate_db_entities;

#[path = "src/db/migrations/mod.rs"]
mod migrations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Generate gRPC Server
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("nodeapi_descriptor.bin"))
        .compile(&["proto/nodeapi.proto"], &["proto"])
        .unwrap();

    // Generate Database Entities
    println!("cargo:rerun-if-changed=src/db/migrations/");
    generate_db_entities::<migrations::Migrator>("db").await?;
    Ok(())
}
