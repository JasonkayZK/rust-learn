use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let protos = [
        "proto/basic/basic.proto",
        "proto/hello.proto",
        "proto/goodbye.proto",
    ];

    tonic_build::configure()
        .build_server(true)
        .compile(&protos, &["proto/"])?;

    rerun(&protos);

    Ok(())
}

fn rerun(proto_files: &[&str]) {
    for proto_file in proto_files {
        println!("cargo:rerun-if-changed={}", proto_file);
    }
}
